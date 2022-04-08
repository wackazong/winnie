pub mod claim;
pub mod end_phase;
pub mod new;
pub mod place_bet;
pub mod terminate;
pub mod view;
pub mod vote_on_outcome;

#[cfg(test)]
mod testdata {
    use near_primitives_core::types::AccountId;
    use near_sdk::VMContext;
    use near_sdk::{collections::UnorderedMap, json_types::U128};
    use std::{collections::HashMap, str::FromStr, vec};

    use crate::models::{Bettor, BettorStatus, Fact};
    use crate::StorageKeys;
    use crate::{models::Outcome, Bet, MINIMUM_WAGER};

    pub struct BetTestData {
        pub description: String,
        pub account_01: AccountId,
        pub account_02: AccountId,
        pub account_03: AccountId,
        pub account_04: AccountId,
        pub account_05: AccountId,
        pub account_06: AccountId,
        pub account_07: AccountId,
        pub account_08: AccountId,
        pub account_09: AccountId,
        pub account_10: AccountId,
        pub account_11: AccountId,
        pub outcomes: HashMap<String, Outcome>,
        pub result_01: String,
        pub result_02: String,
        pub result_03: String,
        pub wager: U128,
        pub wager_too_small: U128,
        pub start_time: u64,
        pub bet_until: u64,
        pub vote_until: u64,
        pub claim_until: u64,
        pub context: VMContext,
    }

    impl Default for BetTestData {
        fn default() -> Self {
            let mut outcomes: HashMap<String, Outcome> = HashMap::new();
            let result_01 = "1".to_string();
            let result_02 = "2".to_string();
            let result_03 = "3".to_string();
            outcomes.insert(result_01.clone(), Outcome::new("Description 1".to_string()));
            outcomes.insert(result_02.clone(), Outcome::new("Description 2".to_string()));
            outcomes.insert(result_03.clone(), Outcome::new("Description 3".to_string()));
            let start_time: u64 = 1000;
            Self {
                description: "Bet description".to_string(),
                account_01: AccountId::from_str("1.near").unwrap(),
                account_02: AccountId::from_str("2.near").unwrap(),
                account_03: AccountId::from_str("3.near").unwrap(),
                account_04: AccountId::from_str("4.near").unwrap(),
                account_05: AccountId::from_str("5.near").unwrap(),
                account_06: AccountId::from_str("6.near").unwrap(),
                account_07: AccountId::from_str("7.near").unwrap(),
                account_08: AccountId::from_str("8.near").unwrap(),
                account_09: AccountId::from_str("9.near").unwrap(),
                account_10: AccountId::from_str("10.near").unwrap(),
                account_11: AccountId::from_str("11.near").unwrap(),
                outcomes,
                result_01,
                result_02,
                result_03,
                wager: U128::from(MINIMUM_WAGER),
                wager_too_small: U128::from(MINIMUM_WAGER - 1),
                start_time: start_time,
                bet_until: 2000,
                vote_until: 3000,
                claim_until: 4000,
                context: VMContext {
                    current_account_id: AccountId::from_str("contract.near").unwrap(),
                    signer_account_id: AccountId::from_str("anybody.near").unwrap(),
                    signer_account_pk: vec![0, 1, 2],
                    predecessor_account_id: AccountId::from_str("anybody.near").unwrap(),
                    input: vec![],
                    block_index: 0,
                    block_timestamp: start_time,
                    account_balance: 0,
                    account_locked_balance: 0,
                    storage_usage: 0,
                    attached_deposit: 0,
                    prepaid_gas: 10u64.pow(18),
                    random_seed: vec![0, 1, 2],
                    view_config: None,
                    output_data_receivers: vec![],
                    epoch_height: 19,
                },
            }
        }
    }
    const INVIT: &'static str = "Invited";
    const PLACE: &'static str = "Placed";
    const VOTED: &'static str = "Voted";
    const CLAIM: &'static str = "Claimed";

    impl BetTestData {
        fn make_outcomes(&self) -> UnorderedMap<String, Outcome> {
            let mut outcomes: UnorderedMap<String, Outcome> =
                UnorderedMap::new(StorageKeys::Outcomes);
            outcomes.insert(&self.result_01, self.outcomes.get(&self.result_01).unwrap());
            outcomes.insert(&self.result_02, self.outcomes.get(&self.result_02).unwrap());
            outcomes.insert(&self.result_03, self.outcomes.get(&self.result_03).unwrap());
            outcomes
        }
        fn make_bettors(&self, bets: [(&str, u8, u8); 10]) -> UnorderedMap<AccountId, Bettor> {
            let mut bettors: UnorderedMap<AccountId, Bettor> =
                UnorderedMap::new(StorageKeys::Bettors);
            let mut count: u8 = 0;
            for bet in bets {
                let mut status: BettorStatus = BettorStatus::Invited;
                count += 1;
                match bet.0 {
                    VOTED => {
                        status = BettorStatus::Voted {
                            outcome: bet.1.to_string(),
                            vote: bet.2.to_string(),
                        };
                    }
                    PLACE => {
                        status = BettorStatus::PlacedBet {
                            outcome: bet.1.to_string(),
                        };
                    }
                    CLAIM => {
                        status = BettorStatus::Claimed {
                            outcome: bet.1.to_string(),
                            vote: Some(bet.2.to_string()),
                        };
                    }
                    _ => {}
                }
                bettors.insert(
                    &AccountId::from_str(&format!("{}.near", count)).unwrap(),
                    &Bettor::new(status),
                );
            }
            bettors
        }

        pub fn bet_new(&self) -> Bet {
            let bets: [(&str, u8, u8); 10] = [
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
            ];
            Bet {
                description: self.description.clone(),
                wager: self.wager.0,
                owner: self.account_11.clone(),
                fact: Fact::Unset,
                winners: vec![],
                bettors: self.make_bettors(bets),
                outcomes: self.make_outcomes(),
                bet_until: self.bet_until,
                vote_until: self.vote_until,
                claim_until: self.claim_until,
                placed_bets: 0,
                outcome_count: 3,
                total_amount: 0,
            }
        }

        pub fn bet_placed_bet(&self) -> Bet {
            let bets: [(&str, u8, u8); 10] = [
                (PLACE, 1, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (INVIT, 0, 0),
            ];
            Bet {
                description: self.description.clone(),
                wager: self.wager.0,
                owner: self.account_11.clone(),
                fact: Fact::Unset,
                winners: vec![],
                bettors: self.make_bettors(bets),
                outcomes: self.make_outcomes(),
                bet_until: self.bet_until,
                vote_until: self.vote_until,
                claim_until: self.claim_until,
                placed_bets: 1,
                outcome_count: 3,
                total_amount: self.wager.0,
            }
        }

        pub fn bet_voted_no_draw_1(&self) -> Bet {
            let bets: [(&str, u8, u8); 10] = [
                (INVIT, 0, 0),
                (PLACE, 1, 0),
                (PLACE, 2, 0),
                (VOTED, 1, 2),
                (VOTED, 2, 1),
                (VOTED, 1, 3),
                (VOTED, 1, 1),
                (VOTED, 2, 1),
                (VOTED, 2, 1),
                (VOTED, 1, 1),
            ];
            Bet {
                description: self.description.clone(),
                wager: self.wager.0,
                owner: self.account_11.clone(),
                fact: Fact::Unset,
                winners: vec![],
                bettors: self.make_bettors(bets),
                outcomes: self.make_outcomes(),
                bet_until: self.bet_until,
                vote_until: self.vote_until,
                claim_until: self.claim_until,
                placed_bets: 9,
                outcome_count: 3,
                total_amount: self.wager.0 * 9,
            }
        }

        pub fn bet_voted_draw_1(&self) -> Bet {
            let bets: [(&str, u8, u8); 10] = [
                (INVIT, 0, 0),
                (INVIT, 0, 0),
                (PLACE, 1, 0),
                (PLACE, 2, 0),
                (VOTED, 1, 1),
                (VOTED, 1, 1),
                (VOTED, 1, 1),
                (VOTED, 2, 2),
                (VOTED, 2, 2),
                (VOTED, 2, 2),
            ];
            Bet {
                description: self.description.clone(),
                wager: self.wager.0,
                owner: self.account_11.clone(),
                fact: Fact::Unset,
                winners: vec![],
                bettors: self.make_bettors(bets),
                outcomes: self.make_outcomes(),
                bet_until: self.bet_until,
                vote_until: self.vote_until,
                claim_until: self.claim_until,
                placed_bets: 8,
                outcome_count: 3,
                total_amount: self.wager.0 * 9,
            }
        }
    }
}
