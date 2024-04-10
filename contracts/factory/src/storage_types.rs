use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug)]
pub enum DataKey {
    Fee,
    Admin,
    Pairs(u64),
    PairsLength,
    Pair(Address, Address),
}

#[derive(Clone)]
#[contracttype]
pub struct Pair {
    pub token0: Address,
    pub token1: Address,
    pub pair_address: Address,
}
