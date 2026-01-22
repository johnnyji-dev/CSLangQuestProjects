# CSLangQuestProjects

Computer Science 언어 학습과 실전 프로젝트를 단계적으로 기록하는 저장소입니다. 기본 이론 복습부터 작은 실습, 미니 프로젝트까지 한 곳에서 관리하며 학습 흐름을 유지하는 것을 목표로 합니다.

## 목표
- CS 기초(자료구조, 알고리즘, 시스템)에 필요한 언어 실습을 꾸준히 진행
- 짧은 주기의 실전 예제를 통해 개념을 빠르게 검증
- 프로젝트별 회고를 남겨 학습 루프를 완성

## 구조 제안
- `basics/<언어>/`: 언어 문법, 자료구조/알고리즘 기초 예제
- `exercises/<언어>/`: 짧은 문제 풀이 및 실습 스니펫
- `projects/<주제>/`: 주제별 미니 프로젝트(README 포함)
- `notes/`: 요약 정리, 회고 및 참고 링크

## 언어 학습 순서 제안
- Rust → Go → Solidity
  - Rust: 안전한 시스템 프로그래밍 감각과 소유권 모델 체득
  - Go: 동시성·서비스 개발을 단순 문법으로 빠르게 익힘
  - Solidity: 스마트 컨트랙트는 보안/가스 모델 특화, 마지막에 집중 학습

### 진행 팁
- 각 언어의 `basics/`→`exercises/`→`projects/` 순으로 난이도 상승
- 최소 단위 실습마다 `notes/`에 회고를 남기고, 다음 목표를 명시

## 시작 가이드
1) 이 저장소를 로컬에 클론합니다.
2) 언어별로 `basics/<언어>/`에 작은 단위 실습을 추가하고, 실행 방법과 배운 점을 각 디렉터리의 README에 적습니다.
3) 새 프로젝트를 시작할 때는 `projects/<주제>` 폴더를 만들고 목표/기능/사용법/회고를 기록합니다.

## 브랜치 & 커밋 가이드
- 작은 단위 기능/예제마다 브랜치를 만들어 작업 후 병합
- 커밋 메시지 예시 (유형별)
  - 기능 추가: `feat: add rust ownership notes`, `feat: go goroutine demo`
  - 버그 수정: `fix: go channel leak in worker`, `fix: rust borrow example typo`
  - 문서/정리: `docs: add solidity gas tips`, `docs: update roadmap`
  - 환경/유지보수: `chore: bump toolchain versions`, `chore: add makefile lint target`

## 학습 로그 예시
- 날짜: 2026-01-22
- 내용: Python 리스트/스택 기본 연습, 간단한 DFS 구현
- 다음 할 일: BFS 추가, 시간 복잡도 정리, `notes/`에 요약 업로드

## 라이선스
필요 시 MIT 또는 Apache-2.0 라이선스로 공개를 고려하세요. 기본값은 미정 상태입니다.
