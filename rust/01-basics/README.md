# Rust Basics

Rust의 핵심 개념(소유권, 빌림, 라이프타임)을 빠르고 깊게 익히기 위한 기초 트랙입니다. 각 주제는 30~90분 내 실습을 목표로 하며, 실행/테스트 결과와 배운 점을 간단히 기록합니다.

## 추천 학습 순서 (numbering)
1) 환경 준비 및 툴체인 확인
2) 소유권/이동/복사/슬라이스와 borrow checker 체험
3) 참조와 라이프타임 표기, 함수/구조체 시그니처 설계
4) `enum` + `match`, `Result`/`Option` 패턴으로 제어 흐름 익히기
5) 컬렉션(`Vec`, `String`, `HashMap`)과 이터레이터 체이닝으로 데이터 처리
6) 에러 처리: `?`, 에러 래핑, `thiserror`로 커스텀 에러 정의
7) 모듈/크레이트 구조, `pub` 가시성, 워크스페이스 감각
8) 동시성 기초: `std::thread`, `JoinHandle`, `Send/Sync` 개념
9) (선택) `async/.await` 기초로 확장

주제마다 작은 예제를 만들고, 끝날 때마다 실행 결과와 회고를 기록하세요.

## 환경 준비
- 설치
  - 스크립트: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    - `rustup` 설치 스크립트로 stable 툴체인, `cargo`, `rustc`가 기본 설치됩니다.
    - 설치 중 프로파일은 기본(default) 선택을 권장합니다(필요 시 minimal 선택 가능).
    - PATH 반영 안내가 나오면 터미널을 재시작하거나 안내된 명령을 실행해 PATH를 갱신하세요.
    - 제거가 필요하면 `rustup self uninstall`로 정리할 수 있습니다.
  - 기본 toolchain 확인: `rustc --version`, `cargo --version`
    - 설치/업데이트가 정상 적용됐는지 버전을 확인합니다.
    - 협업 시 버전 차이로 인한 빌드 편차를 줄이기 위해 기록해 두면 좋습니다.
  - 최신 안정화 업데이트: `rustup update stable`
    - stable 채널을 최신으로 맞춰 예제/도구가 동일 환경에서 동작하도록 합니다.
    - 새 lint/언어 기능이 필요하거나 보안 패치가 있을 때 반드시 실행하세요.
- 포맷/린트
 - 포맷: `cargo fmt`
   - rustfmt로 전체 워크스페이스를 일관된 스타일로 자동 정렬합니다.
   - `.rustfmt.toml`를 두면 팀 규칙을 커스터마이즈할 수 있습니다.
   - 커밋 전 자동 포맷을 위해 pre-commit 훅이나 CI에서 실행하는 것을 권장합니다.
 - 린트: `cargo clippy -- -D warnings`
   - clippy의 권장·pedantic 룰을 적용해 잠재 버그/냄새를 잡습니다.
   - `-D warnings`로 모든 경고를 컴파일 오류로 승격해 CI에서 놓치지 않도록 합니다.
   - 특정 린트를 무시해야 하면 해당 구문에 `#[allow(clippy::<lint_name>)]`를 한정적으로 사용하세요. 무분별한 전역 allow는 지양합니다.
- 실행/테스트
  - 바이너리 실행: `cargo run -p <패키지>` 또는 단일 파일 `rustc file.rs && ./file`
    - 워크스페이스에서 특정 바이너리 패키지를 실행할 때 `-p`를 사용합니다.
    - 단일 파일 실습은 빠르게 컴파일/실행하기 위해 `rustc file.rs && ./file`로 테스트할 수 있습니다.
  - 테스트: `cargo test`
    - 기본은 유닛 테스트, `cargo test -- --ignored`로 무시된 테스트 실행.
    - `cargo test --package <pkg> --test <file>`로 특정 패키지/통합 테스트만 실행 가능.
- 문서
  - 표준 라이브러리 로컬 문서: `rustup doc --std`
    - 오프라인에서도 std 문서를 탐색할 수 있어 빠른 레퍼런스로 유용합니다.
  - 크레이트 문서 빌드: `cargo doc --open`
    - 프로젝트의 public API 문서를 로컬로 빌드/브라우징하여 공개 범위와 타입 정보를 검증합니다.

## 학습 흐름 체크리스트
- 소유권/이동/복사/슬라이스, `borrow checker` 동작 이해
- 참조와 라이프타임 표기, 함수 인자/반환 시 라이프타임 설계
- `enum` + `match`, `Result`/`Option` 패턴
- 컬렉션: `Vec`, `String`, `HashMap`; 이터레이터 어댑터( `map`, `filter`, `collect` )
- 에러 처리: `?` 연산자, 에러 래핑, `thiserror`로 커스텀 에러
- 모듈/크레이트 구조, `pub` 가시성, `cargo` 워크스페이스 감각
- 동시성 기초: `std::thread`, `JoinHandle`, `Send/Sync`; 이후 `async/.await`로 확장

## 추천 실습 세트
- `ownership_intro.rs`: 이동/복사/빌림 기본, 함수 인자 전달 패턴
- `lifetimes_fn.rs`: 함수/구조체에서 라이프타임 주석 적용
- `match_result.rs`: `Result` 처리와 에러 래핑, `?` 연산자
- `iter_playground.rs`: 이터레이터 체이닝, 소유권/빌림이 섞일 때의 수집 패턴
- `concurrency_threads.rs`: 스레드 생성/조인, move 클로저, 데이터 공유 시 소유권 패턴

## 기록 템플릿 (실습 후 README에 추가)
- 파일/주제:
- 실행/테스트: 명령어와 주요 출력
- 배운 점:
- 막힌 점/해결:
- 다음 할 일:
