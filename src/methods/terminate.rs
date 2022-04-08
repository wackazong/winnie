use near_primitives_core::types::AccountId;
use near_sdk::{env, require, Promise};

use crate::errors::Errors;
use crate::utils::convert_old_account_id;
use crate::*;

#[near_bindgen]
impl Bet {
    pub fn terminate(&mut self) -> Promise {
        require!(
            env::block_timestamp() > self.claim_until,
            Errors::CANNOT_TERMINATE_YET.to_string()
        );
        let predecessor: AccountId = convert_old_account_id(env::predecessor_account_id());
        require!(
            self.owner == predecessor,
            Errors::ACCOUNT_NOT_AUTHORIZED.to_string()
        );
        Promise::new(env::current_account_id()).delete_account(env::predecessor_account_id())
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::testing_env;

    use crate::methods::testdata::BetTestData;

    #[test]
    fn success() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_11.clone();
        testdata.context.block_timestamp = testdata.claim_until + 1;
        testdata.context.account_balance = testdata.wager.0;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_voted_no_draw_1();

        // Act
        bet.terminate();
    }

    #[test]
    #[should_panic(expected = "CANNOT_TERMINATE_YET")]
    fn claiming_closed() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_11.clone();
        testdata.context.block_timestamp = testdata.claim_until - 1;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_voted_draw_1();

        // Act
        bet.terminate();
    }

    #[test]
    #[should_panic(expected = "ACCOUNT_NOT_AUTHORIZED")]
    fn not_authorized() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.block_timestamp = testdata.claim_until + 1;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_voted_draw_1();

        // Act
        bet.terminate();
    }
}
