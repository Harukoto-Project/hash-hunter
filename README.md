# Hash Hunter

Gitリポジトリのコミットハッシュから、ゼロ連続や回文などのレアな並びを探すCLIです。

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-edition%202021-dea584.svg)](https://www.rust-lang.org/)

[概要](#概要) · [機能](#機能) · [使い方](#使い方) · [セットアップ](#セットアップ) · [コントリビュート](#コントリビュート) · [ライセンス](#ライセンス)

[日本語](README.md) · [English](README.en.md) · [한국어](README.ko.md)

---

## 概要

Hash Hunterは、GitリポジトリのURLまたはローカルパスを渡し、既存コミットのハッシュを評価してレアな並びをランキング表示します。

対象はコミットハッシュのみです。ナンスを付けてコミットを作り直すマイニングは未実装です。

## 機能

### 履歴スキャン

URLの場合は一時ディレクトリへbare cloneし、ローカルパスの場合はその場のリポジトリを参照します。`git rev-list`でコミットを列挙し、各ハッシュを検出器に通します。一時ディレクトリは終了時に削除します。

SHA-1(40文字)とSHA-256(64文字)のどちらも、長さを固定せず扱います。

### レア判定

各検出器はヒットの有無とレア度スコア(bits)を返します。スコアは `-log2(偶然その条件を満たす確率)` です。1つのハッシュに複数ヒットがあれば、位置の重なりを除かず単純合計します。

| ID | 内容 |
| --- | --- |
| `leading-zeros` | 先頭の`0`の連続(4文字以上) |
| `trailing-zeros` | 末尾の`0`の連続(4文字以上) |
| `repeat-run` | 同じ文字の連続(4文字以上。`7777`など) |
| `ascending` | 16進の昇順連続(4文字以上。`12345` / `abcde`) |
| `descending` | 16進の降順連続(4文字以上。`54321` / `fedcb`) |
| `digits-only` | ハッシュ全体が数字(`0-9`) |
| `letters-only` | ハッシュ全体が英字(`a-f`) |
| `palindrome` | 長さ6以上の回文 |
| `repeat-pattern` | 同じパターンの繰り返し(`abababab`) |
| `hex-word` | `dead` `beef` `cafe` `babe` `face` `c0de` `f00d` |
| `lucky` | `777` `1234567` |

新しい検出器は`src/detect/detectors/`にファイルを追加し、レジストリへ登録します。

### ランキング表示

スコア降順、同点はハッシュ昇順で、上位N件を出します。TTYではヒット文字を黄太字にし、非TTYでは色を付けません。該当がなければ何も出しません。

```
1.   16.00  0000ab
     leading-zeros    0..4    16.00
     repeat-run       0..4    12.00
```

## 使い方

```
hash-hunter <REPO> [OPTIONS]
```

| 引数・オプション | 内容 | 既定値 |
| --- | --- | --- |
| `<REPO>` | GitのURLまたはローカルパス | 必須(`--list-detectors`時は不要) |
| `-n, --top <N>` | 表示する上位件数 | `20` |
| `--min-score <BITS>` | このスコア未満は表示しない | `0` |
| `--branch <NAME>` | 対象ブランチ | 全ブランチ |
| `--only <ID,...>` | 有効にする検出器IDを限定 | 全て |
| `--exclude <ID,...>` | 無効にする検出器ID | なし |
| `--list-detectors` | 検出器の一覧を表示して終了 | - |

```bash
hash-hunter https://github.com/example/repo.git
hash-hunter ./my-repo --top 5 --min-score 8
hash-hunter ./my-repo --only leading-zeros,lucky
hash-hunter --list-detectors
```

エラーは標準エラーへ`error[<code>]: <message>`の形で出し、終了コードは1です。

| コード | 状況 |
| --- | --- |
| `git_not_found` | `git`コマンドが見つからない |
| `clone_failed` | clone失敗、ローカルパスがリポジトリでない、ブランチが存在しない、など |
| `empty_repository` | コミットが0件 |
| `unknown_detector` | `--only` / `--exclude`に存在しないID |

privateリポジトリの認証は、SSH鍵やcredential helperなどgit側の設定に任せます。このアプリは認証情報を持ちません。

## セットアップ

動作にはRust(edition 2021)と、PATH上の`git`コマンドが必要です。確認済みの環境はWindowsです。macOS/Linuxは未確認です。

```bash
git clone https://github.com/Harukoto-Project/hash-hunter.git
cd hash-hunter
cargo run -- --help
```

インストールする場合:

```bash
cargo install --path .
hash-hunter --help
```

## コントリビュート

開発の進め方は[CONTRIBUTING.md](CONTRIBUTING.md)を参照してください。IssueとPull Requestを歓迎します。

## ライセンス

[Apache License 2.0](LICENSE)です。
