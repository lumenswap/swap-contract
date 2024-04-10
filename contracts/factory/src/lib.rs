#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod interfaces;
mod storage;
mod storage_types;

use interfaces::ifactory::IFactory;
use storage::{
    get_admin, get_fee, get_pair_by_id, get_pair_by_tokens, get_pairs_length, set_admin, set_fee,
};
use storage_types::Pair;

#[contract]
pub struct Factory;

#[contractimpl]
impl IFactory for Factory {
    fn initialize(e: Env, admin: Address, fee: u64) {
        set_admin(&e, admin);
        set_fee(&e, fee);
    }

    fn get_admin(e: Env) -> Address {
        get_admin(&e)
    }

    fn get_fee(e: Env) -> u64 {
        get_fee(&e)
    }

    fn set_fee(e: Env, new_fee: u64) {
        set_fee(&e, new_fee);
    }

    fn get_pair(e: Env, token0: Address, token1: Address) -> Pair {
        get_pair_by_tokens(&e, token0, token1)
    }

    fn get_pair_by_id(e: Env, id: u64) -> Pair {
        get_pair_by_id(&e, id)
    }

    fn get_pairs_length(e: Env) -> u64 {
        get_pairs_length(&e)
    }

    fn create_pair(e: Env, token0: Address, token1: Address) -> Address {
        // let pair = Pair {
        //     token0,
        //     token1,
        // }
        //
        // set_pair(&e, Pair { token0, token1 });
        //
        // check for existing pair with 2 tokens
        //
        // deploy a new pair contract
        //
        // return Address
        token0
    }
}

#[cfg(test)]
mod tests;
