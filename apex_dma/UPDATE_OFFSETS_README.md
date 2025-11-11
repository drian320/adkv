# オフセット自動更新スクリプト

## 概要

`update_offsets.py`は、`offsets.ini`から`offsets.h`を自動生成するPythonスクリプトです。

## 使い方

### 基本的な使い方

```bash
cd apex_dma
python3 update_offsets.py
```

これにより、`offsets.ini`の内容を解析し、`offsets.h`ファイルを自動生成します。

### 実行結果の例

```
Reading offsets.ini...
Writing to offsets.h...
✓ offsets.h が正常に更新されました
  更新日時: 2025/11/12
```

## スクリプトの機能

### 自動マッピング

スクリプトは以下のINIセクションからオフセットを抽出します：

- `[Miscellaneous]` - ゲームの主要なアドレス（EntityList、LocalPlayer、NameList等）
- `[Buttons]` - 入力ボタンのアドレス（in_attack、in_duck、in_jump等）
- `[RecvTable.DT_*]` - ネットワーク変数テーブルのオフセット
- `[DataMap.DT_*]` - データマップのオフセット
- `[ConVars]` - コンソール変数のアドレス

### 自動更新機能

- 各`#define`に自動的に更新日付を追加
- 元のソースセクション（INIファイルのどこから取得したか）をコメントとして記録
- オフセット値の形式を自動調整（0xプレフィックスの追加）

### 対応しているオフセット

スクリプトは以下のような主要なオフセットを生成します：

#### ゲーム基本情報
- `OFFSET_ENTITYLIST` - エンティティリストのアドレス
- `OFFSET_LOCAL_ENT` - ローカルプレイヤーのアドレス
- `OFFSET_NAME_LIST` - 名前リストのアドレス

#### プレイヤー情報
- `OFFSET_HEALTH` - 体力
- `OFFSET_SHIELD` - シールド
- `OFFSET_TEAM` - チーム番号
- `OFFSET_LIFE_STATE` - 生死状態

#### ボタン入力
- `OFFSET_IN_ATTACK` - 攻撃ボタン
- `OFFSET_IN_JUMP` - ジャンプボタン
- `OFFSET_IN_DUCK` - しゃがみボタン

#### 描画関連
- `OFFSET_MATRIX` - ビューマトリックス
- `OFFSET_BONES` - ボーン配列
- `OFFSET_GLOW_ENABLE` - グロー有効化フラグ

#### 武器情報
- `OFFSET_WEAPON` - 武器アドレス
- `OFFSET_BULLET_SPEED` - 弾速
- `OFFSET_BULLET_SCALE` - 弾のスケール

## ファイル構造

```
apex_dma/
├── offsets.ini          # 入力: INI形式のオフセットデータベース
├── offsets.h            # 出力: C++ヘッダーファイル
└── update_offsets.py    # このスクリプト
```

## 開発者向け情報

### スクリプトのカスタマイズ

新しいオフセットを追加したい場合は、`generate_header()`メソッド内に以下のようなコードを追加します：

```python
# 例: 新しいオフセットの追加
new_value = self.get_value("セクション名", "キー名")
if new_value:
    lines.append(f"#define OFFSET_NEW_VALUE {self.format_offset(new_value)} //[セクション名].キー名 updated {self.today}")
```

### INIファイルの重複キー対応

INIファイル内に重複するキーが存在する場合がありますが、スクリプトは`strict=False`設定により、これらを適切に処理します。

### エラーハンドリング

- セクションやキーが存在しない場合は、そのオフセットをスキップします
- INIファイルの読み込みエラーは例外として報告されます

## ゲームアップデート時の手順

1. 新しい`offsets.ini`ファイルを入手
2. `apex_dma/offsets.ini`を置き換え
3. `python3 update_offsets.py`を実行
4. 生成された`offsets.h`を確認
5. `make clean && make`でプロジェクトを再ビルド

## 注意事項

- このスクリプトは既存の`offsets.h`を完全に上書きします
- カスタム定義を追加している場合は、スクリプトを編集してそれらを含めるようにしてください
- 生成されたファイルは必ずレビューし、正しい値が設定されていることを確認してください

## トラブルシューティング

### エラー: "No section: 'SectionName'"

特定のセクションが`offsets.ini`に存在しません。INIファイルを確認してください。

### エラー: "DuplicateOptionError"

このエラーは修正されており、`strict=False`設定により回避されます。

### 値が正しく生成されない

`offsets.ini`のフォーマットを確認してください。各値は以下の形式である必要があります：

```ini
[SectionName]
key=0x12345678
```

## ライセンスと用途

このスクリプトは教育・研究目的で提供されています。Apex Legends DMAプロジェクトの一部として、メモリオフセットの管理を簡素化するために作成されました。
