# RSS Reporter

## このプロジェクトのコンセプト / 学習面での目的

- 情報収集環境を自分に合う形にデザインしてみること．

- Rustを使った実装及び設計を通じてバックエンドを中心に学習し，開発経験を積むこと．

## 機能

- `config.toml` に登録した RSS フィードを読み込む
- `enabled = true` のフィードだけを対象にする
- RSS / Atom の XML を取得して記事情報に変換する
- 各購読元について以下の情報を表示する
  - 購読元名
  - サイト URL
  - フィード URL
  - 記事タイトル
  - 記事 URL
  - 配信元ドメイン
  - トピックタグ
- 取得に失敗したフィードについて、成功したものとは分けてエラーを表示する
- `scour.ing` のリダイレクト URL を元記事 URL に正規化する

## 依存関係

- Rust
- Cargo

主な利用クレート:

- `feed-rs`
  - RSS / Atom フィードのパース
- `reqwest`
  - RSS XML の取得
- `config`
  - `config.toml` の読み込み
- `serde`
  - 設定ファイルのデシリアライズ
- `percent-encoding`
  - URL デコード
- `ratatui`, `crossterm`
  - TUI 実装用  
  - 現時点では TUI は未実装

## 実行手順

1. 設定ファイルを作成する

```sh
cp config.example.toml config.toml
```

2. `config.toml` に購読したい RSS フィードを追加する

```toml
[[source]]
name = "example"
feed_url = "https://example.com/feed.xml"
enabled = true
```

3. CLI を実行する

```sh
cargo run -p rss-reporter-cli
```

4. テストを実行する

```sh
cargo test
```

## WIP

- TUI 版の実装
- エラー表示の改善
  - URL が不正な場合
  - XML として解釈できない場合
  - サーバーの応答が遅い場合
  - HTTP ステータスエラーの場合
- RSS 取得・パース処理のテスト追加
- 記事の既読管理
- カテゴリ別・トピック別の整理機能
- 通知機能
- 購読元や記事の表示形式の改善

## ライセンス

現時点ではライセンス未設定です。
