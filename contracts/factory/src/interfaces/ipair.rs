use soroban_sdk::{symbol_short, vec, Address, BytesN, Env, Val, Vec};

pub mod ipair {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/pair.wasm");
}

pub fn deploy_pair(e: &Env, token0: Address, token1: Address, factory: Address) -> (Address, Val) {
    let wasm_hash = e.deployer().upload_contract_wasm(ipair::WASM);

    let salt = BytesN::from_array(&e, &[0; 32]);

    let init_fn = symbol_short!("init");
    let init_fn_args: Vec<Val> = vec![&e, token0.to_val(), token1.to_val(), factory.to_val()];

    let deployed_address = e
        .deployer()
        .with_address(e.current_contract_address(), salt)
        .deploy(wasm_hash);

    let res: Val = e.invoke_contract(&deployed_address, &init_fn, init_fn_args);

    (deployed_address, res)
}
