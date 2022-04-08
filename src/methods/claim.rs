use near_primitives_core::types::AccountId;
use near_sdk::{env, require, Promise};
use std::vec;

use crate::errors::Errors;
use crate::models::{BettorStatus, Fact};
use crate::utils::{convert_old_account_id, vector_subset};
use crate::*;

#[near_bindgen]
impl Bet {
    pub fn claim(&mut self) -> Option<Promise> {
        require!(
            env::block_timestamp() > self.vote_until,
            Errors::CANNOT_CLAIM_YET.to_string()
        );
        require!(
            env::block_timestamp() < self.claim_until,
            Errors::CLAIMING_CLOSED.to_string()
        );
        let predecessor: AccountId = convert_old_account_id(env::predecessor_account_id());
        let mut bettor = self
            .bettors
            .get(&predecessor)
            .expect(&Errors::ACCOUNT_NOT_AUTHORIZED.to_string());
        require!(
            matches!(bettor.status, BettorStatus::PlacedBet { .. })
                || matches!(bettor.status, BettorStatus::Voted { .. }),
            Errors::ACCOUNT_DID_NOT_BET.to_string()
        );
        // calculate result if necessary
        if self.fact == Fact::Unset {
            let mut winner_list: Vec<AccountId> = vec![];
            let mut good_loosers: Vec<AccountId> = vec![];
            for hypothesis in self.outcomes.keys() {
                let mut pro: u8 = 0;
                let mut contra: u8 = 0;
                winner_list = vec![];
                good_loosers = vec![];
                for key in self.bettors.keys() {
                    match self.bettors.get(&key).unwrap().status {
                        BettorStatus::PlacedBet { outcome } if outcome == hypothesis => {
                            winner_list.push(key);
                        }
                        BettorStatus::Voted { vote, outcome }
                            if vote == hypothesis && outcome == hypothesis =>
                        {
                            winner_list.push(key);
                        }
                        BettorStatus::Voted { vote, outcome }
                            if vote == hypothesis && outcome != hypothesis =>
                        {
                            pro += 1;
                            good_loosers.push(key);
                        }
                        BettorStatus::Voted { vote, outcome }
                            if vote != hypothesis && outcome == hypothesis =>
                        {
                            contra += 1;
                            winner_list.push(key);
                        }
                        BettorStatus::Voted { vote, outcome }
                            if vote != hypothesis && outcome != hypothesis =>
                        {
                            contra += 1;
                        }
                        _ => {}
                    }
                }
                // if the majority of the loosers voted for this hypothesis then accept it as fact
                if pro > contra {
                    self.fact = Fact::Set {
                        outcome: hypothesis,
                    };
                    break;
                }
            }
            // check if we could find a fact. If not, it's a draw
            if self.fact == Fact::Unset {
                self.fact = Fact::Draw;
            } else {
                // make the winner list and include the lucky loosers
                let mut lucky_looser_count: u8 = good_loosers.len() as u8 / 10;
                if lucky_looser_count == 0 {
                    lucky_looser_count = 1
                };
                winner_list.append(&mut vector_subset(good_loosers, lucky_looser_count));
                self.winners = winner_list;
            }
        }
        let mut amount: u128 = 0;
        if let Fact::Set { .. } = self.fact {
            // We have a fact
            // 5% goes to the contract owner (minus gas fees)
            // 95% is distributed among the winners and the lucky loosers. The number of
            // bettors is divided by the number of outcomes to get the the number
            // of lucky loosers. These are then chosen from the loosers who voted.
            if self.winners.contains(&predecessor) {
                amount = (self.total_amount * 95 / self.winners.len() as u128) / 100;
            }
        } else {
            // It's a draw
            // 50% goes to the contract owner(minus gas fees)
            // 50% is distributed among all bettors t
            amount = (self.total_amount * 50 / self.placed_bets as u128) / 100;
        }
        // transfer outcome and vote from old status
        bettor.status = BettorStatus::Claimed {
            outcome: match bettor.status {
                BettorStatus::Voted { outcome: ref o, .. } => o.clone(),
                BettorStatus::PlacedBet { outcome: ref o } => o.clone(),
                _ => env::panic_str(&Errors::INTERNAL_ERROR.to_string()),
            },
            vote: match bettor.status {
                BettorStatus::Voted { vote: ref v, .. } => Some(v.clone()),
                _ => None,
            },
        };
        bettor.claimed = amount;
        self.bettors.insert(&predecessor, &bettor);
        if amount == 0 {
            return None;
        }
        Some(Promise::new(env::predecessor_account_id()).transfer(amount))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::testing_env;

    use crate::methods::testdata::BetTestData;

    #[test]
    fn success_no_draw_variant_1_account_2() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_02.clone();
        testdata.context.block_timestamp = testdata.vote_until + 1;
        testdata.context.account_balance = testdata.wager.0 * 9;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_voted_no_draw_1();

        // Act
        bet.claim();

        // Assert
        assert!(matches!(
            bet.bettors.get(&testdata.account_02).unwrap().status,
            BettorStatus::Claimed {outcome: o, vote: v} if o == testdata.result_01 && v == None
        ));
        assert!(matches!(
            bet.fact,
            Fact::Set {
                outcome
            } if outcome =="1".to_string()
        ));
        assert_eq!(
            bet.winners,
            [
                testdata.account_02.clone(),
                testdata.account_04.clone(),
                testdata.account_06.clone(),
                testdata.account_07.clone(),
                testdata.account_10.clone(),
                testdata.account_09.clone()
            ]
        );
        assert_eq!(env::account_balance(), 7_575_000_000_000_000_000_000_000);
        assert_eq!(
            bet.bettors.get(&testdata.account_02).unwrap().claimed,
            1_425_000_000_000_000_000_000_000
        );
    }

    #[test]
    fn success_draw_variant_1_account_10() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_10.clone();
        testdata.context.block_timestamp = testdata.vote_until + 1;
        testdata.context.account_balance = testdata.wager.0 * 9;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_voted_draw_1();

        // Act
        bet.claim();

        // Assert
        assert!(matches!(
            bet.bettors.get(&testdata.account_10).unwrap().status,
            BettorStatus::Claimed {outcome: o, vote: v} if o == testdata.result_02 && v == Some(testdata.result_02)
        ));
        assert!(matches!(bet.fact, Fact::Draw));
        assert_eq!(bet.winners, []);
        assert_eq!(env::account_balance(), 8_437_500_000_000_000_000_000_000)
    }

    #[test]
    #[should_panic(expected = "ACCOUNT_DID_NOT_BET")]
    fn account_did_not_bet() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.block_timestamp = testdata.vote_until + 1;
        testdata.context.account_balance = testdata.wager.0 * 9;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_voted_draw_1();

        // Act
        bet.claim();
    }

    #[test]
    #[should_panic(expected = "CANNOT_CLAIM_YET")]
    fn claiming_not_open_yet() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_01.clone();
        testdata.context.block_timestamp = testdata.vote_until - 1;
        testdata.context.account_balance = testdata.wager.0 * 9;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_voted_draw_1();

        // Act
        bet.claim();
    }

    #[test]
    #[should_panic(expected = "CLAIMING_CLOSED")]
    fn claiming_closed() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_05.clone();
        testdata.context.block_timestamp = testdata.claim_until + 1;
        testdata.context.account_balance = testdata.wager.0 * 9;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_voted_draw_1();

        // Act
        bet.claim();
    }

    #[test]
    #[should_panic(expected = "ACCOUNT_NOT_AUTHORIZED")]
    fn not_authorized() {
        // Arrange
        let mut testdata = BetTestData::default();
        testdata.context.predecessor_account_id = testdata.account_11.clone();
        testdata.context.block_timestamp = testdata.vote_until + 1;
        testdata.context.account_balance = testdata.wager.0 * 9;
        testing_env!(testdata.context.clone());
        let mut bet = testdata.bet_voted_draw_1();

        // Act
        bet.claim();
    }
}
