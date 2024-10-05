use candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, query, update};
use ic_cdk::api::{caller};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Default, Clone)]
struct Wallet {
    balance: u64,
}

#[derive(CandidType, Deserialize, Default)]
struct State {
    owner: String,
    wallets: HashMap<String, Wallet>,
}

static mut STATE: Option<State> = None;

fn get_state() -> &'static mut State {
    unsafe {
        STATE.get_or_insert_with(State::default)
    }
}

#[init]
fn init() {
    let state = get_state();
    state.owner = caller().to_string();
    state.wallets.insert(state.owner.clone(), Wallet { balance: 1000 });
}

#[update]
fn send_tokens(amount: u64, recipient: String) -> String {
    let state = get_state();
    let sender = caller().to_string();

    if !state.wallets.contains_key(&sender) {
        return "Sender wallet does not exist".to_string();
    }

    if !state.wallets.contains_key(&recipient) {
        return "Recipient wallet does not exist".to_string();
    }

    let sender_balance = state.wallets.get(&sender).unwrap().balance;

    if sender_balance < amount {
        return "Insufficient balance".to_string();
    }

    state.wallets.get_mut(&sender).unwrap().balance -= amount;
    state.wallets.get_mut(&recipient).unwrap().balance += amount;

    format!("Sent {} tokens to {}", amount, recipient)
}

#[update]
fn receive_tokens(amount: u64) -> String {
    let state = get_state();
    let recipient = caller().to_string();

    if !state.wallets.contains_key(&recipient) {
        state.wallets.insert(recipient.clone(), Wallet { balance: 0 });
    }

    state.wallets.get_mut(&recipient).unwrap().balance += amount;
    format!("Received {} tokens", amount)
}

#[query]
fn get_balance() -> u64 {
    let state = get_state();
    let caller = caller().to_string();

    match state.wallets.get(&caller) {
        Some(wallet) => wallet.balance,
        None => 0,
    }
}
