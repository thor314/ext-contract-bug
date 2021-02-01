#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  json_types::U128,
  serde::{Deserialize, Serialize},
  serde_json::json,
  *,
};
use near_sdk_sim::{
  account::AccessKey, call, deploy, init_simulator, near_crypto::Signer, to_yocto, view,
  ContractAccount, ExecutionResult, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT,
};

// Bring contract crate into namespace
extern crate no_std_test;
use no_std_test::No_std_test;

// Load contracts' bytes.
near_sdk_sim::lazy_static! {
  static ref WASM_BYTES: &'static [u8] = include_bytes!("../res/no_std_test.wasm").as_ref();
}

/// Deploy the contract(s) and create some no_std_test accounts. Returns:
/// - The No_std_test Contract
/// - Root Account
/// - Testnet Account (utility suffix for building other addresses)
/// - A deployer account address
fn init(
  initial_balance: u128,
  deploy_to: &str,
) -> (
  ContractAccount<No_std_testContract>,
  UserAccount, // root
  UserAccount, // testnet suffix
  UserAccount, // deployer account
) {
  // Root account has address: "root"
  let root_account = init_simulator(None);

  // Other accounts may be created from the root account
  // Note: address naming is fully expressive: we may create any suffix we desire, ie testnet, near, etc.
  // but only those two (.testnet, .near) will be used in practice.
  let testnet = root_account.create_user("testnet".to_string(), to_yocto("1000000"));

  // We need an account to deploy the contracts from. We may create subaccounts of "testnet" as follows:
  let deployer = testnet.create_user(deploy_to.to_string(), to_yocto("1000000"));

  // uses default values for deposit and gas
  let no_std_test_contract = deploy!(
      contract: No_std_testContract,
      contract_id: "no_std_test",
      bytes: &WASM_BYTES,
      // User deploying the contract
      signer_account: deployer,
      // init method
      init_method: new()
  );
  (no_std_test_contract, root_account, testnet, deployer)
}

/// Helper to log ExecutionResult outcome of a call/view
fn print_helper(res: ExecutionResult) {
  println!("Promise results: {:#?}", res.promise_results());
  //println!("Receipt results: {:#?}", res.get_receipt_results());
  println!("Profiling: {:#?}", res.profile_data());
  //println!("Result: {:#?}", res);
  assert!(res.is_ok());
}

#[test]
fn simtest() {
  let (no_std_test_contract, root_account, testnet, deployer) = init(to_yocto("1000"), "me");

  // let view = view!(no_std_test_contract.MYMETHOD());
  // print_helper(res);

  /*
  let res = call!(
    deployer,
    no_std_test_contract.MYMETHOD(),
    deposit = STORAGE_AMOUNT // send this amount to a payable function, or exclude this line if send 0
  );
  print_helper(res);
  */
}
