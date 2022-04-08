use std::fmt;

use near_primitives_core::types::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
#[derive(PartialEq)]
pub enum Phase {
    Bet,
    Vote,
    Claim,
    Terminate,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
#[derive(PartialEq)]
pub enum BettorStatus {
    Invited,
    PlacedBet {
        outcome: String,
    },
    Voted {
        outcome: String,
        vote: String,
    },
    Claimed {
        outcome: String,
        vote: Option<String>,
    },
}

impl fmt::Display for BettorStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
#[derive(PartialEq)]
pub enum Fact {
    Unset,
    Set { outcome: String },
    Draw,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Status {
    pub owner: AccountId,
    pub current_phase: String,
    pub remaining_seconds: u32,
    pub outcomes: Vec<OutcomeData>,
    pub fact: Fact,
    pub bettors: Vec<BettorData>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct BettorData {
    pub account_id: String,
    pub claimed_near: f64,
    pub status: String,
    pub outcome: Option<String>,
    pub vote: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Bettor {
    pub status: BettorStatus,
    pub claimed: u128,
}

impl Bettor {
    pub fn new(status: BettorStatus) -> Self {
        Self { status, claimed: 0 }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OutcomeData {
    pub id: String,
    pub description: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Outcome {
    pub description: String,
}

impl Outcome {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}
