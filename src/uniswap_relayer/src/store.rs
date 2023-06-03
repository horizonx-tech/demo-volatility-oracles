use ic_cdk::export::Principal;
use ic_cdk_timers::TimerId;
use std::cell::RefCell;

use crate::env::EcdsaKeyEnvs;
use crate::types::CallCanisterArgs;

thread_local! {
    static TARGET_CANISTER: RefCell<Principal> = RefCell::new(Principal::anonymous());
    static CALL_CANISTER_ARGS: RefCell<CallCanisterArgs> = RefCell::default();
    static RPC_URL: RefCell<String>  = RefCell::default();
    static CHAIN_ID: RefCell<u64>  = RefCell::default();
    static ORACLE_ADDRESS: RefCell<String> = RefCell::default();
    static KEY_NAME: RefCell<String>  = RefCell::new(EcdsaKeyEnvs::LocalDevelopment.to_key_name());
    static TIMER_ID: RefCell<TimerId> = RefCell::default();
}

pub fn get_target_canister() -> Principal {
    TARGET_CANISTER.with(|state| *state.borrow())
}
pub fn set_target_canister(value: Principal) {
    TARGET_CANISTER.with(|state| *state.borrow_mut() = value);
}

pub fn get_call_canister_args() -> CallCanisterArgs {
    CALL_CANISTER_ARGS.with(|state| state.borrow().clone())
}
pub fn set_call_canister_args(value: CallCanisterArgs) {
    CALL_CANISTER_ARGS.with(|state| *state.borrow_mut() = value);
}

pub fn get_rpc_url() -> String {
    RPC_URL.with(|state| state.borrow().clone())
}
pub fn set_rpc_url(value: String) {
    RPC_URL.with(|state| *state.borrow_mut() = value);
}

pub fn get_chain_id() -> u64 {
    CHAIN_ID.with(|state| *state.borrow())
}
pub fn set_chain_id(value: u64) {
    CHAIN_ID.with(|state| *state.borrow_mut() = value);
}

pub fn get_oracle_address() -> String {
    ORACLE_ADDRESS.with(|state| state.borrow().clone())
}
pub fn set_oracle_address(value: String) {
    ORACLE_ADDRESS.with(|state| *state.borrow_mut() = value);
}

pub fn key_name() -> String {
    KEY_NAME.with(|state| state.borrow().clone())
}
pub fn set_key_name(env: EcdsaKeyEnvs) {
    KEY_NAME.with(|state| *state.borrow_mut() = env.to_key_name());
}

pub fn timer_id() -> TimerId {
    TIMER_ID.with(|state| *state.borrow())
}
pub fn set_timer_id(value: TimerId) {
    TIMER_ID.with(|state| *state.borrow_mut() = value);
}
