#
export NEAR_ENV=mainnet
#
export ANCHOR_ACCOUNT_ID=myriad.octopus-registry.near
export OWNER_ACCOUNT_ID=octopus-registry.near
#
#
near deploy --accountId $ANCHOR_ACCOUNT_ID --wasmFile res/appchain_anchor.wasm
near call $ANCHOR_ACCOUNT_ID new '{"appchain_id":"myriad","appchain_registry":"octopus-registry.near","oct_token":"f5cfbc74057c610c8ef151a439252680ac68c6dc.factory.bridge.near"}' --accountId $OWNER_ACCOUNT_ID --gas 200000000000000
near call f5cfbc74057c610c8ef151a439252680ac68c6dc.factory.bridge.near storage_deposit '{"account_id":"myriad.octopus-registry.near","registration_only":null}' --accountId $OWNER_ACCOUNT_ID --deposit 0.0125
#
near call $ANCHOR_ACCOUNT_ID set_token_price_maintainer_account '{"account_id":"octopus-registry.near"}' --accountId $OWNER_ACCOUNT_ID
near call $ANCHOR_ACCOUNT_ID set_price_of_oct_token '{"price":"4000000"}' --accountId $OWNER_ACCOUNT_ID
#
near call $ANCHOR_ACCOUNT_ID set_relayer_account '{"account_id":"octopus-counter.near"}' --accountId $OWNER_ACCOUNT_ID
#
near call $ANCHOR_ACCOUNT_ID migrate_state '' --accountId $OWNER_ACCOUNT_ID --gas 200000000000000
