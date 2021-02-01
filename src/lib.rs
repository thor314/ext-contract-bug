// Ref: https://github.com/snjax/nep4_nostd_example/blob/main/src/lib.rs
#![allow(unused_imports)]
#![allow(dead_code)]
#![no_std]
#![feature(core_intrinsics, alloc_error_handler)]

extern crate alloc;
use near_sdk_pure::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::*,
  ext_contract,
  *,
};

mod utils;

#[cfg(target = "wasm32")]
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[ext_contract]
pub trait Ext{}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NoStdTest {}

/// Default must be implemented for wasm compilation.
impl Default for NoStdTest {
  fn default() -> Self {
    panic!("No_std_test")
  }
}
#[near_bindgen]
impl NoStdTest {
  #[init]
  pub fn new() -> Self {
    assert!(!env::state_exists(), "Already, initialized");
    Self {}
  }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
  fn test() {
    assert!(true);
  }
}
