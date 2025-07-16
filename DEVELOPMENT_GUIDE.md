# 🚀 온체인 계정 레지스트리: 개발 및 테스트 가이드

이 문서는 `PROJECT_PLAN.md`에 기술된 목표를 달성하기 위한 구체적인 개발 단계와 테스트 절차를 안내합니다.

---

## Phase 1: 환경 설정 및 스마트 컨트랙트 개발

### 1.1. 필수 도구 설치

> 💡 **Mac/Homebrew 설정 팁**: `~/.zshrc` 파일의 **가장 윗부분**에 `eval "$(/opt/homebrew/bin/brew shellenv)"` 라인이 있는지 확인하세요. 설정을 변경한 후에는 **반드시 새 터미널 창을 열어야** 올바르게 적용됩니다.

- [x] **Rust 및 Cargo 설치**: (Homebrew 또는 공식 스크립트를 통해 완료)
- [x] **NEAR CLI 설치**: `npm install -g near-cli`
- [x] **Rust WASM 빌드 타겟 추가**: `rustup target add wasm32-unknown-unknown`
- [x] **Supabase CLI 설치**: `brew install supabase/tap/supabase` (Mac/Docker 필요)

### 1.2. 프로젝트 구조 설정

- [x] **프로젝트 폴더 생성 및 이동**: `mkdir onchain-registry && cd onchain-registry`
- [x] **NEAR 스마트 컨트랙트 프로젝트 생성**: `npx create-near-app --contract=rust --frontend=none contract`

> **참고**: `near-cli`의 `new` 명령어는 오래된 버전에서 사용되었습니다. 현재는 `create-near-app` 도구를 사용하는 것이 공식적인 방법입니다. 이 도구는 `contract`라는 이름의 폴더에 Rust 스마트 컨트랙트 프로젝트를 생성합니다.

### 1.3. 스마트 컨트랙트 코드 작성

- [ ] **`gemini-cli`로 코드 생성**: `contract/src/lib.rs` 파일을 열고, 아래 명령어를 실행하여 컨트랙트 로직을 구현합니다.

```bash
gemini-cli "NEAR 스마트 컨트랙트를 Rust로 작성해줘. 기능은 다음과 같아: 1. '계정 식별자(String)'를 키로, '비밀번호 해시 목록(Vec<String>)'을 값으로 가지는 UnorderedMap을 상태로 관리. 2. 컨트랙트 소유자만 호출할 수 있는 'add_password_hash(account_id, password_hash)' 함수 구현. 3. 특정 계정의 해시 목록을 조회하는 'get_password_hashes(account_id)' 뷰 함수 구현. 이 코드를 contract/src/lib.rs 파일에 적용해줘."
```

### 1.4. 컨트랙트 빌드 및 테스트넷 배포

```bash
# 1. 컨트랙트 빌드 (contract 폴더 내에서 실행)
cargo build --target wasm32-unknown-unknown --release

# 2. NEAR 테스트넷 로그인
near login

# 3. 컨트랙트 배포 (your-account.testnet은 본인 계정으로 변경)
near deploy --wasmFile target/wasm32-unknown-unknown/release/contract.wasm \
  --accountId contract.your-account.testnet \
  --initFunction 'new' \
  --initArgs '{"owner_id": "backend-service.your-account.testnet"}'
```

> **참고**: `backend-service.your-account.testnet`은 백엔드 서비스가 사용할 전용 NEAR 계정입니다. 미리 생성해두세요.

---

## Phase 2: 백엔드 서비스 개발 (Supabase Function)

### 2.1. Supabase 프로젝트 설정

프로젝트 루트 디렉토리(`onchain-registry`)에서 Supabase 프로젝트를 초기화합니다.

```bash
supabase login
supabase init
supabase functions new record-on-chain
```

### 2.2. 백엔드 함수 코드 작성

`gemini-cli`를 사용하여 `supabase/functions/record-on-chain/index.ts` 파일의 로직을 구현합니다.

```bash
gemini-cli "Deno/TypeScript 기반의 Supabase Function 코드를 작성해줘. 이 함수는 Supabase Webhook으로부터 'record' 객체를 JSON으로 받아. 'record'에는 'account_identifier'와 'password' 필드가 있어. 함수는 다음 작업을 수행해야 해: 1. 'password'를 SHA-256으로 해싱. 2. near-api-js를 사용해 이전에 배포한 NEAR 스마트 컨트랙트의 'add_password_hash' 함수를 호출하여 'account_identifier'와 해싱된 비밀번호를 인자로 전달. 환경 변수에서 NEAR 계정 정보와 컨트랙트 주소를 가져오도록 해. 이 코드를 supabase/functions/record-on-chain/index.ts 파일에 적용해줘."
```

### 2.3. 로컬 환경 변수 설정

로컬 테스트를 위해 `supabase/.env` 파일을 생성하고 민감한 정보를 추가합니다.

```
# supabase/.env
NEAR_PRIVATE_KEY=your_backend_service_account_private_key
NEAR_ACCOUNT_ID=backend-service.your-account.testnet
NEAR_CONTRACT_NAME=contract.your-account.testnet
```

---

## Phase 3: 통합 및 테스트 (End-to-End)

> 💡 **워크플로우 선택**: 아래는 **로컬 개발 환경** 기준의 테스트 절차입니다. 만약 **Supabase 클라우드**에서 직접 개발한다면, `supabase start` 대신 Supabase 대시보드에 접속하여 모든 작업을 수행하고, 함수 배포 및 환경 변수 설정도 대시보드에서 직접 진행해야 합니다.

### 3.1. 로컬 환경 테스트 절차

1.  **Supabase 로컬 환경 실행**: 터미널에서 `supabase start` 명령어로 로컬 개발 환경을 시작합니다.
2.  **데이터베이스 테이블 생성**: 출력된 로컬 Supabase Studio 주소로 접속하여 SQL Editor에서 `passwords` 테이블을 생성합니다.
3.  **Webhook 설정**: 로컬 Studio의 `Database > Webhooks` 메뉴로 이동합니다. `passwords` 테이블에 `INSERT` 이벤트가 발생할 때, `record-on-chain` 함수를 호출하도록 Webhook을 설정합니다.
4.  **데이터 삽입**: `passwords` 테이블에 테스트용 데이터를 직접 삽입하여 Webhook이 트리거되는지 확인합니다.
5.  **로그 확인**: `supabase functions serve`를 실행 중인 터미널에서 백엔드 함수의 로그를 확인하여 NEAR 트랜잭션이 성공적으로 전송되었는지 확인합니다.

### 3.2. 온체인 데이터 최종 검증

로컬 테스트 또는 클라우드 배포 후, `near-cli`를 사용하여 블록체인에 데이터가 올바르게 기록되었는지 최종 확인합니다.

    ```bash
    near view contract.your-account.testnet get_password_hashes '{"account_id": "테스트한_계정_ID"}'
    ```
    명령어 실행 결과로 방금 추가한 비밀번호의 해시 값이 포함된 배열이 반환되면 통합 테스트가 성공한 것입니다.

### 3.3. (참고) 클라우드 환경에 함수 배포

로컬에서 개발 및 테스트가 완료된 함수는 아래 명령어로 Supabase 클라우드 프로젝트에 배포할 수 있습니다.

```bash
# 1. Supabase 프로젝트와 연결
supabase link --project-ref <your-project-ref>

# 2. 클라우드에 환경 변수(Secret) 설정
supabase secrets set --env-file ./supabase/.env

# 3. 함수 배포
supabase functions deploy record-on-chain
```
