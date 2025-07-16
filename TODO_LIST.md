# 📝 프로젝트 상세 작업 목록 (To-Do List)

이 문서는 `PROJECT_PLAN.md`에 명시된 각 개발 단계를 구체적인 작업 항목으로 나눈 목록입니다.

---

## Phase 1: 환경 설정 및 스마트 컨트랙트 개발

- [x] **Rust 프로젝트 초기화**
  - [x] `onchain-registry` 디렉토리로 이동
  - [x] `cargo init --lib` 명령어로 Rust 라이브러리 프로젝트 생성
- [x] **`Cargo.toml` 의존성 설정**
  - [x] `near-sdk` 라이브러리 추가
- [x] **스마트 컨트랙트 데이터 구조 정의 (`src/lib.rs`)**
  - [x] `#[near_bindgen]` 매크로와 함께 기본 `Contract` struct 선언
  - [x] 계정 ID(String)와 비밀번호 해시 목록(`Vec<String>`)을 저장할 `UnorderedMap` 정의
- [x] **스마트 컨트랙트 핵심 함수 구현 (`src/lib.rs`)**
  - [x] `add_password(account_id: String, password_hash: String)`: 계정에 비밀번호 해시를 추가하는 함수. 계정이 없으면 새로 생성하고, 있으면 기존 목록에 추가.
  - [x] `get_passwords(account_id: String) -> Vec<String>`: 특정 계정에 저장된 모든 비밀번호 해시 목록을 반환하는 조회 함수.
- [x] **유닛 테스트 작성**
  - [x] `add_password` 기능 정상 동작 테스트
  - [x] `get_passwords` 기능 정상 동작 테스트
  - [x] 존재하지 않는 계정 조회 시, 빈 목록(`[]`)이 반환되는지 테스트
- [ ] **NEAR 테스트넷 배포**
  - [x] `near-cli` 설치 및 테스트넷 계정 생성
    - [x] `cargo build --target wasm32-unknown-unknown --release` 명령어로 컨트랙트 컴파일
    - [x] `near deploy` 명령어를 사용하여 컴파일된 `wasm` 파일을 테스트넷에 배포
  - [ ] beyond fly there fat slice clay leg slam dash ridge color tube
  - [ ] private key를 안전하게 관리하기 위한 `.env` 파일 생성 ed25519:5fN8pDmEfSv4Y5PJWf2SBh6JR3YmUTp7FwT4tCDEPsrE2Z4xz2LQqAt428cyeD9qzRionRWkRk8h56C7CGDyn29u
  - [ ] testaccount = geminitest.testnet
  - [ ] https://docs.near.org/api/rpc/providers
  - [ ] export NEAR_CLI_TESTNET_RPC_SERVER_URL=https://rpc.web4.testnet.page/account/testnet

---

## Phase 2: 백엔드 서비스 개발 (Supabase Function)

- [ ] **로컬 Supabase 환경 설정**
  - [ ] `supabase init`으로 Supabase 프로젝트 초기화
  - [ ] `supabase start`로 로컬 Supabase 서비스 실행
- [ ] **Supabase Function 생성**
  - [ ] `supabase functions new onchain-sync` 명령어로 함수 생성
- [ ] **`onchain-sync` 함수 구현 (`index.ts`)**
  - [ ] Webhook 요청(Request) 본문에서 계정, 비밀번호 해시 데이터 파싱
  - [ ] `near-api-js` 라이브러리를 사용하여 NEAR 테스트넷과 연동 설정
  - [ ] 환경 변수를 통해 백엔드 서비스의 NEAR 계정 정보 및 Private Key 관리
  - [ ] 스마트 컨트랙트의 `add_password` 함수를 호출하는 트랜잭션 생성 및 전송
  - [ ] 모든 과정에 대한 에러 처리 및 로그 기록 로직 추가
- [ ] **환경 변수 설정**
  - [ ] `.env` 파일에 NEAR 테스트넷 관련 정보 (계정 ID, 컨트랙트 ID) 설정

---

## Phase 3: 통합 및 테스트

- [ ] **Supabase 데이터베이스 트리거 설정**
  - [ ] `passwords` 테이블에 새로운 행(Row)이 `INSERT`될 때 `onchain-sync` 함수를 호출하는 데이터베이스 함수(Trigger) 작성
- [ ] **End-to-End (E2E) 테스트**
  - [ ] **시나리오 1:** Supabase에 새로운 계정의 비밀번호를 추가
  - [ ] **검증 1:** `onchain-sync` 함수 로그를 확인하여 정상 실행 여부 검증
  - [ ] **검증 2:** `near-cli`를 사용하여 NEAR 테스트넷의 스마트 컨트랙트에서 `get_passwords`를 호출, 데이터가 정확히 기록되었는지 확인
  - [ ] **시나리오 2:** 기존에 있던 계정에 다른 비밀번호를 추가
  - [ ] **검증 3:** 위와 동일한 방법으로 데이터가 정상적으로 누적되는지 확인

---

## Phase 4: 배포 및 운영

- [ ] **NEAR 스마트 컨트랙트 메인넷 배포**
  - [ ] 메인넷용 NEAR 계정 생성 및 `near-cli` 설정
  - [ ] `near deploy`를 사용하여 메인넷에 최종 컨트랙트 배포
- [ ] **Supabase Function 배포**
  - [ ] `supabase functions deploy onchain-sync` 명령어로 함수 배포
- [ ] **Supabase 프로젝트 환경 변수 설정**
  - [ ] Supabase 프로젝트 대시보드에서 메인넷용 NEAR 계정 정보, Private Key, 컨트랙트 ID 등 환경 변수 설정
- [ ] **최종 프로덕션 테스트**
  - [ ] 실제 운영 환경에서 E2E 테스트를 다시 한번 수행하여 모든 기능이 정상 동작하는지 최종 확인
- [ ] **모니터링 및 유지보수**
  - [ ] Supabase 및 NEAR 네트워크의 상태를 주기적으로 확인할 수 있는 방법론 수립
