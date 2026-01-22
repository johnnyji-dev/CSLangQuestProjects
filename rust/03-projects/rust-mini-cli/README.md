# Rust Mini CLI

간단한 CLI 도구를 만들어 Rust 실전 감각을 쌓습니다. 입력 파싱, 파일/네트워크 I/O, 에러 처리, 테스트를 모두 포함하는 것을 목표로 합니다.

## 목표
- `clap` 혹은 `argh`로 명령행 인자 파싱
- `Result` 기반 에러 전파와 사용자 친화적 메시지
- 소규모 모듈 구조(`main.rs`, `lib.rs`, `commands/` 등) 설계

## 기능 예시
- 텍스트 파일을 읽고 라인 수/단어 수/문자 수를 출력 (`wc` 유사)
- `--format json` 옵션으로 결과를 JSON 출력
- `--watch` 옵션으로 파일 변경을 감지하고 자동 재실행 (선택)

## 실행 예시
- 기본: `cargo run --bin rust-mini-cli -- sample.txt`
- JSON 출력: `cargo run --bin rust-mini-cli -- sample.txt --format json`

## 테스트
- 유닛 테스트: `cargo test`
- CLI 스냅샷 테스트는 `assert_cmd`, `predicates`, `insta` 조합을 권장

## 다음 확장
- 멀티 스레드로 대용량 파일 스트리밍 처리 실험
- 에러를 `thiserror`로 정의하고, 사용자 메시지는 `miette`로 예쁘게 출력
