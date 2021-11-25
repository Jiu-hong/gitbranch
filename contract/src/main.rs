#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, Key};

#[no_mangle]
pub extern "C" fn call() {
    let a =  storage::new_dictionary("mydic").unwrap_or_revert();
    let b = runtime::get_key("mydic").unwrap_or_revert();
    runtime::put_key("myvalue1", a.into());
    runtime::put_key("myvalue", b);
}
