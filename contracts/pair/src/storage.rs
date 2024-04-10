use soroban_sdk::{Address, Env};

use crate::storage_types::{DataKey, Tokens};

pub fn set_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn set_tokens(e: &Env, token0: Address, token1: Address) {
    let tokens = Tokens { token0, token1 };

    e.storage().instance().set(&DataKey::Tokens, &tokens);
}

pub fn get_tokens(e: &Env) -> Tokens {
    e.storage().instance().get(&DataKey::Tokens).unwrap()
}
