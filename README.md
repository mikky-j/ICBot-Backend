# IC Bot

Welcome to IC Bot, a telegram bot that helps people manage their ICP wallets

# Running

Run this command to deploy the ledger canister

```bash
dfx deploy ledger --argument "(variant {Init = record { token_name = \"NAME\"; token_symbol = \"SYMB\"; transfer_fee = 1000000; metadata = vec {}; minting_account = record {owner = principal \"$(dfx identity get-principal)\";}; initial_balances = vec {}; archive_options = record {num_blocks_to_archive = 1000000; trigger_threshold = 1000000; controller_id = principal \"$(dfx identity get-principal)\"}; }})"
```

Run this command to deploy the backend canister

```bash
dfx deploy
```
