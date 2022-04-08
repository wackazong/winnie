use near_primitives_core::types::AccountId;
use near_sdk::{env, near_bindgen, require};

use crate::errors::Errors;
use crate::models::Phase;
use crate::utils::convert_old_account_id;
use crate::*;

#[near_bindgen]
impl Bet {
    pub fn end_phase(&mut self) {
        let predecessor: AccountId = convert_old_account_id(env::predecessor_account_id());
        require!(
            self.owner == predecessor,
            Errors::ACCOUNT_NOT_AUTHORIZED.to_string()
        );
        match self.phase() {
            Phase::Bet => self.bet_until = env::block_timestamp(),
            Phase::Vote => self.vote_until = env::block_timestamp(),
            Phase::Claim => self.claim_until = env::block_timestamp(),
            Phase::Terminate => env::panic_str(&Errors::BET_ALREADY_IS_IN_LAST_PHASE.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::testing_env;

    use crate::methods::testdata::BetTestData;

    #[test]
    fn end_bet_phase() {
        // Arrange
        let mut testdata = BetTestData::default();
        let current_time: u64 =
            testdata.start_time + (testdata.bet_until - testdata.start_time) / 2;
        testdata.context.block_timestamp = current_time;
        testdata.context.predecessor_account_id = testdata.account_11.clone();
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_new();

        // Act
        bet.end_phase();

        // Assert
        assert_eq!(current_time, bet.bet_until);
        let _phase: Phase = bet.phase();
        assert!(matches!(Phase::Vote, _phase));
    }

    #[test]
    fn end_vote_phase() {
        // Arrange
        let mut testdata = BetTestData::default();
        let current_time: u64 =
            testdata.bet_until + (testdata.vote_until - testdata.bet_until) / 2;
        testdata.context.block_timestamp = current_time;
        testdata.context.predecessor_account_id = testdata.account_11.clone();
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_new();

        // Act
        bet.end_phase();

        // Assert
        assert_eq!(current_time, bet.vote_until);
        let _phase: Phase = bet.phase();
        assert!(matches!(Phase::Claim, _phase));
    }

    #[test]
    fn end_claim_phase() {
        // Arrange
        let mut testdata = BetTestData::default();
        let current_time: u64 =
            testdata.vote_until + (testdata.claim_until - testdata.vote_until) / 2;
        testdata.context.block_timestamp = current_time;
        testdata.context.predecessor_account_id = testdata.account_11.clone();
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_new();

        // Act
        bet.end_phase();

        // Assert
        assert_eq!(current_time, bet.claim_until);
        let _phase: Phase = bet.phase();
        assert!(matches!(Phase::Terminate, _phase));
    }

    #[test]
    #[should_panic(expected = "BET_ALREADY_IS_IN_LAST_PHASE")]
    fn not_cannot_end_last_phase() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.block_timestamp = testdata.claim_until + 1;
        testdata.context.predecessor_account_id = testdata.account_11.clone();
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_new();

        // Act
        bet.end_phase();
    }

    #[test]
    #[should_panic(expected = "NOT_AUTHORIZED")]
    fn not_authorized() {
        // Arrange
        let mut testdata = BetTestData::default();
        let current_time: u64 =
            testdata.start_time + (testdata.bet_until - testdata.start_time) / 2;
        testdata.context.block_timestamp = current_time;
        testdata.context.predecessor_account_id = testdata.account_10.clone();
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_new();

        // Act
        bet.end_phase();
    }
}
