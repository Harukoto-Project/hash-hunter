# Hash Hunter

A CLI that scans commit hashes in a Git repository and ranks rare patterns such as leading zeros and palindromes.

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-edition%202021-dea584.svg)](https://www.rust-lang.org/)

[Overview](#overview) · [Features](#features) · [Usage](#usage) · [Setup](#setup) · [Contributing](#contributing) · [License](#license)

[日本語](README.md) · [English](README.en.md) · [한국어](README.ko.md)

---

## Overview

Hash Hunter takes a Git URL or a local path, evaluates existing commit hashes, and prints a ranking of rare patterns.

It only looks at commit hashes. Mining that rewrites commits with a nonce is not implemented.

## Features

### History scan

A URL is bare-cloned into a temporary directory. A local path is used in place. Commits are listed with `git rev-list`, and each hash is passed through the detectors. Temporary directories are removed on exit.

Both SHA-1 (40 characters) and SHA-256 (64 characters) are accepted. Hash length is not fixed.

### Rarity detection

Each detector returns hits and a rarity score in bits. The score is `-log2(probability of that pattern occurring by chance)`. If one hash matches several detectors, the scores are added without removing overlapping ranges.

| ID | What it finds |
| --- | --- |
| `leading-zeros` | A run of `0` at the start |
| `trailing-zeros` | A run of `0` at the end |
| `repeat-run` | A run of the same character (`7777` and similar) |
| `ascending` | An ascending hex run (`12345` / `abcde`) |
| `descending` | A descending hex run (`54321` / `fedcb`) |
| `digits-only` | The whole hash is digits (`0-9`) |
| `letters-only` | The whole hash is letters (`a-f`) |
| `palindrome` | A palindrome of length 6 or more |
| `repeat-pattern` | A repeated pattern (`abababab`) |
| `hex-word` | `dead` `beef` `cafe` `babe` `face` `c0de` `f00d` |
| `lucky` | `777` `1234567` |

Add a new detector as a file under `src/detect/detectors/` and register it.

### Ranking

Results are ordered by score descending, then hash ascending, and the top N are printed. On a TTY, hit characters are yellow and bold. Without a TTY, no color is used. Nothing is printed if nothing matches.

```
1.   16.00  0000ab
     leading-zeros    0..4    16.00
     repeat-run       0..4    12.00
```

## Usage

```
hash-hunter <REPO> [OPTIONS]
```

| Argument / option | Meaning | Default |
| --- | --- | --- |
| `<REPO>` | Git URL or local path | Required (except with `--list-detectors`) |
| `-n, --top <N>` | Number of top results to show | `20` |
| `--min-score <BITS>` | Hide scores below this value | `0` |
| `--branch <NAME>` | Branch to scan | All branches |
| `--only <ID,...>` | Enable only these detector IDs | All |
| `--exclude <ID,...>` | Disable these detector IDs | None |
| `--list-detectors` | Print detector IDs and exit | - |

```bash
hash-hunter https://github.com/example/repo.git
hash-hunter ./my-repo --top 5 --min-score 8
hash-hunter ./my-repo --only leading-zeros,lucky
hash-hunter --list-detectors
```

Errors go to stderr as `error[<code>]: <message>` and the process exits with status 1.

| Code | When |
| --- | --- |
| `git_not_found` | The `git` command was not found |
| `clone_failed` | Clone failed, the local path is not a repository, the branch does not exist, and similar cases |
| `empty_repository` | The repository has no commits |
| `unknown_detector` | `--only` or `--exclude` named an unknown ID |

Authentication for private repositories is left to Git (SSH keys, credential helpers). This app does not store credentials.

## Setup

You need Rust (edition 2021) and a `git` command on `PATH`.

```bash
git clone https://github.com/Harukoto-Project/hash-hunter.git
cd hash-hunter
cargo run -- --help
```

To install:

```bash
cargo install --path .
hash-hunter --help
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to work on this project. Issues and pull requests are welcome.

## License

[Apache License 2.0](LICENSE).
