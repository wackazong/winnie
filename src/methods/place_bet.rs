use near_primitives_core::types::AccountId;
use near_sdk::{env, near_bindgen, require};

use crate::errors::Errors;
use crate::models::BettorStatus;
use crate::utils::convert_old_account_id;
use crate::*;

#[near_bindgen]
impl Bet {
    #[payable]
    pub fn place_bet(&mut self, outcome: String) {
        require!(
            env::attached_deposit() == self.wager,
            Errors::WAGER_AMOUNT_INCORRECT.to_string()
        );
        require!(
            env::block_timestamp() < self.bet_until,
            Errors::BETTING_CLOSED.to_string()
        );
        require!(
            self.outcomes.get(&outcome).is_some(),
            Errors::OUTCOME_NOT_FOUND.to_string()
        );
        let predecessor: AccountId = convert_old_account_id(env::predecessor_account_id());
        let mut bettor = self
            .bettors
            .get(&predecessor)
            .expect(&Errors::ACCOUNT_NOT_AUTHORIZED.to_string());
        require!(
            bettor.status == BettorStatus::Invited,
            Errors::ACCOUNT_ALREADY_PLACED_BET.to_string()
        );
        bettor.status = BettorStatus::PlacedBet { outcome };
        self.total_amount += self.wager;
        self.bettors.insert(&predecessor, &bettor);
        self.placed_bets += 1;
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
        testdata.context.attached_deposit = testdata.wager.0;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_new();

        // Act
        bet.place_bet(testdata.result_01.clone());

        // Assert
        assert!(matches!(
            bet.bettors.get(&testdata.account_01).unwrap().status,
            BettorStatus::PlacedBet { outcome: o } if o == testdata.result_01
        ));
        assert_eq!(env::account_balance(), testdata.wager.0);
        assert_eq!(bet.total_amount, testdata.wager.0);
        assert_eq!(bet.placed_bets, 1)
    }

    #[test]
    #[should_panic(expected = "ACCOUNT_ALREADY_PLACED_BET")]
    fn can_not_bet_twice() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.attached_deposit = testdata.wager.0;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_new();

        // Act
        bet.place_bet(testdata.result_01.clone());
        bet.place_bet(testdata.result_01.clone());
    }

    #[test]
    #[should_panic(expected = "WAGER_AMOUNT_INCORRECT")]
    fn wager_amount_incorrect() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testing_env!(testdata.context.clone());

        // Act
        testdata.bet_new().place_bet(testdata.result_01.clone());
    }

    #[test]
    #[should_panic(expected = "ACCOUNT_NOT_AUTHORIZED")]
    fn not_authorized() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_11.clone();
        testdata.context.attached_deposit = testdata.wager.0;
        testing_env!(testdata.context.clone());

        // Act
        testdata.bet_new().place_bet(testdata.result_01.clone());
    }

    #[test]
    #[should_panic(expected = "OUTCOME_NOT_FOUND")]
    fn outcome_not_found() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.attached_deposit = testdata.wager.0;
        testing_env!(testdata.context.clone());

        // Act
        testdata.bet_new().place_bet("not a result".to_string());
    }

    #[test]
    #[should_panic(expected = "BETTING_CLOSED")]
    fn betting_closed() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.attached_deposit = testdata.wager.0;
        testdata.context.block_timestamp = 2001;
        testing_env!(testdata.context.clone());

        // Act
        testdata.bet_new().place_bet(testdata.result_01.clone());
    }
}
