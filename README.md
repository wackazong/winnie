# Winnie

A smart contract for crowdsourced, oracle-less betting on future events. Featuring built-in incentive mechanisms for truthful resolution.

## Concept

The contract can be deployed by somebody wishing to start a bet on a future event (the owner). They can allow other participants to participate in the bet. Additionally, they define a number of possible outcomes for the bet and set a wager amount. Also, they define until which time betting is possible. Bettors can place a bet with a smart contract call.

After the betting is closed, a voting round is started automatically. All bettors vote on the outcome of the bet via another smart contract call. The result of the bet is calculated using the votes of the bettors. If the majority of the loosing bettors voted for one specific outcome, this outcome is accepted as final result. If there is no majority, the bet is declared a draw. Voting is also limited to a pre-defined period.

After voting the bettors can claim theirs wins. The win amount for each bettor depends on whether the bet is a draw or not.

In the case of a final result, bettors who bet on the final result are added to the list of winners, plus a random selection of 10% (at least 1) of the bettors that did not bet on the final result but voted on it. This is done for incentivizing truthful voting. 95% of the total wager amount is distributed equally to this list of winners. 5% go to the owner.

In the case of a draw, 50% of the total wager amount is distributed among all bettors who placed a bet. The other 50% go to the owner. This is also done for incentivizing truthful voting.

## Example Story

Alice wants to bet on a football game of the Distributed Ledgers vs. the Merkle Trees. She deploys the contract with the following outcomes and invites nine of her friends to bet:

- A: the Distributed Ledgers win the game
- B: the Merkle Trees win the game
- Draw: It's a draw

She sets the duration of the betting, voting and claiming phases to 1 day each, as the game is tomorrow at the same time. The wager is defined by her with 1 NEAR. All of her friends place their bets, paying 1 NEAR each.

|Player|Bet|
| --- | --- |
|One|B|
|Two|Draw|
|Three|A|
|Four|A|
|Five|B|
|Six|B|
|Seven|B|
|Eight|Draw|
|Nine|B|
|Ten|A|

The takes place the next day. The Distributed Ledgers win over the Merkle Trees. After the game, the voting phase begins. Alice and her friends add their votes to the smart contract.

|Player|Bet|Vote|
| --- | --- | --- |
|One|B|A|
|Two|Draw|A|
|Three|A|A|
|Four|A|A|
|Five|B|A|
|Six|B|A|
|Seven|B|A|
|Eight|Draw|B|
|Nine|B|B|
|Ten|A|B|

After voting has finished, everybody claims their wins. The smart contract determines the result. There is one good looser who bet on the wrong outcome but voted for the final result that is rewarded with a win (Player One). Players Three, Four and Ten bet on the correct outcome and also win the same amount.

|Player|Bet|Vote|Win|
| --- | --- | --- | --- |
|One|B|A|2.38|
|Two|Draw|A|0|
|Three|A|A|2.38|
|Four|A|A|2.38|
|Five|B|A|0|
|Six|B|A|0|
|Seven|B|A|0|
|Eight|Draw|B|0|
|Nine|B|B|0|
|Ten|A|B|2.38|

## Getting started

This project contains:

- The source code for the smart contract, structured into different files. Each method of the smart contract has its own file.
- Unit tests for the methods. The unit tests are located in the same file as the method.
- A script to test, build and run a full bet cycle with all necessary commands. You just need to supply a NEAR account id with about 40 NEAR on it. The script will create accounts for the contract and ten bettors, deploy the contract and run a full betting cycle with random bets and votes. The resulting terminal output will contain JSON data that will show you the results of the bet.

## Installation

1. Clone the repo
2. Create a NEAR account on testnet (<https://wallet.testnet.near.org>)
3. Install near-cli (<https://docs.near.org/docs/tools/near-cli#setup>)
4. Login with near-cli into your new account: `near login`
5. Run the script with `./run.sh --bet <YOUR_NEW_ACCOUNT_ID>`
6. Watch the terminal output

## Where to go from here

It would be nice to let bettors bet/vote secretly. For that, the bets/votes would have to be encrypted. The private key could be distributed to all bettors at the beginning of the bet. Only when all bettors have placed their bet it would be possible to decrypt the bets/votes. But: if one bettor does not participate the bets/votes can not be decrypted.
