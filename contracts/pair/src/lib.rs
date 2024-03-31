#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String};

pub trait IPair {
    /// Returns the allowance for `spender` to transfer from `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address spending the tokens held by `from`.
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    /// Set the allowance by `amount` for `spender` to transfer/burn from
    /// `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address being authorized to spend the tokens held by
    ///   `from`.
    /// * `amount` - The tokens to be made available to `spender`.
    /// * `expiration_ledger` - The ledger number where this allowance expires. Cannot
    ///    be less than the current ledger number unless the amount is being set to 0.
    ///    An expired entry (where expiration_ledger < the current ledger number)
    ///    should be treated as a 0 amount allowance.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["approve", from: Address,
    /// spender: Address], data = [amount: i128, expiration_ledger: u32]`
    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32);

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which a balance is being queried. If the
    ///   address has no existing balance, returns 0.
    fn balance(env: Env, id: Address) -> i128;

    /// Transfer `amount` from `from` to `to`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens which will be
    ///   withdrawn from.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address],
    /// data = [amount: i128]`
    fn transfer(env: Env, from: Address, to: Address, amount: i128);

    /// Transfer `amount` from `from` to `to`, consuming the allowance of
    /// `spender`. Authorized by spender (`spender.require_auth()`).
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the transfer, and having its
    ///   allowance consumed during the transfer.
    /// * `from` - The address holding the balance of tokens which will be
    ///   withdrawn from.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address],
    /// data = [amount: i128]`
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);

    // /// Burn `amount` from `from`.
    // ///
    // /// # Arguments
    // ///
    // /// * `from` - The address holding the balance of tokens which will be
    // ///   burned from.
    // /// * `amount` - The amount of tokens to be burned.
    // ///
    // /// # Events
    // ///
    // /// Emits an event with topics `["burn", from: Address], data = [amount:
    // /// i128]`
    // fn burn(env: Env, from: Address, amount: i128);

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the burn, and having its allowance
    ///   consumed during the burn.
    /// * `from` - The address holding the balance of tokens which will be
    ///   burned from.
    /// * `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["burn", from: Address], data = [amount:
    /// i128]`
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);

    /// Returns the number of decimals used to represent amounts of this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn decimals(env: Env) -> u32;

    /// Returns the name for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn name(env: Env) -> String;

    /// Returns the symbol for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn symbol(env: Env) -> String;

    fn factory(env: Env) -> Address;

    fn token0(env: Env) -> Address;

    fn token1(env: Env) -> Address;

    fn get_reserves(env: Env) -> (i128, i128, i128);

    fn price0_cumulative_last(env: Env) -> i128;

    fn price1_cumulative_last(env: Env) -> i128;

    fn k_last(env: Env) -> i128;

    fn mint(env: Env, to: Address) -> i128;

    fn burn(env: Env, from: Address, amount: i128) -> (i128, i128);

    fn swap(env: Env, amount0_out: i128, amount1_out: i128, to: Address, data: Bytes);

    fn skim(env: Env, to: Address);

    fn sync(env: Env);

    fn initialize(env: Env, token0: Address, token1: Address);
}

#[contract]
pub struct Pair;

#[contractimpl]
impl IPair for Pair {
    /// Returns the allowance for `spender` to transfer from `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address spending the tokens held by `from`.
    fn allowance(_env: Env, _from: Address, _spender: Address) -> i128 {
        20000000
    }

    /// Set the allowance by `amount` for `spender` to transfer/burn from
    /// `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address being authorized to spend the tokens held by
    ///   `from`.
    /// * `amount` - The tokens to be made available to `spender`.
    /// * `expiration_ledger` - The ledger number where this allowance expires. Cannot
    ///    be less than the current ledger number unless the amount is being set to 0.
    ///    An expired entry (where expiration_ledger < the current ledger number)
    ///    should be treated as a 0 amount allowance.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["approve", from: Address,
    /// spender: Address], data = [amount: i128, expiration_ledger: u32]`
    fn approve(
        _env: Env,
        _from: Address,
        _spender: Address,
        _amount: i128,
        _expiration_ledger: u32,
    ) {
    }

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which a balance is being queried. If the
    ///   address has no existing balance, returns 0.
    fn balance(_env: Env, _id: Address) -> i128 {
        20000000
    }

    /// Transfer `amount` from `from` to `to`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens which will be
    ///   withdrawn from.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address],
    /// data = [amount: i128]`
    fn transfer(_env: Env, _from: Address, _to: Address, _amount: i128) {}

    /// Transfer `amount` from `from` to `to`, consuming the allowance of
    /// `spender`. Authorized by spender (`spender.require_auth()`).
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the transfer, and having its
    ///   allowance consumed during the transfer.
    /// * `from` - The address holding the balance of tokens which will be
    ///   withdrawn from.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address],
    /// data = [amount: i128]`
    fn transfer_from(_env: Env, _spender: Address, _from: Address, _to: Address, _amount: i128) {}

    // /// Burn `amount` from `from`.
    // ///
    // /// # Arguments
    // ///
    // /// * `from` - The address holding the balance of tokens which will be
    // ///   burned from.
    // /// * `amount` - The amount of tokens to be burned.
    // ///
    // /// # Events
    // ///
    // /// Emits an event with topics `["burn", from: Address], data = [amount:
    // /// i128]`
    // fn burn(env: Env, from: Address, amount: i128);

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the burn, and having its allowance
    ///   consumed during the burn.
    /// * `from` - The address holding the balance of tokens which will be
    ///   burned from.
    /// * `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["burn", from: Address], data = [amount:
    /// i128]`
    fn burn_from(_env: Env, _spender: Address, _from: Address, _amount: i128) {}

    /// Returns the number of decimals used to represent amounts of this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn decimals(_env: Env) -> u32 {
        7
    }

    /// Returns the name for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn name(env: Env) -> String {
        String::from_str(&env, "UniswapPool")
    }

    /// Returns the symbol for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn symbol(env: Env) -> String {
        String::from_str(&env, "Pool")
    }

    fn factory(env: Env) -> Address {
        let a = String::from_str(
            &env,
            "GA555NCMFLIHKGGM3D6PMUJ2ELKU6RH2ESYOWMQOM2J54LYSDKTRH7N6",
        );

        Address::from_string(&a)
    }

    fn token0(env: Env) -> Address {
        let a = String::from_str(
            &env,
            "GA555NCMFLIHKGGM3D6PMUJ2ELKU6RH2ESYOWMQOM2J54LYSDKTRH7N6",
        );

        Address::from_string(&a)
    }

    fn token1(env: Env) -> Address {
        let a = String::from_str(
            &env,
            "GA555NCMFLIHKGGM3D6PMUJ2ELKU6RH2ESYOWMQOM2J54LYSDKTRH7N6",
        );

        Address::from_string(&a)
    }

    fn get_reserves(_env: Env) -> (i128, i128, i128) {
        (1, 2, 3)
    }

    fn price0_cumulative_last(_env: Env) -> i128 {
        123
    }

    fn price1_cumulative_last(_env: Env) -> i128 {
        123
    }

    fn k_last(_: Env) -> i128 {
        123
    }

    fn mint(_: Env, _to: Address) -> i128 {
        123
    }

    fn burn(_: Env, _from: Address, _amount: i128) -> (i128, i128) {
        (123, 456)
    }

    fn swap(_env: Env, _amount0_out: i128, _amount1_out: i128, _to: Address, _data: Bytes) {}

    fn skim(_env: Env, _to: Address) {}

    fn sync(_env: Env) {}

    fn initialize(_env: Env, _token0: Address, _token1: Address) {}
}
