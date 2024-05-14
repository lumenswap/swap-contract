use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    PairAlreadyExist = 0,
    InvalidAmount = 1,
    PairNotInitialized = 2,
}
