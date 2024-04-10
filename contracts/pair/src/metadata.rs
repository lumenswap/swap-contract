use soroban_sdk::{contracttype, Env, String};

use crate::storage_types::DataKey;

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: String,
    pub symbol: String,
}

pub fn read_decimal(e: &Env) -> u32 {
    get_metadata(e).decimal
}

pub fn read_name(e: &Env) -> String {
    get_metadata(e).name
}

pub fn read_symbol(e: &Env) -> String {
    get_metadata(e).symbol
}

pub fn set_metadata(e: &Env, metadata: &TokenMetadata) {
    e.storage().instance().set(&DataKey::Metadata, metadata);
}

pub fn get_metadata(e: &Env) -> TokenMetadata {
    e.storage().instance().get(&DataKey::Metadata).unwrap()
}
