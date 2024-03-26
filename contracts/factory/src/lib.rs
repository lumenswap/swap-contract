#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

pub trait IFactory {
    fn admin(e: Env) -> Address;
    fn fee(e: Env) -> u64;
    fn set_fee(e: Env) -> Result<bool, ()>;
    fn get_pair(e: Env, token_a: Address, token_b: Address) -> Result<Address, ()>;
    fn get_pair_by_index(e: Env, index: u64) -> Result<Address, ()>;
    fn get_pairs_length(e: Env) -> Result<u64, ()>;
    fn create_pair(e: Env, token_a: Address, token_b: Address) -> Result<Address, ()>;
}

#[contract]
pub struct Factory;

#[contractimpl]
impl Factory {}

#[cfg(test)]
mod tests;
