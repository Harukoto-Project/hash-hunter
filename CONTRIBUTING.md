# コントリビュートについて

IssueとPull Requestを歓迎します。

## 開発環境

- OS: TODO: 要確認
- Rust (edition 2021)

## 提出前の確認

- `cargo fmt` — コードを整形する
- `cargo check` — コンパイルできることを確認する
- `cargo clippy` — Lintを実行する
- `cargo test` — テストを実行する

## バグ報告

次の情報があると調査しやすくなります。

- 対象がURLかローカルパスか
- 指定したオプション(`--branch` / `--only` / `--exclude` / `--top` / `--min-score`)
- 表示されたエラー(`error[<code>]: ...`のコードとメッセージ)
- `git --version`の出力
- コミットハッシュの桁数(40文字のSHA-1か、64文字のSHA-256か)

## セキュリティ上の問題

セキュリティに関わる問題は公開Issueではなく、GitHubのSecurity Advisory(Report a vulnerability)から非公開で報告してください。

## ライセンス

コントリビュートされた内容は、本プロジェクトと同じApache License 2.0で提供されるものとします。
