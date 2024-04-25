#![no_std]

mod allowance;
mod balance;
mod contract;
mod errors;
mod events;
mod interface;
mod metadata;
mod storage;
mod storage_types;
mod string;

pub use crate::contract::Pair;
