use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct Pair {
    pub token0: Address,
    pub token1: Address,
    pub address: Address,
}
