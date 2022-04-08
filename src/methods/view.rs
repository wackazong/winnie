use crate::models::{BettorData, BettorStatus, OutcomeData, Status};
use crate::utils::near_amount;
use crate::*;
use near_sdk::near_bindgen;

#[near_bindgen]
impl Bet {
    pub fn view(&self) -> Status {
        let mut outcomes: Vec<OutcomeData> = vec![];
        for key in self.outcomes.keys() {
            let outcome: Outcome = self.outcomes.get(&key).unwrap();
            outcomes.push(OutcomeData {
                id: key.clone(),
                description: outcome.description.clone(),
            })
        }
        let mut bettors: Vec<BettorData> = vec![];
        for key in self.bettors.keys() {
            let bettor: Bettor = self.bettors.get(&key).unwrap();
            bettors.push(BettorData {
                account_id: key.to_string(),
                status: bettor.status.to_string().split(" ").collect::<Vec<&str>>()[0].to_string(),
                claimed_near: near_amount(bettor.claimed),
                outcome: match &bettor.status {
                    BettorStatus::PlacedBet { outcome: o } => Some(o.clone()),
                    BettorStatus::Voted { outcome: o, .. } => Some(o.clone()),
                    BettorStatus::Claimed { outcome: o, .. } => Some(o.clone()),
                    _ => None,
                },
                vote: match &bettor.status {
                    BettorStatus::Voted { vote: v, .. } => Some(v.clone()),
                    BettorStatus::Claimed { vote: v, .. } => v.clone(),
                    _ => None,
                },
            });
        }
        Status {
            owner: self.owner.clone(),
            current_phase: self.phase().to_string(),
            remaining_seconds: self.seconds_remaining(),
            outcomes,
            fact: self.fact.clone(),
            bettors,
        }
    }
}
