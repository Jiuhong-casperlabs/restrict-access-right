#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec};

use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Key, Parameter, RuntimeArgs, URef, U512,
};

#[no_mangle]
pub fn get_deposit_purse() {
    let a = runtime::get_key("deposit_purse").unwrap();
    let b = a.into_uref().unwrap();
    let c = b.into_add();
    runtime::ret(CLValue::from_t(c).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut named_keys: BTreeMap<String, Key> = BTreeMap::new();
    let contract_purse = system::create_purse();

    named_keys.insert(String::from("deposit_purse"), contract_purse.into());

    let mut entry_points = EntryPoints::new();

    let entry_point_1 = EntryPoint::new(
        "get_deposit_purse",
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(entry_point_1);

    let (contracthash, _) = storage::new_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key("contract1", contracthash.into());
}
