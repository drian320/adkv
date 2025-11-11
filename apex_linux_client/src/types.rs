use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

/// 共有メモリのサイズ (100プレイヤー + 100スペクテイター + 設定)
pub const SHARED_MEMORY_SIZE: usize = 1024 * 1024; // 1MB
pub const SHARED_MEMORY_NAME: &str = "/apex_dma_shared";

/// プレイヤー情報 (ESPデータ)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Player {
    pub head_x: f32,
    pub head_y: f32,
    pub origin_x: f32,
    pub origin_y: f32,
    pub health: i32,
    pub shield: i32,
    pub team_num: i32,
    pub distance: f32,
    pub is_visible: u8,
    pub is_knocked: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            head_x: 0.0,
            head_y: 0.0,
            origin_x: 0.0,
            origin_y: 0.0,
            health: 0,
            shield: 0,
            team_num: 0,
            distance: 0.0,
            is_visible: 0,
            is_knocked: 0,
        }
    }
}

/// スペクテイター情報
#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Spectator {
    #[serde(with = "BigArray")]
    pub name: [u8; 64],
}

impl Default for Spectator {
    fn default() -> Self {
        Self { name: [0; 64] }
    }
}

/// 設定データ (GUIからapex_dmaへ)
#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Settings {
    pub aim_enabled: u8,
    pub esp_enabled: u8,
    pub player_glow_enabled: u8,
    pub aim_no_recoil: u8,
    pub aiming: u8,
    pub shooting: u8,
    pub firing_range: u8,
    pub onevone: u8,

    pub max_dist: f32,
    pub smooth: f32,
    pub max_fov: f32,
    pub bone: i32,

    // グローカラー (非表示時)
    pub glow_r: f32,
    pub glow_g: f32,
    pub glow_b: f32,

    // グローカラー (表示時)
    pub glow_r_visible: f32,
    pub glow_g_visible: f32,
    pub glow_b_visible: f32,

    // グローカラー (ノックダウン時)
    pub glow_r_knocked: f32,
    pub glow_g_knocked: f32,
    pub glow_b_knocked: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            aim_enabled: 0,
            esp_enabled: 0,
            player_glow_enabled: 0,
            aim_no_recoil: 1,
            aiming: 0,
            shooting: 0,
            firing_range: 0,
            onevone: 0,

            max_dist: 200.0 * 40.0,
            smooth: 10.0,
            max_fov: 5.0,
            bone: 2,

            glow_r: 1.0,
            glow_g: 0.0,
            glow_b: 0.0,

            glow_r_visible: 0.0,
            glow_g_visible: 1.0,
            glow_b_visible: 0.0,

            glow_r_knocked: 0.0,
            glow_g_knocked: 0.0,
            glow_b_knocked: 1.0,
        }
    }
}

/// apex_dmaからGUIへのデータ
#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedData {
    pub magic: u32,              // 0xABCD (データ有効性チェック)
    pub g_base: u64,             // ゲームベースアドレス
    pub spectators: i32,         // スペクテイター数
    pub allied_spectators: i32,  // 同チームスペクテイター数
    pub player_count: usize,     // 有効なプレイヤー数
    pub spectator_count: usize,  // 有効なスペクテイター数

    pub settings: Settings,      // 設定 (GUI → apex_dma)
    #[serde(with = "BigArray")]
    pub players: [Player; 100],  // プレイヤーリスト
    #[serde(with = "BigArray")]
    pub spectators_list: [Spectator; 100], // スペクテイターリスト
}

impl Default for SharedData {
    fn default() -> Self {
        Self {
            magic: 0xABCD,
            g_base: 0,
            spectators: 0,
            allied_spectators: 0,
            player_count: 0,
            spectator_count: 0,
            settings: Settings::default(),
            players: [Player::default(); 100],
            spectators_list: [Spectator::default(); 100],
        }
    }
}

impl SharedData {
    pub fn is_valid(&self) -> bool {
        self.magic == 0xABCD && self.g_base != 0
    }
}
