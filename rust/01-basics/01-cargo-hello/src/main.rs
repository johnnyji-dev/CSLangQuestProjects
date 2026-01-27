// 01-cargo-hello/src/main.rs
// 목적: Cargo 패키지 구조에서 헬로월드를 빌드/실행해 본다.
// 실행: `cargo run` (현재 디렉터리에서) 또는 `cargo run --bin cargo-hello`
// 차이: rustc 단일 파일 컴파일과 달리 Cargo는 패키지/의존성/프로필을 관리한다.

fn main() {
    println!("Hello, Cargo!");
}
