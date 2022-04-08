use near_primitives_core::types::AccountId;
use near_sdk::{env, near_bindgen, require};

use crate::errors::Errors;
use crate::models::BettorStatus;
use crate::utils::convert_old_account_id;
use crate::*;

#[near_bindgen]
impl Bet {
    pub fn vote_on_outcome(&mut self, vote: String) {
        require!(
            env::block_timestamp() < self.vote_until,
            Errors::VOTING_CLOSED.to_string()
        );
        require!(
            env::block_timestamp() > self.bet_until,
            Errors::VOTING_NOT_OPEN_YET.to_string()
        );
        require!(
            self.outcomes.get(&vote).is_some(),
            Errors::OUTCOME_NOT_FOUND.to_string()
        );
        let predecessor: AccountId = convert_old_account_id(env::predecessor_account_id());
        let mut bettor = self
            .bettors
            .get(&predecessor)
            .expect(&Errors::ACCOUNT_NOT_AUTHORIZED.to_string());
        require!(
            !matches!(bettor.status, BettorStatus::Invited),
            Errors::ACCOUNT_DID_NOT_BET.to_string()
        );
        // get outcome value from old BettorStatus
        if let BettorStatus::PlacedBet { outcome: x } = bettor.status {
            bettor.status = BettorStatus::Voted { outcome: x, vote };
        } else {
            env::panic_str(&Errors::ACCOUNT_ALREADY_VOTED.to_string());
        }
        self.bettors.insert(&predecessor, &bettor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::testing_env;

    use crate::methods::testdata::BetTestData;

    #[test]
    fn success() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.block_timestamp = 2001;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_placed_bet();

        // Act
        bet.vote_on_outcome(testdata.result_02.clone());

        // Assert
        assert!(matches!(
            bet.bettors.get(&testdata.account_01).unwrap().status,
            BettorStatus::Voted { outcome: o, vote: v }
                if o == testdata.result_01 && v == testdata.result_02
        ));
    }

    #[test]
    #[should_panic(expected = "VOTING_NOT_OPEN_YET")]
    fn voting_not_open_yet() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.block_timestamp = 1500;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_placed_bet();

        // Act
        bet.vote_on_outcome(testdata.result_02.clone());
    }

    #[test]
    #[should_panic(expected = "VOTING_CLOSED")]
    fn voting_closed() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.block_timestamp = 3500;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_placed_bet();

        // Act
        bet.vote_on_outcome(testdata.result_02.clone());
    }

    #[test]
    #[should_panic(expected = "OUTCOME_NOT_FOUND")]
    fn outcome_not_found() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.block_timestamp = 2500;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_placed_bet();

        // Act
        bet.vote_on_outcome("not a result".to_string());
    }

    #[test]
    #[should_panic(expected = "NOT_AUTHORIZED")]
    fn not_authorized() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_11.clone();
        testdata.context.block_timestamp = 2500;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_placed_bet();

        // Act
        bet.vote_on_outcome(testdata.result_02.clone());
    }

    #[test]
    #[should_panic(expected = "ACCOUNT_DID_NOT_BET")]
    fn account_did_not_bet() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.block_timestamp = 2500;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_new();

        // Act
        bet.vote_on_outcome(testdata.result_02.clone());
    }

    #[test]
    #[should_panic(expected = "ACCOUNT_ALREADY_VOTED")]
    fn already_voted() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.block_timestamp = 2500;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_placed_bet();

        // Act
        bet.vote_on_outcome(testdata.result_02.clone());
        bet.vote_on_outcome(testdata.result_02.clone());
    }
}
