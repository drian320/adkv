mod app;
mod shm;
mod types;

use app::ApexClientApp;
use types::{Settings, Player, Spectator, SharedData};

fn main() -> eframe::Result<()> {
    env_logger::init(); // ログ初期化

    // 構造体のサイズをデバッグ出力
    println!("[DEBUG Rust] sizeof Settings: {} bytes", std::mem::size_of::<Settings>());
    println!("[DEBUG Rust] sizeof Player: {} bytes", std::mem::size_of::<Player>());
    println!("[DEBUG Rust] sizeof Spectator: {} bytes", std::mem::size_of::<Spectator>());
    println!("[DEBUG Rust] sizeof SharedData: {} bytes", std::mem::size_of::<SharedData>());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Apex Legends DMA Client",
        options,
        Box::new(|cc| Ok(Box::new(ApexClientApp::new(cc)))),
    )
}
