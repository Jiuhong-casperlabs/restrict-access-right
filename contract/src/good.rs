#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, RuntimeArgs, URef, U512};

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg("amount");

    let marketplace_contract = runtime::get_named_arg("marketplace_contract");

    let deposit_purse: URef =
        runtime::call_contract(marketplace_contract, "get_deposit_purse", runtime_args! {});
    let account_purse = account::get_main_purse();

    system::transfer_from_purse_to_purse(account_purse, deposit_purse, amount, None)
        .unwrap_or_revert();
    // system::transfer_from_purse_to_purse(deposit_purse, account_purse, amount, None)
    //     .unwrap_or_revert();
}
