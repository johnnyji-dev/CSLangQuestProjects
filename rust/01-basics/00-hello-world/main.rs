// 00-hello-world/main.rs
// 목적: Rust 툴체인이 정상 동작하는지 확인하고 최소 실행 흐름을 익힌다.
// 실행:
// - 단일 파일(rustc): `rustc main.rs -o hello && ./hello`
//   - rustc로 단일 파일을 바로 컴파일해 바이너리를 생성(빠른 확인용)
//   - 의존성/패키지 관리 없음, 가장 단순한 경로
// - Cargo 프로젝트가 있을 때: `cargo run --bin <bin-name>`
//   - Cargo가 빌드/실행을 함께 수행하며 의존성, 프로필(debug/release) 등을 관리
//   - 이 폴더는 Cargo.toml이 없으므로 rustc 방식으로 실행하세요.

fn main() {
    // println! 매크로로 문자열을 출력한다.
    // 매크로 호출은 느낌표(!)를 사용하며, 서식 문자열과 인자를 받아 컴파일 타임에 확장된다.
    println!("Hello, Rust!");
}
