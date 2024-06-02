use soroban_sdk::{
    assert_with_error, contract, contractimpl, contractmeta,
    token::{self, Interface as _, TokenClient as Client},
    Address, Bytes, Env, String, Vec,
};

use crate::{
    allowance::{read_allowance, spend_allowance, write_allowance},
    balance::{get_balance, read_balance, receive_balance, spend_balance},
    errors::{self},
    events,
    interface::IPair,
    metadata::{
        get_metadata_result, read_decimal, read_name, read_symbol, set_metadata, TokenMetadata,
    },
    storage::{get_factory, get_reserve_0, get_reserve_1, get_tokens, set_factory, set_tokens},
    string::create_name,
    utils::check_nonnegative_amount,
};

contractmeta!(
    key = "Description",
    val = "Lumenswap Protocol - Constant product AMM"
);

#[contract]
pub struct Pair;

impl IPair for Pair {
    fn init(e: Env, token0: Address, token1: Address, factory: Address) {
        assert_with_error!(
            &e,
            get_metadata_result(&e).is_some(),
            errors::Error::PairAlreadyExist
        );

        let name0 = Client::new(&e, &token0).name();
        let symbol0 = Client::new(&e, &token0).symbol();

        let name1 = Client::new(&e, &token1).name();
        let symbol1 = Client::new(&e, &token1).symbol();

        let name = create_name(&e, &name0, &name1);
        let symbol = create_name(&e, &symbol0, &symbol1);

        let m = TokenMetadata {
            decimal: 7,
            name,
            symbol,
        };

        set_metadata(&e, &m);
        set_factory(&e, factory);
        set_tokens(&e, token0, token1);
    }

    fn factory(e: Env) -> Address {
        get_factory(&e)
    }

    fn token0(e: Env) -> Address {
        get_tokens(&e).token0
    }

    fn token1(e: Env) -> Address {
        get_tokens(&e).token1
    }

    // TODO: complete
    fn get_reserves(e: Env) -> (i128, i128) {
        (get_reserve_0(&e), get_reserve_1(&e))
    }

    // TODO: complete
    fn price0_cumulative_last(_env: Env) -> i128 {
        123
    }

    // TODO: complete
    fn price1_cumulative_last(_env: Env) -> i128 {
        123
    }

    // TODO: complete
    fn k_last(_: Env) -> i128 {
        123
    }

    // TODO: Complete
    fn mint(e: Env, to: Address, amounts: Vec<i128>, min_amount_out: i128) -> i128 {
        assert_with_error!(
            &e,
            get_metadata_result(&e).is_none(),
            errors::Error::PairNotInitialized
        );

        assert_with_error!(&e, amounts.len() != 2, errors::Error::InvalidAmount);

        let reserve0 = get_reserve_0(&e);
        let reserve1 = get_reserve_1(&e);

        let tokens = get_tokens(&e);

        let balance0 = get_balance(&e, tokens.token0);
        let balance1 = get_balance(&e, tokens.token1);

        let amount0 = balance0 - reserve0;
        let amount1 = balance1 - reserve1;

        amount0
    }

    // fn burn(_: Env, _from: Address, _amount: i128) -> (i128, i128) {
    //     (123, 456)
    // }

    // TODO: Complete
    fn swap(_env: Env, _amount0_out: i128, _amount1_out: i128, _to: Address, _data: Bytes) {}

    // TODO: Complete
    fn skim(_env: Env, _to: Address) {}

    // TODO: Complete
    fn sync(_env: Env) {}
}

#[contractimpl]
impl token::Interface for Pair {
    /// Returns the allowance for `spender` to transfer from `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address spending the tokens held by `from`.
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        read_allowance(&e, from, spender).amount
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
    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);

        events::approve(&e, from, spender, amount, expiration_ledger);
    }

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which a balance is being queried. If the
    ///   address has no existing balance, returns 0.
    fn balance(e: Env, id: Address) -> i128 {
        read_balance(&e, id)
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
    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        events::transfer(&e, from, to, amount);
    }

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
    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        events::transfer(&e, from, to, amount)
    }

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
    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        events::burn(&e, from, amount)
    }

    /// Returns the number of decimals used to represent amounts of this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    /// Returns the name for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn name(e: Env) -> String {
        read_name(&e)
    }

    /// Returns the symbol for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        spend_balance(&e, from.clone(), amount);
        events::burn(&e, from, amount);
    }
}
