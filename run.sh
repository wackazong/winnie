#!/usr/bin/env bash
set -e

# if you change this you need to adjust the initArgs for the contract as well
OUTCOMES=( A B Draw ) 
BETTORS=( one two three four five six seven eight nine ten )
OWNER=$2
CONTRACT=bet.$OWNER

if [ "$1" == "--cleanup" ] && [ ! -z "$OWNER" ];then
    echo ▶️ DELETE CONTRACT ACCOUNT
    if near state bet.$OWNER >> /dev/null; then
        near delete bet.$OWNER $OWNER
    fi

    echo ▶️ DELETE BETTOR ACCOUNTS
    for I in "${BETTORS[@]}"; do
        if near state $I.$OWNER >> /dev/null; then
            echo Delete bettor account $I.$OWNER
            near delete $I.$OWNER $OWNER 
        fi
    done
    exit
elif [ "$1" == "--bet" ] && [ ! -z "$OWNER" ];then
    echo ▶️ TEST
    cargo test -- --nocapture

    echo ▶️ BUILD
    cargo build --target wasm32-unknown-unknown --release

    echo ▶️ CREATE BETTOR ACCOUNTS
    for I in "${BETTORS[@]}"; do
        if ! near state $I.$OWNER >> /dev/null; then
            echo Create bettor account $I.$OWNER
            near create-account $I.$OWNER --masterAccount $OWNER --initialBalance 3
        fi
    done

    echo ▶️ DEPLOY
    if near state bet.$OWNER 2>&1 >> /dev/null; then
        echo Deleting old contract account to remove old state
        near delete bet.$OWNER $OWNER
    fi
    near create-account bet.$OWNER --masterAccount $OWNER --initialBalance 20
    near deploy bet.$OWNER target/wasm32-unknown-unknown/release/winnie.wasm --initFunction new --initArgs "{
        \"owner\": \"$OWNER\",
        \"description\": \"How will the game end?\",
        \"bettors\": [\"one.$OWNER\",\"two.$OWNER\",\"three.$OWNER\",\"four.$OWNER\",\"five.$OWNER\",
        \"six.$OWNER\",\"seven.$OWNER\",\"eight.$OWNER\",\"nine.$OWNER\",\"ten.$OWNER\"],
        \"wager\": \"1000000000000000000000000\",
        \"outcomes\": {
            \"A\": {\"description\": \"Team A will win\"},
            \"B\": {\"description\": \"Team B will win\"},
            \"Draw\": {\"description\": \"It will be a draw\"}
            },
        \"bet_until\": $(($(echo $(date +%s)000000000) + 240000000000)),
        \"vote_until\": $(($(echo $(date +%s)000000000) + 480000000000)),
        \"claim_until\": $(($(echo $(date +%s)000000000) + 720000000000))
        }"
    near view $CONTRACT view

    echo ▶️ MAKE BETS
    for I in "${BETTORS[@]}"; do
        OUTCOME=${OUTCOMES[$(($RANDOM % ${#OUTCOMES[@]}))]}
        near call $CONTRACT place_bet "{\"outcome\": \"$OUTCOME\"}" --accountId $I.$OWNER --deposit 1
    done
    near view $CONTRACT view
    near call $CONTRACT end_phase --accountId $OWNER

    echo ▶️ MAKE VOTES
    OUTCOMES=( A A A B Draw ) # make A more likely for the voting
    for I in "${BETTORS[@]}"; do
        VOTE=${OUTCOMES[$(($RANDOM % ${#OUTCOMES[@]}))]}
        near call $CONTRACT vote_on_outcome "{\"vote\": \"$VOTE\"}" --accountId $I.$OWNER
    done
    near view $CONTRACT view
    near call $CONTRACT end_phase --accountId $OWNER

    echo ▶️ CLAIM
    for I in "${BETTORS[@]}"; do
        near call $CONTRACT claim --accountId $I.$OWNER --gas 100000000000000
    done
    near view $CONTRACT view
    near call $CONTRACT end_phase --accountId $OWNER

    echo ▶️ TERMINATE
    near call $CONTRACT terminate "{\"vote\": \"$OUTCOME\"}" --accountId $OWNER

    echo ▶️ END 
    exit
fi

echo
echo "Usage: run.sh [--bet|--cleanup] account_id"
echo
echo "--bet        Runs a full bet cycle"
echo "--cleanup    Deletes all created accounts"
echo
echo "account_id   Account to use for creating subaccounts for contract and bettors"
echo
