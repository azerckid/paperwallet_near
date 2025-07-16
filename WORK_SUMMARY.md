# 📝 작업 요약 및 명령어 설명 (NEAR vs. 이더리움)

이 문서는 지금까지 진행된 NEAR 스마트 컨트랙트 개발 및 배포 작업에 대한 요약, 사용된 명령어 설명, 그리고 이더리움 스마트 컨트랙트 배포 과정과의 비교를 담고 있습니다.

---

## 1. 작업 요약

저희는 NEAR 프로토콜 기반의 온체인 계정 레지스트리 스마트 컨트랙트 개발의 첫 번째 단계인 **Phase 1: 환경 설정 및 스마트 컨트랙트 개발**을 성공적으로 완료했습니다.

주요 성과는 다음과 같습니다:
*   Rust 개발 환경 설정 및 `near-sdk` 의존성 관리.
*   계정 ID와 비밀번호 해시 목록을 저장하고 조회하는 NEAR 스마트 컨트랙트의 핵심 로직 구현.
*   구현된 스마트 컨트랙트의 유닛 테스트 작성 및 성공적인 실행.
*   컴파일된 `.wasm` 파일을 NEAR 테스트넷에 성공적으로 배포.

---

## 2. 사용한 명령어 설명

지금까지 프로젝트를 진행하며 사용했던 주요 명령어들과 그 역할은 다음과 같습니다:

*   **`cargo init --lib`**
    *   **역할:** 현재 디렉토리에 새로운 Rust 라이브러리 프로젝트를 초기화합니다. `Cargo.toml` 파일과 `src/lib.rs` 파일을 생성하여 Rust 개발을 시작할 수 있는 기본 구조를 만듭니다.

*   **`cargo test`**
    *   **역할:** Rust 프로젝트 내에 작성된 모든 유닛 테스트를 실행합니다. 코드가 예상대로 작동하는지 확인하는 데 사용됩니다.

*   **`cargo build --target wasm32-unknown-unknown --release`**
    *   **역할:** Rust 소스 코드를 WebAssembly(WASM) 바이너리 파일로 컴파일합니다. `--target wasm32-unknown-unknown`은 WebAssembly 타겟을 지정하며, `--release`는 최적화된 배포용 버전을 생성하도록 지시합니다. NEAR 스마트 컨트랙트는 WASM 형태로 배포됩니다.

*   **`rustup target add wasm32-unknown-unknown`**
    *   **역할:** Rust 툴체인에 WebAssembly 컴파일을 위한 타겟을 추가합니다. 이 타겟이 있어야 Rust 코드를 WASM으로 빌드할 수 있습니다.

*   **`rustup component remove rust-std --target wasm32-unknown-unknown` / `rustup component add rust-std --target wasm32-unknown-unknown`**
    *   **역할:** 특정 타겟(여기서는 `wasm32-unknown-unknown`)에 대한 Rust 표준 라이브러리(rust-std) 컴포넌트를 제거하거나 다시 추가합니다. 빌드 환경 문제 발생 시 컴포넌트를 재설치하여 문제를 해결하는 데 사용됩니다.

*   **`cargo clean`**
    *   **역할:** 프로젝트의 `target` 디렉토리에 있는 모든 빌드 아티팩트(컴파일된 파일, 캐시 등)를 삭제합니다. 깨끗한 상태에서 다시 빌드해야 할 때 사용됩니다.

*   **`near login`**
    *   **역할:** `near-cli`를 사용하여 NEAR 계정에 로그인합니다. 웹 브라우저를 통해 NEAR 지갑과 연동하여 로컬에 자격 증명(키 파일)을 저장합니다. 이 자격 증명은 트랜잭션 서명에 사용됩니다.

*   **`near add-credentials <account-id>`**
    *   **역할:** 특정 NEAR 계정의 자격 증명(비밀 키 또는 시드 문구)을 로컬에 명시적으로 추가합니다. `near login`이 제대로 작동하지 않거나 비대화형 방식으로 키를 추가해야 할 때 사용됩니다.

*   **`export NEAR_CLI_SECRET_KEY="..."`**
    *   **역할:** 현재 셸 세션에 `NEAR_CLI_SECRET_KEY`라는 환경 변수를 설정합니다. 이 변수에 비밀 키를 할당하면 `near-cli`가 비대화형 방식으로 트랜잭션에 서명할 수 있습니다. (보안상 주의 필요)

*   **`near deploy <account-id> <wasm-file>`**
    *   **역할:** 컴파일된 WebAssembly(`.wasm`) 컨트랙트 파일을 지정된 NEAR 계정에 배포합니다. 계정은 컨트랙트 스토리지 및 트랜잭션 수수료를 지불할 NEAR 토큰을 가지고 있어야 합니다.

*   **`ls -l`**
    *   **역할:** 특정 디렉토리의 파일 및 하위 디렉토리 목록을 자세한 정보(권한, 소유자, 크기, 수정 시간 등)와 함께 표시합니다. 파일 존재 여부 및 크기 확인에 유용합니다.

*   **`rustup show`**
    *   **역할:** 현재 설치된 Rust 툴체인 및 활성화된 툴체인에 대한 정보를 표시합니다. 설치된 타겟 목록도 확인할 수 있습니다.

*   **`xcode-select --install`**
    *   **역할:** macOS에서 Xcode Command Line Tools를 설치합니다. 이는 Rust를 포함한 다양한 개발 도구의 컴파일 및 빌드에 필요한 기본 유틸리티를 제공합니다.

---

## 3. NEAR 스마트 컨트랙트 배포 vs. 이더리움 스마트 컨트랙트 배포

NEAR 프로토콜과 이더리움은 블록체인 플랫폼이지만, 스마트 컨트랙트 개발 및 배포 과정에서 몇 가지 중요한 차이점이 있습니다.

| 특징           | NEAR 프로토콜                                     | 이더리움                                          |
| :------------- | :------------------------------------------------ | :------------------------------------------------ |
| **스마트 컨트랙트 언어** | Rust (주로), AssemblyScript                       | Solidity (주로), Vyper                            |
| **컴파일러**   | `rustc` (WASM으로 컴파일)                         | `solc` (EVM 바이트코드로 컴파일)                  |
| **런타임 환경** | WebAssembly (WASM)                                | Ethereum Virtual Machine (EVM)                    |
| **주요 개발 툴링** | `cargo`, `near-cli`, `cargo-near`, `near-sdk-rs`  | Truffle, Hardhat, Remix, Ganache, `geth`, `web3.js`, `ethers.js` |
| **계정 모델**  | 인간이 읽을 수 있는 계정 ID (예: `yourname.near`), 암시적 계정 생성 | 16진수 주소 (예: `0x...`), 명시적 계정 생성       |
| **가스 모델**  | 스토리지 스테이킹 (사용자가 스토리지 비용을 스테이킹), 예측 가능한 수수료 | 가스 한도(Gas Limit), 변동성 높은 가스 가격 (Gas Price) |
| **배포 과정**  | 1. Rust 코드 작성<br>2. `cargo build --target wasm32-unknown-unknown --release`로 `.wasm` 파일 컴파일<br>3. `near deploy <account-id> <wasm-file>` 명령어로 기존 계정에 `.wasm` 파일 배포 (계정에 NEAR 토큰 필요) | 1. Solidity 코드 작성<br>2. `solc` 등으로 EVM 바이트코드 및 ABI 컴파일<br>3. `web3.js` 또는 `ethers.js` 라이브러리, Truffle/Hardhat 프레임워크를 사용하여 트랜잭션 전송 (계정에 ETH 필요) |
| **키 관리**    | 로컬 키 파일, `near login`, `NEAR_CLI_SECRET_KEY` 환경 변수 | MetaMask, 개인 키, Keystore 파일, 환경 변수       |

**주요 차이점 요약:**

*   **언어 및 런타임:** NEAR는 Rust와 WebAssembly를 사용하여 더 높은 성능과 유연성을 제공하는 반면, 이더리움은 Solidity와 EVM을 통해 광범위한 생태계를 구축했습니다.
*   **계정 및 가스 모델:** NEAR는 인간 친화적인 계정 ID와 예측 가능한 가스 모델을 통해 사용자 경험을 개선하려 합니다. 이더리움은 16진수 주소와 변동성 높은 가스 가격을 가집니다.
*   **툴링:** 두 플랫폼 모두 개발을 위한 풍부한 툴링을 제공하지만, NEAR는 Rust 생태계의 `cargo`와 긴밀하게 통합되어 있습니다.

이 요약이 지금까지의 작업과 명령어들을 이해하는 데 도움이 되기를 바랍니다.