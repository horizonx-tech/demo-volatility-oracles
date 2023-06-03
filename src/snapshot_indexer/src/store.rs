use candid::{Principal, candid_method, CandidType};
use ic_cdk::{query};
use ic_cdk_timers::TimerId;
use serde::{Serialize, Deserialize};
use std::{cell::RefCell, collections::HashMap};

use crate::types::CallCanisterArgs;

thread_local! {
    static TARGET_CANISTER: RefCell<Principal> = RefCell::new(Principal::anonymous());
    static CALL_CANISTER_ARGS: RefCell<CallCanisterArgs> = RefCell::default();
    static SNAPSHOT_DATA: RefCell<HashMap<u64, u128>> = RefCell::new(HashMap::new());
    static TASK_INTERVAL_SECS: RefCell<u32> = RefCell::default();
    static TIMER_ID: RefCell<TimerId> = RefCell::default();
}

#[query]
#[candid_method(query)]
pub fn get_target_canister() -> Principal {
    TARGET_CANISTER.with(|state| *state.borrow())
}
pub fn set_target_canister(val: Principal) {
    TARGET_CANISTER.with(|state| *state.borrow_mut() = val);
}

#[query]
#[candid_method(query)]
pub fn get_call_canister_args() -> CallCanisterArgs {
    CALL_CANISTER_ARGS.with(|state| state.borrow().clone())
}
pub fn set_call_canister_args(value: CallCanisterArgs) {
    CALL_CANISTER_ARGS.with(|state| *state.borrow_mut() = value);
}

#[query]
#[candid_method(query)]
pub fn get_snapshot_data(idx: u64) -> u128 {
    SNAPSHOT_DATA.with(|state| *state.borrow().get(&idx).unwrap_or(&0))
}
#[query]
#[candid_method(query)]
pub fn get_snapshot_data_len() -> usize {
    SNAPSHOT_DATA.with(|state| state.borrow().len())
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnapshotDataStruct {
    pub key: u64,
    pub value: u128
}
#[query]
#[candid_method(query)]
pub fn get_all_snapshot_data() -> Vec<SnapshotDataStruct> {
    SNAPSHOT_DATA.with(|state| {
        state.borrow().iter().map(|(key, value)| {
            SnapshotDataStruct {
                key: *key,
                value: *value
            }
        }).collect()
    })
}

pub fn add_snapshot_data(idx: u64, value: u128) {
    SNAPSHOT_DATA.with(|state| state.borrow_mut().insert(idx, value));
}

pub fn clean_snapshot_data() {
    SNAPSHOT_DATA.with(|state| state.borrow_mut().clear());
}

#[query]
#[candid_method(query)]
pub fn get_task_interval_secs() -> u32 {
    TASK_INTERVAL_SECS.with(|state| *state.borrow())
}
pub fn set_task_interval_secs(value: u32) {
    TASK_INTERVAL_SECS.with(|state| *state.borrow_mut() = value);
}

pub fn get_timer_id() -> TimerId {
    TIMER_ID.with(|state| *state.borrow())
}
pub fn set_timer_id(value: TimerId) {
    TIMER_ID.with(|state| *state.borrow_mut() = value);
}
