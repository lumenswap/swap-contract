use soroban_sdk::{assert_with_error, contract, contractimpl, Address, Env};

use crate::{
    errors,
    interfaces::{ifactory::IFactory, ipair::deploy_pair},
    storage::{
        get_admin, get_fee, get_pair_by_id, get_pair_by_tokens, get_pairs_length,
        increase_pairs_length, set_admin, set_fee, set_pair,
    },
    storage_types::{DataKey, Pair},
};

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
        let admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();

        admin.require_auth();

        set_fee(&e, new_fee);
    }

    fn get_pair(e: Env, token0: Address, token1: Address) -> Pair {
        get_pair_by_tokens(&e, token0, token1).unwrap()
    }

    fn get_pair_by_id(e: Env, id: u64) -> Pair {
        get_pair_by_id(&e, id)
    }

    fn get_pairs_length(e: Env) -> u64 {
        get_pairs_length(&e)
    }

    fn create_pair(e: Env, token0: Address, token1: Address) -> Address {
        let pair_exists = get_pair_by_tokens(&e, token0.clone(), token1.clone()).is_ok();

        assert_with_error!(&e, pair_exists, errors::Error::PairAlreadyExist);

        let (address, _) = deploy_pair(&e, token0.clone(), token1.clone());

        let pair = Pair {
            token0,
            token1,
            pair_address: address.clone(),
        };

        increase_pairs_length(&e);
        set_pair(&e, 0, pair);

        address
    }
}
