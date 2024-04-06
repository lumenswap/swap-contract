use soroban_sdk::{Address, Env};

use crate::storage_types::{Tokens, ADMIN_KEY, TOKENS_KEY};

pub fn set_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&ADMIN_KEY, admin);
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&ADMIN_KEY).unwrap()
}

pub fn set_tokens(e: &Env, token0: Address, token1: Address) {
    let tokens = Tokens { token0, token1 };

    e.storage().instance().set(&TOKENS_KEY, &tokens);
}

pub fn get_tokens(e: &Env) -> Tokens {
    e.storage().instance().get(&TOKENS_KEY).unwrap()
}
