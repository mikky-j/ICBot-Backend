# ICBot Backend Canister

Welcome to the backend canister repository for ICBot! This repository contains the codebase for the backend functionality of ICBot, a user-friendly Telegram Bot designed to simplify interaction and management of Internet Computer Wallets.

### Overview 📋

ICBot aims to provide users with a seamless experience for managing their ICP wallets directly from the Telegram platform. The backend canister is responsible for handling user authentication, wallet creation, transaction processing, and other essential functionalities to ensure smooth operation of the bot.
Features

- **Authentication System**: Secure user authentication mechanism to manage user access and ensure data integrity.
- **Wallet Creation**: Functionality for users to create and manage ICP wallets within the application.
- **Transaction Processing**: Handling the sending and receiving of ICP tokens between users' wallets.
- **Error Handling**: Comprehensive error handling to provide informative feedback to users in case of errors or issues.

### Technologies Used 🧑‍💻

- **Rust**: The programming language used to develop the backend canister, offering strong typing and scalability and speed 🦀.
- **Internet Computer Protocol (ICP)**: The underlying blockchain infrastructure used to process transactions and interact with users' wallets.
- **DFINITY Canister SDK**: Enables interaction with the Internet Computer environment and deployment of canisters.

# Running 

To set up the backend canister locally for development or testing purposes, follow these steps:

- Clone this repository to your local machine:

```bash

git clone https://github.com/mikky-j/ICBot-Backend.git
```

- Navigate to the project directory:

```bash

cd ICBot-Backend
```

- Install dependencies:
```bash
npm install
```

- Deploy the ledger canister

```bash
dfx deploy ledger --argument "(variant {Init = record { token_name = \"NAME\"; token_symbol = \"SYMB\"; transfer_fee = 1000000; metadata = vec {}; minting_account = record {owner = principal \"$(dfx identity get-principal)\";}; initial_balances = vec {}; archive_options = record {num_blocks_to_archive = 1000000; trigger_threshold = 1000000; controller_id = principal \"$(dfx identity get-principal)\"}; }})"
```

Build and deploy the backend canister:

```bash
dfx deploy
```

Once deployed, you can start testing and interacting with the backend canister using appropriate API endpoints.
