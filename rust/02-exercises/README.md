# Rust Exercises

짧은 문제 풀이로 개념을 굳힙니다. 각 예제는 단일 바이너리로 두고 `cargo run --bin <name>` 형태로 실행하거나, `rustc file.rs`로 단독 테스트하세요.

## 진행 방식
- 각 주제별 최소 예제 작성 → `cargo fmt`/`cargo clippy`로 점검
- README에 배운 점/실패 사례를 간단히 기록
- 입력·출력 예시를 주석으로 남겨 재현 가능성 확보

## 예제 아이디어
- `ownership_pairs.rs`: 소유권 이전과 빌림을 혼합한 데이터 변환
- `lifetimes_notes.rs`: 라이프타임 주석을 요구하는 함수 시그니처 연습
- `iter_collect.rs`: 이터레이터 체이닝으로 필터/매핑/집계
- `error_wrap.rs`: 표준 에러를 커스텀 에러로 감싸 반환
- `thread_pool_mini.rs`: 간단한 스레드 풀(고루틴 느낌) 구현
