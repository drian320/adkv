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
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 日本語フォントを設定（eguiデフォルトフォントを使用）
        let mut fonts = egui::FontDefinitions::default();

        // egui組み込みの日本語対応フォント（epaint_default_fonts）を使用
        // これにより外部フォントファイル不要で日本語が表示可能

        cc.egui_ctx.set_fonts(fonts);

        Self::default()
    }

    fn try_connect(&mut self) {
        match SharedMemoryClient::connect() {
            Ok(client) => {
                self.shm_client = Some(client);
                self.connection_status = "Connected".to_string();
            }
            Err(e) => {
                self.connection_status = format!("接続失敗: {}", e);
            }
        }
    }

    fn update_data(&mut self) {
        if let Some(ref client) = self.shm_client {
            match client.read_data() {
                Ok(data) => {
                    if data.is_valid() {
                        self.connection_status = format!("接続中 (g_Base: 0x{:X})", data.g_base);
                        self.shared_data = data;
                    } else {
                        self.connection_status = "データ無効 (apex_dma未起動?)".to_string();
                    }
                }
                Err(e) => {
                    self.connection_status = format!("読み取りエラー: {}", e);
                }
            }
        }
    }

    fn write_settings(&mut self) {
        if let Some(ref client) = self.shm_client {
            if let Err(e) = client.write_settings(&self.settings) {
                eprintln!("設定書き込みエラー: {}", e);
            }
        }
    }
}

impl eframe::App for ApexClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 定期的にデータ更新 (60 FPS)
        if self.last_update.elapsed().as_millis() > 16 {
            self.update_data();
            self.last_update = std::time::Instant::now();
        }

        // 継続的な再描画をリクエスト
        ctx.request_repaint();

        // トップパネル
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🎮 Apex Legends DMA Client");
                ui.separator();
                ui.label(&self.connection_status);

                if self.shm_client.is_none() {
                    if ui.button("接続").clicked() {
                        self.try_connect();
                    }
                }
            });
        });

        // サイドパネル - 設定
        egui::SidePanel::left("settings_panel").min_width(250.0).show(ctx, |ui| {
            ui.heading("⚙ 設定");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.group(|ui| {
                    ui.label("機能");
                    if ui.checkbox(&mut self.settings.aim_enabled, "AIM有効").changed() {
                        self.write_settings();
                    }
                    if ui.checkbox(&mut self.settings.esp_enabled, "ESP有効").changed() {
                        self.write_settings();
                    }
                    if ui.checkbox(&mut self.settings.player_glow_enabled, "プレイヤーグロー").changed() {
                        self.write_settings();
                    }
                    if ui.checkbox(&mut self.settings.aim_no_recoil, "リコイル制御").changed() {
                        self.write_settings();
                    }
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("AIM設定");
                    if ui.add(egui::Slider::new(&mut self.settings.max_fov, 1.0..=20.0).text("FOV")).changed() {
                        self.write_settings();
                    }
                    if ui.add(egui::Slider::new(&mut self.settings.smooth, 1.0..=50.0).text("スムース")).changed() {
                        self.write_settings();
                    }
                    if ui.add(egui::Slider::new(&mut self.settings.max_dist, 1000.0..=20000.0).text("最大距離")).changed() {
                        self.write_settings();
                    }

                    ui.horizontal(|ui| {
                        ui.label("ターゲットボーン:");
                        if ui.selectable_value(&mut self.settings.bone, 0, "頭").changed() {
                            self.write_settings();
                        }
                        if ui.selectable_value(&mut self.settings.bone, 1, "首").changed() {
                            self.write_settings();
                        }
                        if ui.selectable_value(&mut self.settings.bone, 2, "胸").changed() {
                            self.write_settings();
                        }
                    });
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("グローカラー設定");

                    ui.label("非表示時:");
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
                    ui.label("表示時:");
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
                    ui.label("その他");
                    if ui.checkbox(&mut self.settings.firing_range, "射撃場モード").changed() {
                        self.write_settings();
                    }
                    if ui.checkbox(&mut self.settings.onevone, "1v1モード").changed() {
                        self.write_settings();
                    }
                });
            });
        });

        // 中央パネル - 統計情報
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📊 統計情報");
            ui.separator();

            egui::Grid::new("stats_grid").striped(true).show(ui, |ui| {
                ui.label("ゲームベースアドレス:");
                ui.label(format!("0x{:X}", self.shared_data.g_base));
                ui.end_row();

                ui.label("スペクテイター数:");
                ui.label(format!("{}", self.shared_data.spectators));
                ui.end_row();

                ui.label("味方スペクテイター数:");
                ui.label(format!("{}", self.shared_data.allied_spectators));
                ui.end_row();

                ui.label("検出プレイヤー数:");
                ui.label(format!("{}", self.shared_data.player_count));
                ui.end_row();
            });

            ui.add_space(20.0);

            // プレイヤーリスト
            if self.shared_data.player_count > 0 {
                ui.heading("👥 検出されたプレイヤー");
                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    egui::Grid::new("player_grid")
                        .striped(true)
                        .num_columns(6)
                        .show(ui, |ui| {
                            ui.label("距離");
                            ui.label("HP");
                            ui.label("シールド");
                            ui.label("チーム");
                            ui.label("可視");
                            ui.label("ダウン");
                            ui.end_row();

                            for player in &self.shared_data.players[..self.shared_data.player_count] {
                                ui.label(format!("{:.0}m", player.distance / 40.0));
                                ui.label(format!("{}", player.health));
                                ui.label(format!("{}", player.shield));
                                ui.label(format!("{}", player.team_num));
                                ui.label(if player.is_visible { "✓" } else { "✗" });
                                ui.label(if player.is_knocked { "✓" } else { "✗" });
                                ui.end_row();
                            }
                        });
                });
            }

            ui.add_space(20.0);

            // スペクテイターリスト
            if self.shared_data.spectator_count > 0 {
                ui.heading("👁 スペクテイター");
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for spec in &self.shared_data.spectators_list[..self.shared_data.spectator_count] {
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
