use near_primitives_core::types::AccountId;
use near_rng::Rng;
use near_sdk::env;
use near_sdk::AccountId as OldAccountId;
use std::str::FromStr;

const INVALID_OLD_ACCOUNT_ID: &str = "INVALID_OLD_ACCOUNT_ID";

#[macro_export]
macro_rules! near {
    ($a:literal) => {
        $a * 1_000_000_000_000_000_000_000_000
    };
}

pub fn convert_old_account_id(old: OldAccountId) -> AccountId {
    AccountId::from_str(&old.to_string()).unwrap_or_else(|_| env::panic_str(INVALID_OLD_ACCOUNT_ID))
}

pub fn convert_new_account_id(new: AccountId) -> OldAccountId {
    OldAccountId::from_str(&new.to_string())
        .unwrap_or_else(|_| env::panic_str(INVALID_OLD_ACCOUNT_ID))
}

pub fn vector_subset<T>(mut items: Vec<T>, count: u8) -> Vec<T> {
    if items.len() as u8 <= count {
        return items;
    }
    let mut rng = Rng::new(&env::random_seed());
    let mut i: u8;
    while items.len() as u8 > count {
        i = rng.rand_range_u32(0, items.len() as u32) as u8;
        items.remove(i as usize);
    }
    items
}

pub fn near_amount(value: u128) -> f64 {
    let y: u128 = 1_000_000_000_000_000_000_000_000;
    format!("{0:.2}", value as f64 / y as f64)
        .parse::<f64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};

    fn get_context() -> VMContext {
        VMContext {
            current_account_id: AccountId::from_str("contract.testnet").unwrap(),
            signer_account_id: AccountId::from_str("alice.testnet").unwrap(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: AccountId::from_str("alice.testnet").unwrap(),
            input: vec![],
            block_index: 0,
            block_timestamp: 1,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            view_config: None,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_near_macro() {
        // Act
        let i: u128 = near!(1);

        // Assert
        assert_eq!(i, 1_000_000_000_000_000_000_000_000);
    }

    #[test]
    fn test_vector_subset() {
        // Arrange
        let source: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let context = get_context();
        testing_env!(context);

        // Act
        let target: Vec<u8> = vector_subset(source, 5);

        // Assert
        assert_eq!(target.len(), 5);
        assert_eq!(target, vec![3, 5, 6, 12, 14])
    }
}
