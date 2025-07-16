// near-sdk에서 필요한 모듈과 타입을 가져옵니다.
// borsh: 컨트랙트의 상태(state)를 직렬화/역직렬화하기 위한 라이브러리입니다. 블록체인에 데이터를 저장하고 읽기 위해 필요합니다.
// collections::UnorderedMap: NEAR 프로토콜의 스토리지에 최적화된 해시맵(key-value 저장소)입니다.
// near_bindgen: 이 매크로는 Rust 구조체와 그 구현체를 NEAR 스마트 컨트랙트로 만들어주는 역할을 합니다. 
// AccountId: NEAR 계정 ID를 다루기 위한 타입입니다. (예: "user.testnet")
// PanicOnDefault: 컨트랙트가 `new` 함수를 통해 정식으로 초기화되지 않고 기본값으로 생성되는 것을 방지합니다.
use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

// `#[near_bindgen]` 매크로를 통해 이 구조체가 스마트 컨트랙트의 핵심이 됨을 선언합니다.
#[near_bindgen]
// 아래 `derive` 매크로들은 컨트랙트 상태 관리에 필수적입니다.
// BorshDeserialize, BorshSerialize: 이 구조체(Contract)의 인스턴스를 블록체인 스토리지에 저장하거나 읽어올 수 있게 합니다.
// PanicOnDefault: `new` 초기화 함수가 호출되지 않은 상태에서 컨트랙트가 사용되는 것을 막아 안정성을 높입니다.
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // 컨트랙트의 소유자 계정 ID를 저장합니다. 상태 변경 함수는 이 소유자만 호출할 수 있습니다.
    owner_id: AccountId,
    // 이 컨트랙트의 주된 데이터 저장소입니다.
    // - 키(Key): `AccountId` 타입의 NEAR 계정 ID
    // - 값(Value): `Vec<String>` 타입의 비밀번호 해시 목록
    // 표준 Rust의 HashMap 대신 UnorderedMap을 사용하는 이유는 NEAR의 스토리지 사용 방식에 더 효율적이기 때문입니다.
    passwords_by_account: UnorderedMap<AccountId, Vec<String>>,
}

// `#[near_bindgen]` 블록 내부에 컨트랙트의 공개 함수들을 구현합니다.
#[near_bindgen]
impl Contract {
    // 컨트랙트 초기화 함수입니다. `#[init]` 매크로는 이 함수가 배포 시 단 한 번만 호출될 수 있도록 보장합니다.
    // 배포 시 `owner_id`를 인자로 받아 컨트랙트 상태에 저장합니다.
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            // UnorderedMap을 새로 생성하여 초기화합니다.
            // `b"m".to_vec()`는 이 UnorderedMap을 위한 고유한 스토리지 접두사(prefix)입니다.
            // 만약 나중에 다른 UnorderedMap을 추가하게 될 경우, 이 접두사를 통해 데이터가 섞이는 것을 방지할 수 있습니다.
            passwords_by_account: UnorderedMap::new(b"m".to_vec()),
        }
    }

    // 특정 계정에 새로운 비밀번호 해시를 추가하는 함수입니다.
    // `&mut self`는 이 함수가 컨트랙트의 상태(state)를 변경함을 의미합니다. (쓰기 함수)
    // 이제 이 함수는 오직 `owner_id`만 호출할 수 있습니다.
    pub fn add_password(&mut self, account_id: AccountId, password_hash: String) {
        // 함수를 호출한 계정이 컨트랙트 소유자인지 확인합니다.
        // `env::predecessor_account_id()`는 이 함수를 직접 호출한 계정의 ID를 반환합니다.
        // 만약 소유자가 아니라면, 에러 메시지와 함께 트랜잭션을 즉시 중단(panic)시킵니다.
        assert_eq!(env::predecessor_account_id(), self.owner_id, "This function can only be called by the contract owner.");

        // 주어진 `account_id`에 해당하는 기존 비밀번호 목록을 가져옵니다.
        // `get()`은 Option 타입을 반환하는데, `unwrap_or_default()`를 사용하면 편리하게 처리할 수 있습니다.
        // - 계정이 이미 존재하면: `Some(Vec<String>)`을 풀어 `Vec<String>`을 반환합니다.
        // - 계정이 존재하지 않으면(`get`이 None을 반환): 새로운 빈 `Vec<String>`을 생성하여 반환합니다.
        let mut passwords = self.passwords_by_account.get(&account_id).unwrap_or_default();

        // 가져온 목록에 새로운 비밀번호 해시를 추가합니다.
        passwords.push(password_hash);

        // 변경된 목록을 다시 스토리지에 저장합니다.
        // `insert`는 해당 `account_id`에 대한 값을 덮어쓰거나, 없는 경우 새로 생성합니다.
        self.passwords_by_account.insert(&account_id, &passwords);
    }

    // 특정 계정에 저장된 모든 비밀번호 해시 목록을 조회하는 함수입니다.
    // `&self`는 이 함수가 상태를 변경하지 않는 읽기 전용(view) 함수임을 의미합니다.
    // 읽기 전용 함수는 가스 비용 없이 무료로 호출할 수 있습니다.
    pub fn get_passwords(&self, account_id: AccountId) -> Vec<String> {
        // `add_password`와 마찬가지로 `get`을 통해 목록을 조회합니다.
        // `unwrap_or_default()`는 여기서도 유용하게 사용됩니다.
        // 만약 조회하려는 계정이 존재하지 않으면, 빈 벡터(`[]`)가 반환되어 안전하고 예측 가능한 결과를 보장합니다.
        self.passwords_by_account.get(&account_id).unwrap_or_default()
    }
}

// `#[cfg(test)]`는 이 모듈이 `cargo test` 명령 실행 시에만 컴파일되도록 합니다.
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // 테스트를 위한 가상 블록체인 환경(VMContext)을 생성하는 헬퍼 함수입니다.
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        // `predecessor_account_id`는 테스트에서 컨트랙트 함수를 "호출하는" 계정을 설정합니다.
        builder.predecessor_account_id(predecessor_account_id);
        builder
    }

    // `new` 초기화 함수가 올바르게 동작하는지 테스트합니다.
    #[test]
    fn test_new() {
        let owner_id: AccountId = "owner.testnet".to_string().try_into().unwrap();
        let context = get_context(owner_id.clone());
        testing_env!(context.build());

        // 컨트랙트를 초기화합니다.
        let contract = Contract::new(owner_id.clone());

        // 소유자 ID가 올바르게 설정되었는지 확인합니다.
        assert_eq!(contract.owner_id, owner_id);
        // 초기화 직후에는 저장된 데이터가 없어야 함을 확인합니다.
        assert!(contract.passwords_by_account.is_empty());
    }

    // 비밀번호 추가 기능이 올바르게 동작하는지 테스트합니다.
    // (소유자가 호출하는 성공 케이스)
    #[test]
    fn test_add_password() {
        // 소유자 계정을 설정합니다.
        let owner_id: AccountId = "owner.testnet".to_string().try_into().unwrap();
        // 함수 호출자(`predecessor`)를 소유자로 설정하여 테스트 환경을 구성합니다.
        let context = get_context(owner_id.clone());
        testing_env!(context.build());
        // 컨트랙트를 소유자 ID와 함께 초기화합니다.
        let mut contract = Contract::new(owner_id);

        let account_id: AccountId = "user1.testnet".to_string().try_into().unwrap();
        let password_hash1 = "hash123".to_string();
        let password_hash2 = "hash456".to_string();

        // 1. 첫 번째 비밀번호를 추가합니다.
        // 이 호출은 소유자가 했으므로 성공해야 합니다.
        contract.add_password(account_id.clone(), password_hash1.clone());
        let passwords = contract.get_passwords(account_id.clone());
        // 목록의 길이가 1인지 확인합니다.
        assert_eq!(passwords.len(), 1);
        // 저장된 해시가 올바른지 확인합니다.
        assert_eq!(passwords[0], password_hash1);

        // 2. 같은 계정에 두 번째 비밀번호를 추가합니다.
        contract.add_password(account_id.clone(), password_hash2.clone());
        let passwords = contract.get_passwords(account_id.clone());
        // 이제 목록의 길이가 2인지 확인합니다.
        assert_eq!(passwords.len(), 2);
        // 기존 해시와 새로 추가된 해시가 모두 올바르게 저장되었는지 순서대로 확인합니다.
        assert_eq!(passwords[0], password_hash1);
        assert_eq!(passwords[1], password_hash2);
    }

    // 소유자가 아닌 계정이 `add_password`를 호출할 때 실패하는지 테스트합니다.
    #[test]
    #[should_panic(expected = "This function can only be called by the contract owner.")]
    fn test_add_password_panics_if_not_owner() {
        let owner_id: AccountId = "owner.testnet".to_string().try_into().unwrap();
        let mut contract = Contract::new(owner_id);

        // 함수 호출자를 소유자가 아닌 다른 계정으로 설정합니다.
        let non_owner_id: AccountId = "imposter.testnet".to_string().try_into().unwrap();
        let context = get_context(non_owner_id);
        testing_env!(context.build());

        // 이 호출은 `assert_eq!`에 의해 panic을 발생시켜야 합니다.
        let account_id: AccountId = "user1.testnet".to_string().try_into().unwrap();
        let password_hash = "hash123".to_string();
        contract.add_password(account_id, password_hash);
    }

    // 존재하지 않는 계정을 조회할 때의 동작을 테스트합니다.
    #[test]
    fn test_get_passwords_non_existent_account() {
        let owner_id: AccountId = "owner.testnet".to_string().try_into().unwrap();
        let context = get_context(owner_id.clone());
        testing_env!(context.build());
        let contract = Contract::new(owner_id);

        let account_id: AccountId = "nonexistent.testnet".to_string().try_into().unwrap();

        // 아직 아무 데이터도 추가하지 않은 계정을 조회합니다.
        let passwords = contract.get_passwords(account_id);
        // 결과가 빈 벡터(`[]`)여야 함을 확인합니다.
        assert!(passwords.is_empty());
    }
}