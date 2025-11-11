# Apex Legends Linux GUI Client

Rust + eguiで作成されたApex Legends DMA用のLinux GUIクライアント。

## 特徴

- 🎨 **モダンなGUI**: eguiを使用した軽量で高速なUI
- 🚀 **共有メモリ通信**: `apex_dma`とのリアルタイム通信
- ⚙️ **リアルタイム設定**: UIから直接AIM、ESP、グロー設定を変更可能
- 📊 **統計表示**: プレイヤー情報、スペクテイター情報をリアルタイム表示

## 必要条件

- Rust (edition 2021+)
- Linux (共有メモリ対応)
- `apex_dma` が実行中であること

## ビルド方法

```bash
cd apex_linux_client
cargo build --release
```

## 実行方法

1. **apex_dmaを起動**:
```bash
cd apex_dma
./run_apex_dma.sh
```

2. **GUIクライアントを起動**:
```bash
cd apex_linux_client
cargo run --release
```

3. GUIウィンドウで「接続」ボタンをクリック

## 機能

### 設定パネル

- **AIM設定**:
  - AIM有効/無効
  - FOV調整 (1.0 - 20.0)
  - スムース調整 (1.0 - 50.0)
  - 最大距離 (1000 - 20000)
  - ターゲットボーン選択 (頭/首/胸)

- **ESP設定**:
  - ESP有効/無効
  - プレイヤーグロー有効/無効
  - リコイル制御

- **グローカラー**:
  - 非表示時の色 (RGB)
  - 表示時の色 (RGB)
  - ノックダウン時の色 (RGB)

### 統計情報

- ゲームベースアドレス
- スペクテイター数
- 検出プレイヤー一覧
- スペクテイターリスト

## アーキテクチャ

```
apex_dma (C++)  <---> 共有メモリ <---> apex_linux_client (Rust)
      ↓                                       ↓
  Apex Legends                            egui GUI
```

### データフロー

1. **apex_dma → GUI**: ゲームデータ (プレイヤー、スペクテイター情報)
2. **GUI → apex_dma**: 設定データ (AIM、ESP設定)

### 共有メモリ

- **名前**: `/apex_dma_shared`
- **サイズ**: 1MB
- **形式**: bincode (Rustシリアライゼーション)

## トラブルシューティング

### 「接続失敗」と表示される

- `apex_dma`が起動しているか確認
- `apex_dma`がroot権限で実行されているか確認

### データが更新されない

- ゲーム (Apex Legends) が起動しているか確認
- `apex_dma`のコンソール出力で "Apex process found" と表示されているか確認

### 権限エラー

共有メモリへのアクセス権限がない場合:
```bash
sudo chmod 666 /dev/shm/apex_dma_shared
```

## 開発

### ファイル構成

```
src/
├── main.rs       # エントリーポイント
├── app.rs        # eguiアプリケーション
├── shm.rs        # 共有メモリクライアント
└── types.rs      # データ型定義
```

### 依存クレート

- `eframe`: eguiのウィンドウフレームワーク
- `egui`: Immediate Mode GUI
- `shared_memory`: POSIX共有メモリ
- `serde`: シリアライゼーション
- `bincode`: バイナリエンコーディング

## ライセンス

教育・研究目的のみ。
