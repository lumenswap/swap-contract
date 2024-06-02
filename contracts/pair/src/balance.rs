use crate::{
    extend::{extend_instance_ttl, extend_persistent_ttl},
    extend::{BUMP_AMOUNT, LIFETIME_THRESHOLD},
    storage_types::DataKey,
};
use soroban_sdk::{token::Client, Address, Env};

pub fn get_balance(e: &Env, contract: Address) -> i128 {
    Client::new(&e, &contract).balance(&e.current_contract_address())
}

pub fn read_balance(e: &Env, addr: Address) -> i128 {
    let key = DataKey::Balance(addr);

    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);

        balance
    } else {
        0
    }
}

fn write_balance(e: &Env, addr: Address, amount: i128) {
    let key = DataKey::Balance(addr);

    e.storage().persistent().set(&key, &amount);

    extend_instance_ttl(e);
    extend_persistent_ttl(e, &key);
}

pub fn receive_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());

    write_balance(e, addr, balance + amount);
}

pub fn spend_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());

    if balance < amount {
        panic!("insufficient balance");
    }

    write_balance(e, addr, balance - amount);
}
