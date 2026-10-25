# Hash Hunter

Git 저장소의 커밋 해시에서 앞자리 0이나 회문 같은 드문 패턴을 찾아 순위를 매기는 CLI입니다.

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-edition%202021-dea584.svg)](https://www.rust-lang.org/)

[개요](#개요) · [기능](#기능) · [사용법](#사용법) · [설치](#설치) · [기여](#기여) · [라이선스](#라이선스)

[日本語](README.md) · [English](README.en.md) · [한국어](README.ko.md)

---

## 개요

Hash Hunter는 Git URL 또는 로컬 경로를 받아 기존 커밋 해시를 평가하고, 드문 패턴을 순위표로 출력합니다.

대상은 커밋 해시뿐입니다. 논스를 붙여 커밋을 다시 만드는 마이닝은 구현되어 있지 않습니다.

## 기능

### 기록 스캔

URL이면 임시 디렉터리에 bare clone하고, 로컬 경로면 그 저장소를 그대로 사용합니다. `git rev-list`로 커밋을 나열한 뒤 각 해시를 검출기에 통과시킵니다. 임시 디렉터리는 종료 시 삭제합니다.

SHA-1(40자)과 SHA-256(64자) 모두 길이를 고정하지 않고 처리합니다.

### 희귀도 판정

각 검출기는 히트 여부와 희귀도 점수(bits)를 반환합니다. 점수는 `-log2(우연히 그 조건을 만족할 확률)`입니다. 하나의 해시에 여러 히트가 있으면 겹치는 구간을 제거하지 않고 점수를 그대로 합산합니다.

| ID | 내용 |
| --- | --- |
| `leading-zeros` | 앞쪽 `0`의 연속 |
| `trailing-zeros` | 끝쪽 `0`의 연속 |
| `repeat-run` | 같은 문자의 연속(`7777` 등) |
| `ascending` | 16진 오름차순 연속(`12345` / `abcde`) |
| `descending` | 16진 내림차순 연속(`54321` / `fedcb`) |
| `digits-only` | 해시 전체가 숫자(`0-9`) |
| `letters-only` | 해시 전체가 영문(`a-f`) |
| `palindrome` | 길이 6 이상의 회문 |
| `repeat-pattern` | 같은 패턴의 반복(`abababab`) |
| `hex-word` | `dead` `beef` `cafe` `babe` `face` `c0de` `f00d` |
| `lucky` | `777` `1234567` |

새 검출기는 `src/detect/detectors/`에 파일을 추가하고 레지스트리에 등록합니다.

### 순위 표시

점수 내림차순, 동점이면 해시 오름차순으로 상위 N건을 출력합니다. TTY에서는 히트 문자를 노란 굵은 글씨로 강조하고, TTY가 아니면 색을 쓰지 않습니다. 해당하는 항목이 없으면 아무것도 출력하지 않습니다.

```
1.   16.00  0000ab
     leading-zeros    0..4    16.00
     repeat-run       0..4    12.00
```

## 사용법

```
hash-hunter <REPO> [OPTIONS]
```

| 인자·옵션 | 내용 | 기본값 |
| --- | --- | --- |
| `<REPO>` | Git URL 또는 로컬 경로 | 필수(`--list-detectors`일 때는 불필요) |
| `-n, --top <N>` | 표시할 상위 건수 | `20` |
| `--min-score <BITS>` | 이 점수 미만은 표시하지 않음 | `0` |
| `--branch <NAME>` | 대상 브랜치 | 모든 브랜치 |
| `--only <ID,...>` | 사용할 검출기 ID만 지정 | 전부 |
| `--exclude <ID,...>` | 끌 검출기 ID | 없음 |
| `--list-detectors` | 검출기 목록을 출력하고 종료 | - |

```bash
hash-hunter https://github.com/example/repo.git
hash-hunter ./my-repo --top 5 --min-score 8
hash-hunter ./my-repo --only leading-zeros,lucky
hash-hunter --list-detectors
```

오류는 표준 오류에 `error[<code>]: <message>` 형식으로 출력하고, 종료 코드는 1입니다.

| 코드 | 상황 |
| --- | --- |
| `git_not_found` | `git` 명령을 찾을 수 없음 |
| `clone_failed` | clone 실패, 로컬 경로가 저장소가 아님, 브랜치가 없음 등 |
| `empty_repository` | 커밋이 0건 |
| `unknown_detector` | `--only` / `--exclude`에 없는 ID |

비공개 저장소 인증은 SSH 키나 credential helper 등 git 쪽 설정에 맡깁니다. 이 앱은 인증 정보를 갖지 않습니다.

## 설치

Rust(edition 2021)와 PATH의 `git` 명령이 필요합니다.

```bash
git clone https://github.com/Harukoto-Project/hash-hunter.git
cd hash-hunter
cargo run -- --help
```

설치하려면:

```bash
cargo install --path .
hash-hunter --help
```

## 기여

작업 방법은 [CONTRIBUTING.md](CONTRIBUTING.md)를 참고하세요. Issue와 Pull Request를 환영합니다.

## 라이선스

[Apache License 2.0](LICENSE)입니다.
