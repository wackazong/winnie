use std::fmt;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Errors {
    // general
    ACCOUNT_NOT_AUTHORIZED,
    INTERNAL_ERROR,

    //new
    MORE_THAN_ONE_OUTCOME_REQUIRED,
    TOO_MANY_OUTCOMES,
    MORE_THAN_ONE_BETTOR_REQUIRED,
    TOO_MANY_BETTORS,
    MINIMUM_WAGER_NOT_REACHED,
    BET_DURATION_TOO_SHORT,
    VOTE_DURATION_TOO_SHORT,
    CLAIM_DURATION_TOO_SHORT,
    TIMESTAMPS_INCONSISTENT,

    // place_bet
    ACCOUNT_ALREADY_PLACED_BET,
    WAGER_AMOUNT_INCORRECT,
    BETTING_CLOSED,

    // place_bet and vote_on_outcome
    OUTCOME_NOT_FOUND,

    // vote_on_outcome
    ACCOUNT_ALREADY_VOTED,
    VOTING_CLOSED,
    VOTING_NOT_OPEN_YET,

    // vote_on_outcome and claim
    ACCOUNT_DID_NOT_BET,

    // claim
    CANNOT_CLAIM_YET,
    CLAIMING_CLOSED,

    // terminate
    CANNOT_TERMINATE_YET,

    // end_phase
    BET_ALREADY_IS_IN_LAST_PHASE,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
