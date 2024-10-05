# `ICP Token Wallet Backend`

This project is a basic implementation of a token wallet system on the Internet Computer using Rust and Candid. It allows users to send and receive tokens, as well as check their balance.

## Features
* Initialize Wallet: The contract initializes with the deployer as the owner and credits the owner's wallet with an initial balance of 1000 tokens.
* Send Tokens: Users can send tokens to other users if they have sufficient balance.
* Receive Tokens: Users can receive tokens from other users or external sources.
Get Balance: Users can query the current balance in their wallet.

## Prerequisites
To build and deploy this project, ensure you have the following tools installed:
* DFX
* Rust with wasm32-unknown-unknown target
  * Install the target: rustup target add wasm32-unknown-unknown
* Cargo

## Project Structure
```
.
├── src
│   ├── icp_token_wallet_backend_backend
│   │   ├── src
│   │   │   ├── lib.rs                   # Contains the backend logic for token wallet
│   │   ├── Cargo.toml                   # Rust project configuration file
│   │   ├── icp_token_wallet_backend.did # Candid interface file
├── dfx.json                              # DFX project configuration
├── Cargo.lock                            # Lockfile for dependencies
├── README.md                             # This readme file

```

## How It Works
* State Management: The contract maintains a global mutable state containing user wallets. Each wallet stores the user's balance.
* State Initialization: Upon initialization, the deployer's wallet is created and initialized with 1000 tokens.
* Token Transfer: Users can transfer tokens to other users if their wallet exists and they have sufficient balance.
* Receiving Tokens: Users can add tokens to their wallet using the receive_tokens function.
* Query Balance: Users can query the balance of their wallet at any time.

## Setup and Usage
### 1. Clone the repository
```bash
git clone https://github.com/yourusername/icp_token_wallet_backend.git
cd icp_token_wallet_backend
```
### 2. Build the canister
```
dfx build
# if you encounter error try : cargo build --release
```
This will compile the Rust code into WebAssembly (Wasm) and prepare it for deployment on the Internet Computer.

### 3. Deploy the Canister
Deploy the canister to a local Internet Computer network:
```
dfx start --background
dfx deploy
```
### 4. Interact with the Canister
After deployment, you can interact with the deployed canister using DFX commands:
```bash
* Check Balance :
dfx canister call icp_token_wallet_backend get_balance

* Send Tokens:
dfx canister call icp_token_wallet_backend send_tokens '(100, "recipient_principal_id")'

* Receive Tokens:
dfx canister call icp_token_wallet_backend receive_tokens '(100)'
```
| Note: Replace "recipient_principal_id" with an actual principal ID (you can use dfx identity get-principal to generate one).

### 5. Stop the Local DFX Network
Once you're done testing, stop the local DFX network:
```bash
dfx stop
```


## Sample Output Imaages :
### 1. Get Balance Before Transaction :
![get_balance_before_transaction](https://github.com/user-attachments/assets/c9b5500a-d83c-41cd-aaaa-218f7595dc5a)

### 2. Send Token :
![send_token](https://github.com/user-attachments/assets/87d3f5da-a96b-456e-bee6-411f7372487a)

### 3. Receive Token :
![receive_token](https://github.com/user-attachments/assets/9ca7cbf6-b885-4cee-bce3-bf6c6c7563f1)

### 4. Get Balance After Trancation :
![get_balance_after_transaction](https://github.com/user-attachments/assets/cc1c7d26-6329-4022-a497-b2071534d245)

