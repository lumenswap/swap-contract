use soroban_sdk::{contracttype, Address, String};

#[derive(Clone)]
#[contracttype]
pub struct Tokens {
    pub token0: Address,
    pub token1: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct TokenMeta {
    pub decimal: u32,
    pub name: String,
    pub symbol: String,
}

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Tokens,
    Metadata,
    Factory,
    Nonce(Address),
    State(Address),
    Balance(Address),
    Allowance(AllowanceDataKey),
}
