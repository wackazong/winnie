use near_primitives_core::types::AccountId;
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, require};
use std::collections::HashMap;

use crate::errors::Errors;
use crate::models::{Bettor, BettorStatus, Outcome};
use crate::*;

#[near_bindgen]
impl Bet {
    #[init]
    pub fn new(
        owner: AccountId,
        description: String,
        bettors: Vec<AccountId>,
        wager: U128,
        outcomes: HashMap<String, Outcome>,
        bet_until: u64,
        vote_until: u64,
        claim_until: u64,
    ) -> Self {
        require!(
            env::block_timestamp() < bet_until
                && bet_until < vote_until
                && vote_until < claim_until,
            &Errors::TIMESTAMPS_INCONSISTENT.to_string()
        );
        require!(
            env::block_timestamp() + MINIMUM_BET_NANOSECONDS < bet_until,
            &Errors::BET_DURATION_TOO_SHORT.to_string()
        );
        require!(
            bet_until + MINIMUM_VOTE_NANOSECONDS < vote_until,
            &Errors::VOTE_DURATION_TOO_SHORT.to_string()
        );
        require!(
            vote_until + MINIMUM_CLAIM_NANOSECONDS < claim_until,
            &Errors::CLAIM_DURATION_TOO_SHORT.to_string()
        );
        require!(
            outcomes.len() > 1,
            &Errors::MORE_THAN_ONE_OUTCOME_REQUIRED.to_string()
        );
        require!(
            bettors.len() > 1,
            &Errors::MORE_THAN_ONE_BETTOR_REQUIRED.to_string()
        );
        require!(
            outcomes.len() <= MAXIMUM_OUTCOMES as usize,
            &Errors::TOO_MANY_OUTCOMES.to_string()
        );
        require!(
            bettors.len() <= MAXIMUM_BETTORS as usize,
            &Errors::TOO_MANY_BETTORS.to_string()
        );
        require!(
            wager.0 >= MINIMUM_WAGER,
            &Errors::MINIMUM_WAGER_NOT_REACHED.to_string()
        );
        let mut bettors_map: UnorderedMap<AccountId, Bettor> =
            UnorderedMap::new(StorageKeys::Bettors);
        for i in &bettors {
            bettors_map.insert(i, &Bettor::new(BettorStatus::Invited));
        }
        let mut outcomes_map: UnorderedMap<String, Outcome> =
            UnorderedMap::new(StorageKeys::Outcomes);
        for i in outcomes.keys() {
            let v: Outcome = Outcome::new(outcomes.get(i).unwrap().description.clone());
            outcomes_map.insert(i, &v);
        }
        Self {
            description,
            wager: wager.0,
            owner,
            fact: Fact::Unset,
            winners: vec![],
            bettors: bettors_map,
            outcomes: outcomes_map,
            bet_until,
            vote_until,
            claim_until,
            placed_bets: 0,
            outcome_count: outcomes.len() as u8,
            total_amount: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use near_sdk::testing_env;

    use crate::methods::testdata::BetTestData;

    #[test]
    fn success() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());

        // Act
        let bet = Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager.clone(),
            testdata.outcomes.clone(),
            testdata.bet_until,
            testdata.vote_until,
            testdata.claim_until,
        );

        // Assert
        assert_eq!(testdata.description, bet.description);
        assert_eq!(testdata.account_11, bet.owner);
        assert_eq!(testdata.wager.0, bet.wager);
        assert!(matches!(bet.fact, Fact::Unset));
        assert!(bet.winners.is_empty());
        assert!(matches!(
            bet.bettors.get(&testdata.account_01).unwrap().status,
            BettorStatus::Invited
        ));
        assert!(matches!(
            bet.bettors.get(&testdata.account_01).unwrap().status,
            BettorStatus::Invited
        ));
        assert_eq!(
            testdata
                .outcomes
                .get(&testdata.result_01)
                .unwrap()
                .description,
            bet.outcomes.get(&testdata.result_01).unwrap().description
        );
        assert_eq!(
            testdata
                .outcomes
                .get(&testdata.result_02)
                .unwrap()
                .description,
            bet.outcomes.get(&testdata.result_02).unwrap().description
        );
        assert_eq!(bet.bet_until, testdata.bet_until);
        assert_eq!(bet.vote_until, testdata.vote_until);
        assert_eq!(bet.claim_until, testdata.claim_until);
        assert_eq!(bet.placed_bets, 0);
        assert_eq!(bet.outcome_count, 3);
        assert_eq!(bet.total_amount, 0);
    }

    #[test]
    #[should_panic(expected = "MORE_THAN_ONE_OUTCOME_REQUIRED")]
    fn not_enough_outcomes() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.outcomes.remove(&testdata.result_01);
        testdata.outcomes.remove(&testdata.result_02);
        testing_env!(testdata.context.clone());

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager.clone(),
            testdata.outcomes,
            testdata.bet_until,
            testdata.vote_until,
            testdata.claim_until,
        );
    }

    #[test]
    #[should_panic(expected = "MORE_THAN_ONE_OUTCOME_REQUIRED")]
    fn no_outcomes() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.outcomes.remove(&testdata.result_01);
        testdata.outcomes.remove(&testdata.result_02);
        testdata.outcomes.remove(&testdata.result_03);
        testing_env!(testdata.context.clone());

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager.clone(),
            testdata.outcomes,
            testdata.bet_until,
            testdata.vote_until,
            testdata.claim_until,
        );
    }

    #[test]
    #[should_panic(expected = "MORE_THAN_ONE_BETTOR_REQUIRED")]
    fn not_enough_bettors() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_02.clone()],
            testdata.wager.clone(),
            testdata.outcomes.clone(),
            testdata.bet_until,
            testdata.vote_until,
            testdata.claim_until,
        );
    }

    #[test]
    #[should_panic(expected = "MINIMUM_WAGER_NOT_REACHED")]
    fn minimum_wager_not_reached() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager_too_small.clone(),
            testdata.outcomes.clone(),
            testdata.bet_until,
            testdata.vote_until,
            testdata.claim_until,
        );
    }

    #[test]
    #[should_panic(expected = "TIMESTAMPS_INCONSISTENT")]
    fn bet_until_in_past() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager.clone(),
            testdata.outcomes.clone(),
            env::block_timestamp() - 1,
            testdata.vote_until,
            testdata.claim_until,
        );
    }

    #[test]
    #[should_panic(expected = "TIMESTAMPS_INCONSISTENT")]
    fn vote_until_before_bet_until() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager.clone(),
            testdata.outcomes.clone(),
            testdata.bet_until,
            testdata.bet_until - 1,
            testdata.claim_until,
        );
    }

    #[test]
    #[should_panic(expected = "TIMESTAMPS_INCONSISTENT")]
    fn claim_until_before_vote_until() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager.clone(),
            testdata.outcomes.clone(),
            testdata.bet_until,
            testdata.vote_until,
            testdata.vote_until - 1,
        );
    }

    #[test]
    #[should_panic(expected = "BET_DURATION_TOO_SHORT")]
    fn bet_duration_too_short() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager.clone(),
            testdata.outcomes.clone(),
            env::block_timestamp() + 1,
            testdata.vote_until,
            testdata.claim_until,
        );
    }

    #[test]
    #[should_panic(expected = "VOTE_DURATION_TOO_SHORT")]
    fn vote_duration_too_short() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager.clone(),
            testdata.outcomes.clone(),
            testdata.bet_until,
            testdata.bet_until + 1,
            testdata.claim_until,
        );
    }

    #[test]
    #[should_panic(expected = "CLAIM_DURATION_TOO_SHORT")]
    fn claim_duration_too_short() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager.clone(),
            testdata.outcomes.clone(),
            testdata.bet_until,
            testdata.vote_until,
            testdata.vote_until + 1,
        );
    }

    #[test]
    #[should_panic(expected = "TOO_MANY_OUTCOMES")]
    fn too_many_outcomes() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());
        let mut outcomes: HashMap<String, Outcome> = HashMap::new();
        for i in 0..MAXIMUM_OUTCOMES + 1 {
            outcomes.insert(i.to_string(), Outcome::new(i.to_string()));
        }

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            vec![testdata.account_01.clone(), testdata.account_02.clone()],
            testdata.wager.clone(),
            outcomes,
            testdata.bet_until,
            testdata.vote_until,
            testdata.claim_until,
        );
    }

    #[test]
    #[should_panic(expected = "TOO_MANY_BETTORS")]
    fn too_many_bettors() {
        // Arrange
        let testdata = BetTestData::default();
        testing_env!(testdata.context.clone());
        let mut bettors: Vec<AccountId> = vec![];
        for i in 0..MAXIMUM_BETTORS + 1 {
            let account_id: String = format!("bob{}.near", i);
            bettors.push(AccountId::from_str(&account_id).unwrap());
        }

        // Act
        Bet::new(
            testdata.account_11.clone(),
            testdata.description.clone(),
            bettors,
            testdata.wager.clone(),
            testdata.outcomes.clone(),
            testdata.bet_until,
            testdata.vote_until,
            testdata.claim_until,
        );
    }
}
