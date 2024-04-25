use soroban_sdk::{Address, Bytes, Env};

pub trait IPair {
    fn factory(env: Env) -> Address;

    fn token0(env: Env) -> Address;

    fn token1(env: Env) -> Address;

    fn get_reserves(env: Env) -> (i128, i128, i128);

    fn price0_cumulative_last(env: Env) -> i128;

    fn price1_cumulative_last(env: Env) -> i128;

    fn k_last(env: Env) -> i128;

    fn mint(env: Env, to: Address) -> i128;

    // fn burn(env: Env, from: Address, amount: i128) -> (i128, i128);

    fn swap(env: Env, amount0_out: i128, amount1_out: i128, to: Address, data: Bytes);

    fn skim(env: Env, to: Address);

    fn sync(env: Env);

    fn init(env: Env, token0: Address, token1: Address, factory: Address);
}
