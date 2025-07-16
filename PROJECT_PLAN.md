# 📝 온체인 계정 레지스트리 프로젝트 기획안

## 1. 프로젝트 개요

### 1.1. 목표

하나의 계정(일반 주소, 예: BTC 주소)에 여러 개의 비밀번호를 연동하여 관리할 수 있으며, 이 정보가 Supabase 데이터베이스에 추가/변경될 때마다 NEAR 프로토콜 블록체인에 **자동으로** 안전하게 기록하는 시스템을 구축한다. 이를 통해 데이터의 무결성과 투명성을 보장한다.

### 1.2. 핵심 기능

- **데이터베이스 연동**: 사용자가 웹을 통해 Supabase DB에 계정 또는 비밀번호를 신규 등록/변경하는 것을 Webhook으로 감지.
- **자동화된 온체인 기록**: 감지된 데이터를 백엔드 서비스가 처리하여 NEAR 스마트 컨트랙트에 자동으로 기록.
- **보안**: 데이터베이스에 이미 해싱(Hashing)되어 저장된 비밀번호 값을 **변형 없이 그대로** 블록체인에 기록하여 데이터의 무결성을 보장한다. 백엔드 서비스는 추가적인 해싱을 수행하지 않는다.
- **데이터 조회**: 특정 계정에 연결된 모든 비밀번호의 해시 목록을 블록체인에서 조회하는 기능 제공.

---

## 2. 시스템 아키텍처

```
[사용자 앱] ---> [Supabase DB] --(Webhook)--> [백엔드 (Supabase Function)] ---> [NEAR 스마트 컨트랙트]
```

- **Supabase**: 주 데이터베이스. `accounts`와 `passwords` 테이블을 관리하며, 데이터 변경 시 Webhook을 통해 백엔드에 알림을 보낸다.
- **백엔드 (Supabase Function)**: Deno/TypeScript 기반의 서버리스 함수. Webhook 요청을 받아 비밀번호를 해싱하고, NEAR 트랜잭션을 생성 및 서명하여 스마트 컨트랙트를 호출한다.
- **NEAR 스마트 컨트랙트**: Rust로 작성된 온체인 데이터 저장소. 계정 식별자와 비밀번호 해시 목록을 `UnorderedMap` 자료구조에 저장한다.

---

## 3. 개발 단계 (Phases)

1.  **Phase 1: 환경 설정 및 스마트 컨트랙트 개발**

    - 개발 도구 설치 및 프로젝트 구조 설정.
    - Rust로 데이터 저장/조회 기능이 있는 NEAR 스마트 컨트랙트 작성 및 테스트넷 배포.

2.  **Phase 2: 백엔드 서비스 개발**

    - Supabase Function 프로젝트 설정.
    - 데이터 수신, 해싱, NEAR 컨트랙트 호출 로직 구현.

3.  **Phase 3: 통합 및 테스트**

    - Supabase Webhook 설정.
    - 전체 시스템의 End-to-End 테스트 진행.

4.  **Phase 4: 배포 및 운영**
    - Supabase Function 및 스마트 컨트랙트 메인넷 배포.
    - 환경 변수 등 보안 설정 강화.

---

## 4. 기술 스택

- **Blockchain**: NEAR Protocol
- **Database/BaaS**: Supabase
- **Smart Contract**: Rust (`near-sdk-rs`)
- **Backend**: Deno, TypeScript (Supabase Functions)
- **Tooling**: `near-cli`, `supabase-cli`, `gemini-cli`
