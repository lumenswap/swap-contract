#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod interface;
mod storage_types;

use interface::IFactory;
use storage_types::Pair;

#[contract]
pub struct Factory;

#[contractimpl]
impl IFactory for Factory {
    fn initialize(e: Env, admin: Address) {}

    fn get_admin(e: Env) -> Address {
        // get_admin(&e)
    }

    fn get_fee(e: Env) -> u64 {
        3000
        // get_fee(&e)
    }

    fn set_fee(e: Env, new_fee: u64) {
        set_fee(&e, new_fee);
    }

    fn get_pair(e: Env, token0: Address, token1: Address) -> Result<Pair, ()> {}

    fn get_pair_by_index(e: Env, index: u64) -> Result<Pair, ()> {}

    fn get_pairs_length(e: Env) -> u64 {
        0
    }

    fn create_pair(e: Env, token0: Address, token1: Address) -> Address {
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
