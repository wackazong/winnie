use near_primitives_core::types::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, Balance, BorshStorageKey, PanicOnDefault};

pub mod errors;
pub mod methods;
pub mod models;
pub mod utils;

use models::{Bettor, Fact, Outcome, Phase};

const MINIMUM_BET_NANOSECONDS: u64 = 100;
const MINIMUM_VOTE_NANOSECONDS: u64 = 100;
const MINIMUM_CLAIM_NANOSECONDS: u64 = 100;
const MINIMUM_WAGER: Balance = near!(1);
const MAXIMUM_BETTORS: u8 = 100;
const MAXIMUM_OUTCOMES: u8 = 50;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Bettors,
    Outcomes,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Bet {
    owner: AccountId,
    description: String,
    wager: Balance,
    fact: Fact,
    winners: Vec<AccountId>,
    bettors: UnorderedMap<AccountId, Bettor>,
    outcomes: UnorderedMap<String, Outcome>,
    bet_until: u64,
    vote_until: u64,
    claim_until: u64,
    placed_bets: u8,
    outcome_count: u8,
    total_amount: u128,
}

impl Bet {
    fn phase(&self) -> Phase {
        if env::block_timestamp() < self.bet_until {
            Phase::Bet
        } else if env::block_timestamp() < self.vote_until {
            Phase::Vote
        } else if env::block_timestamp() < self.claim_until {
            Phase::Claim
        } else {
            Phase::Terminate
        }
    }

    fn seconds_remaining(&self) -> u32 {
        let until: u64;
        match self.phase() {
            Phase::Bet => {
                until = self.bet_until;
            }
            Phase::Vote => {
                until = self.vote_until;
            }
            Phase::Claim => {
                until = self.claim_until;
            }
            Phase::Terminate => {
                until = 0;
            }
        }
        if until == 0 {
            return 0 as u32;
        }
        ((until - env::block_timestamp()) / 1000000000) as u32
    }
}
