# Rust Notes & Retros

학습 중 발견한 개념, 시행착오, 회고를 간단히 기록합니다.

## 기록 템플릿
- 날짜:
- 주제/파일:
- 배운 점:
- 막힌 점/해결:
- 다음 할 일:

## 참고 링크
- Rust Book: https://doc.rust-lang.org/book/
- Rustlings: https://github.com/rust-lang/rustlings
- Error Handling: https://nick.groenen.me/posts/rust-error-handling/
- Async: https://rust-lang.github.io/async-book/

## 기록 템플릿

- 날짜: 2026-02-03#02
- 주제/파일:
    >> rust/01-basics/01-cargo-hello/src/main.rs
- 배운 점: Rust Toolchain(cargo)가 정상적으로 동작하는 확인하는 간단한 예시.
    >> cargo : Cargo.toml을 기준으로 패키지·의존성·빌드 프로필(debug/release)을 관리. 재현성, 멀티 파일/모듈, 외부 크레이트 사용에 적합.
        >> 사용법 (⚠️ Cargo.toml이 있는 프로젝트 루트 디렉토리에서 실행)
            >> 컴파일
                >> cargo build : 개발 모드로 컴파일 (target/debug/에 바이너리 생성)
                    >> Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
                >> cargo build --release : 릴리스 모드로 컴파일 (최적화, target/release/에 생성)
                    >> Compiling cargo-hello v0.1.0 (rust/01-basics/01-cargo-hello)
                    >> Finished `release` profile [optimized] target(s) in 3.49s
                >> cargo check : 컴파일 확인만 (바이너리 생성 안 함, 빠름)
                    >> Checking cargo-hello v0.1.0 (rust/01-basics/01-cargo-hello)
                    >> Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
            >> 실행
                >> cargo run : 컴파일 후 즉시 실행
                    >> Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
                    >> Running `target/debug/cargo-hello`
                    >> Hello, Cargo!
                >> ./target/debug/cargo-hello : 빌드 후 직접 실행
                    >> ./target/debug/cargo-hello 
                    >> Hello, Cargo!
            >> 기타
                >> cargo clean : 빌드 산출물 삭제 (target/ 폴더)
                    >> Removed 46 files, 1.8MiB total
                >> cargo test : 테스트 실행
                    >> Compiling cargo-hello v0.1.0 (rust/01-basics/01-cargo-hello)
                    >> Finished `test` profile [unoptimized + debuginfo] target(s) in 0.91s
                    >> Running unittests src/main.rs (target/debug/deps/cargo_hello-e248a537ef8b204b)
                    >> running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
- 막힌 점/해결: 없음
- 다음 할 일: 없음

===== ===== ===== ===== =====

- 날짜: 2026-02-03#01
- 주제/파일:
    >> rust/01-basics/00-hello-world/main.rs
- 배운 점: Rust Toolchain(rustc)가 정상적으로 동작하는 확인하는 간단한 예시.
    >> rustc : 단일 파일을 바로 컴파일해 빠르게 결과를 확인, 의존성/패키지를 관리하지않아, 실습실험에 적합하다.
        >> 사용법
            >> 컴파일(바이너리 생성 file) : rustc file.rs
            >> 바이너리 실행 : ./file
- 막힌 점/해결: 없음
- 다음 할 일: 없음