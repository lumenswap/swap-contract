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

pub fn get_pair_by_tokens(e: &Env, token0: Address, token1: Address) -> Pair {
    e.storage()
        .instance()
        .get(&DataKey::Pair(token0, token1))
        .unwrap()
}

pub fn get_pair_by_id(e: &Env, id: u64) -> Pair {
    e.storage().instance().get(&DataKey::Pairs(id)).unwrap()
}

pub fn get_pairs_length(e: &Env) -> u64 {
    e.storage().instance().get(&DataKey::PairsLength).unwrap()
}
