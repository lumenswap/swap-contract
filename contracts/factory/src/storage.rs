use soroban_sdk::{Address, Env};

use crate::storage_types::{DataKey, Pair};

pub fn set_admin(e: &Env, admin: Address) {
    e.storage().instance().set(&DataKey::Admin, &admin);
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn set_fee(e: &Env, fee: u64) {
    e.storage().instance().set(&DataKey::Fee, &fee);
}

pub fn get_fee(e: &Env) -> u64 {
    e.storage().instance().get(&DataKey::Fee).unwrap()
}

pub fn set_pair(e: &Env, id: u64, pair: Pair) {
    let p = pair.clone();

    e.storage().instance().set(&DataKey::Pairs(id), &pair);
    e.storage()
        .instance()
        .set(&DataKey::Pair(pair.token0, pair.token1), &p);
}

pub fn get_pair_by_tokens(e: &Env, token0: Address, token1: Address) -> Result<Pair, ()> {
    let p = e.storage().instance().get(&DataKey::Pair(token0, token1));

    match p {
        Some(x) => Ok(x),
        None => Err(()),
    }
}

pub fn get_pair_by_id(e: &Env, id: u64) -> Pair {
    e.storage().instance().get(&DataKey::Pairs(id)).unwrap()
}

pub fn get_pairs_length(e: &Env) -> u64 {
    e.storage().instance().get(&DataKey::PairsLength).unwrap()
}

pub fn increase_pairs_length(e: &Env) {
    let current_len = get_pairs_length(&e);

    let new_len = current_len + 1;

    e.storage().instance().set(&DataKey::PairsLength, &new_len);
    // TODO: extend_ttl
}
