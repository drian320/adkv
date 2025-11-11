use eframe::egui;
use crate::shm::SharedMemoryClient;
use crate::types::{SharedData, Settings};

pub struct ApexClientApp {
    shm_client: Option<SharedMemoryClient>,
    shared_data: SharedData,
    settings: Settings,
    connection_status: String,
    last_update: std::time::Instant,
}

impl Default for ApexClientApp {
    fn default() -> Self {
        Self {
            shm_client: None,
            shared_data: SharedData::default(),
            settings: Settings::default(),
            connection_status: "Not Connected".to_string(),
            last_update: std::time::Instant::now(),
        }
    }
}

impl ApexClientApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn try_connect(&mut self) {
        match SharedMemoryClient::connect() {
            Ok(client) => {
                self.shm_client = Some(client);
                self.connection_status = "Connected".to_string();
            }
            Err(e) => {
                self.connection_status = format!("Connection failed: {}", e);
            }
        }
    }

    fn update_data(&mut self) {
        if let Some(ref client) = self.shm_client {
            match client.read_data() {
                Ok(data) => {
                    // packed構造体のフィールドアクセスには注意が必要
                    let g_base = data.g_base;
                    let magic = data.magic;

                    if magic == 0xABCD && g_base != 0 {
                        self.connection_status = format!("Connected (g_Base: 0x{:X})", g_base);
                        self.shared_data = data;
                    } else {
                        self.connection_status = "Invalid data (apex_dma not running?)".to_string();
                    }
                }
                Err(e) => {
                    self.connection_status = format!("Read error: {}", e);
                }
            }
        }
    }

    fn write_settings(&mut self) {
        if let Some(ref client) = self.shm_client {
            if let Err(e) = client.write_settings(&self.settings) {
                eprintln!("Settings write error: {}", e);
            }
        }
    }
}

impl eframe::App for ApexClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update data periodically (60 FPS)
        if self.last_update.elapsed().as_millis() > 16 {
            self.update_data();
            self.last_update = std::time::Instant::now();
        }

        // Request continuous repaint
        ctx.request_repaint();

        // Top panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Apex Legends DMA Client");
                ui.separator();
                ui.label(&self.connection_status);

                if self.shm_client.is_none() {
                    if ui.button("Connect").clicked() {
                        self.try_connect();
                    }
                }
            });
        });

        // Side panel - Settings
        egui::SidePanel::left("settings_panel").min_width(250.0).show(ctx, |ui| {
            ui.heading("Settings");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.group(|ui| {
                    ui.label("Features");
                    let mut aim_enabled = self.settings.aim_enabled != 0;
                    if ui.checkbox(&mut aim_enabled, "AIM Enabled").changed() {
                        self.settings.aim_enabled = if aim_enabled { 1 } else { 0 };
                        self.write_settings();
                    }
                    let mut esp_enabled = self.settings.esp_enabled != 0;
                    if ui.checkbox(&mut esp_enabled, "ESP Enabled").changed() {
                        self.settings.esp_enabled = if esp_enabled { 1 } else { 0 };
                        self.write_settings();
                    }
                    let mut player_glow = self.settings.player_glow_enabled != 0;
                    if ui.checkbox(&mut player_glow, "Player Glow").changed() {
                        self.settings.player_glow_enabled = if player_glow { 1 } else { 0 };
                        self.write_settings();
                    }
                    let mut no_recoil = self.settings.aim_no_recoil != 0;
                    if ui.checkbox(&mut no_recoil, "No Recoil").changed() {
                        self.settings.aim_no_recoil = if no_recoil { 1 } else { 0 };
                        self.write_settings();
                    }
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("AIM Settings");
                    if ui.add(egui::Slider::new(&mut self.settings.max_fov, 1.0..=20.0).text("FOV")).changed() {
                        self.write_settings();
                    }
                    if ui.add(egui::Slider::new(&mut self.settings.smooth, 1.0..=50.0).text("Smooth")).changed() {
                        self.write_settings();
                    }
                    if ui.add(egui::Slider::new(&mut self.settings.max_dist, 1000.0..=20000.0).text("Max Distance")).changed() {
                        self.write_settings();
                    }

                    ui.horizontal(|ui| {
                        ui.label("Target Bone:");
                        if ui.selectable_value(&mut self.settings.bone, 0, "Head").changed() {
                            self.write_settings();
                        }
                        if ui.selectable_value(&mut self.settings.bone, 1, "Neck").changed() {
                            self.write_settings();
                        }
                        if ui.selectable_value(&mut self.settings.bone, 2, "Chest").changed() {
                            self.write_settings();
                        }
                    });
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Glow Colors");

                    ui.label("Not Visible:");
                    ui.horizontal(|ui| {
                        ui.label("R:");
                        if ui.add(egui::Slider::new(&mut self.settings.glow_r, 0.0..=1.0)).changed() {
                            self.write_settings();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("G:");
                        if ui.add(egui::Slider::new(&mut self.settings.glow_g, 0.0..=1.0)).changed() {
                            self.write_settings();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("B:");
                        if ui.add(egui::Slider::new(&mut self.settings.glow_b, 0.0..=1.0)).changed() {
                            self.write_settings();
                        }
                    });

                    ui.separator();
                    ui.label("Visible:");
                    ui.horizontal(|ui| {
                        ui.label("R:");
                        if ui.add(egui::Slider::new(&mut self.settings.glow_r_visible, 0.0..=1.0)).changed() {
                            self.write_settings();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("G:");
                        if ui.add(egui::Slider::new(&mut self.settings.glow_g_visible, 0.0..=1.0)).changed() {
                            self.write_settings();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("B:");
                        if ui.add(egui::Slider::new(&mut self.settings.glow_b_visible, 0.0..=1.0)).changed() {
                            self.write_settings();
                        }
                    });
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Other");
                    let mut firing_range = self.settings.firing_range != 0;
                    if ui.checkbox(&mut firing_range, "Firing Range Mode").changed() {
                        self.settings.firing_range = if firing_range { 1 } else { 0 };
                        self.write_settings();
                    }
                    let mut onevone = self.settings.onevone != 0;
                    if ui.checkbox(&mut onevone, "1v1 Mode").changed() {
                        self.settings.onevone = if onevone { 1 } else { 0 };
                        self.write_settings();
                    }
                });
            });
        });

        // Central panel - Statistics
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Statistics");
            ui.separator();

            egui::Grid::new("stats_grid").striped(true).show(ui, |ui| {
                // packed構造体からのフィールドコピー
                let g_base = self.shared_data.g_base;
                let spectators = self.shared_data.spectators;
                let allied_spectators = self.shared_data.allied_spectators;
                let player_count = self.shared_data.player_count;

                ui.label("Game Base Address:");
                ui.label(format!("0x{:X}", g_base));
                ui.end_row();

                ui.label("Spectators:");
                ui.label(format!("{}", spectators));
                ui.end_row();

                ui.label("Allied Spectators:");
                ui.label(format!("{}", allied_spectators));
                ui.end_row();

                ui.label("Detected Players:");
                ui.label(format!("{}", player_count));
                ui.end_row();
            });

            ui.add_space(20.0);

            // Player list
            let player_count = self.shared_data.player_count;
            if player_count > 0 {
                ui.heading("Detected Players");
                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    egui::Grid::new("player_grid")
                        .striped(true)
                        .num_columns(6)
                        .show(ui, |ui| {
                            ui.label("Distance");
                            ui.label("HP");
                            ui.label("Shield");
                            ui.label("Team");
                            ui.label("Visible");
                            ui.label("Knocked");
                            ui.end_row();

                            for i in 0..player_count {
                                // packed構造体から値をコピー
                                let distance = self.shared_data.players[i].distance;
                                let health = self.shared_data.players[i].health;
                                let shield = self.shared_data.players[i].shield;
                                let team_num = self.shared_data.players[i].team_num;
                                let is_visible = self.shared_data.players[i].is_visible;
                                let is_knocked = self.shared_data.players[i].is_knocked;

                                ui.label(format!("{:.0}m", distance / 40.0));
                                ui.label(format!("{}", health));
                                ui.label(format!("{}", shield));
                                ui.label(format!("{}", team_num));
                                ui.label(if is_visible != 0 { "Yes" } else { "No" });
                                ui.label(if is_knocked != 0 { "Yes" } else { "No" });
                                ui.end_row();
                            }
                        });
                });
            }

            ui.add_space(20.0);

            // Spectator list
            let spectator_count = self.shared_data.spectator_count;
            if spectator_count > 0 {
                ui.heading("Spectators");
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for i in 0..spectator_count {
                        let spec = self.shared_data.spectators_list[i];
                        let name = String::from_utf8_lossy(&spec.name)
                            .trim_end_matches('\0')
                            .to_string();
                        if !name.is_empty() {
                            ui.label(format!("• {}", name));
                        }
                    }
                });
            }
        });
    }
}
