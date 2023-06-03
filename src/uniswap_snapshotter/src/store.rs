use ic_cdk_timers::TimerId;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::types::Price;
use crate::utils::round_timestamp;

thread_local! {
    static PRICES: RefCell<Vec<Price>> = RefCell::default();
    static PRICE_INDEXES: RefCell<HashMap<u32,u64>> = RefCell::new(HashMap::new());
    static PRICE_INDEX_INTERVAL_SEC: RefCell<u32> = RefCell::default();
    static FROM_SYNCED_TIMESTAMP: RefCell<u32> = RefCell::default();
    static FROM_PAST_SYNCED_TIMESTAMP: RefCell<u32> = RefCell::default();
    static TIMER_ID: RefCell<TimerId> = RefCell::default();
    static RPC_URL: RefCell<String> = RefCell::default();
    static POOL_ADDRESS: RefCell<String> = RefCell::default();
}

pub fn prices() -> Vec<Price> {
    PRICES.with(|val| val.borrow().clone())
}
pub fn last_price() -> Option<Price> {
    PRICES.with(|val| val.borrow().last().cloned())
}
pub fn prices_length() -> u64 {
    PRICES.with(|val| val.borrow().len()) as u64
}
pub fn price(idx: u64) -> Option<Price> {
    PRICES.with(|val| val.borrow().get(idx as usize).cloned())
}
pub fn add_price(price: Price) {
    PRICES.with(|val| val.borrow_mut().push(price));
}
pub fn clear_prices() {
    PRICES.with(|val| val.borrow_mut().clear());
}

pub fn get_price_index(timestamp: u32) -> Option<u64> {
    PRICE_INDEXES.with(|val| val.borrow().get(&timestamp).cloned())
}
pub fn get_closest_high_price_index(ts: u32, max: u32) -> Result<u64, String> {
    // at first, specified ts is used for key
    if let Some(value) = get_price_index(ts) {
        return Ok(value);
    }

    let index_interval = get_price_index_interval_sec();
    let mut current_ts = round_timestamp(ts + index_interval, index_interval);
    loop {
        let index = get_price_index(current_ts);
        if let Some(value) = index {
            return Ok(value);
        }
        current_ts += index_interval;
        if current_ts > max {
            break;
        }
    }
    Err(format!(
        "No price index found for timestamp {} - {}",
        ts, current_ts
    ))
}
pub fn get_closest_low_price_index(ts: u32, min: u32) -> Result<u64, String> {
    // at first, specified ts is used for key
    if let Some(value) = get_price_index(ts) {
        return Ok(value);
    }

    let index_interval = get_price_index_interval_sec();
    let mut current_ts = round_timestamp(ts, index_interval);
    loop {
        let index = get_price_index(current_ts);
        if let Some(value) = index {
            return Ok(value);
        }
        current_ts -= index_interval;
        if current_ts < min {
            break;
        }
    }
    Err(format!(
        "No price index found for timestamp {} - {}",
        ts, current_ts
    ))
}
pub fn insert_price_index(timestamp: u32, index: u64) {
    PRICE_INDEXES.with(|val| val.borrow_mut().insert(timestamp, index));
}
pub fn clear_price_indexes() {
    PRICE_INDEXES.with(|val| val.borrow_mut().clear());
}

pub fn get_price_index_interval_sec() -> u32 {
    PRICE_INDEX_INTERVAL_SEC.with(|val| *val.borrow())
}
pub fn set_price_index_interval_sec(value: u32) {
    PRICE_INDEX_INTERVAL_SEC.with(|val| *val.borrow_mut() = value);
}

pub fn get_from_synced_timestamp() -> u32 {
    FROM_SYNCED_TIMESTAMP.with(|val| *val.borrow())
}
pub fn set_from_synced_timestamp(value: u32) {
    FROM_SYNCED_TIMESTAMP.with(|val| *val.borrow_mut() = value);
}

pub fn get_from_past_synced_timestamp() -> u32 {
    FROM_PAST_SYNCED_TIMESTAMP.with(|val| *val.borrow())
}
pub fn set_from_past_synced_timestamp(value: u32) {
    FROM_PAST_SYNCED_TIMESTAMP.with(|val| *val.borrow_mut() = value);
}

pub fn get_timer_id() -> TimerId {
    TIMER_ID.with(|value| *value.borrow())
}
pub fn set_timer_id(timer_id: TimerId) {
    TIMER_ID.with(|value| *value.borrow_mut() = timer_id);
}

pub fn get_rpc_url() -> String {
    RPC_URL.with(|value| value.borrow().clone())
}
pub fn set_rpc_url(url: String) {
    RPC_URL.with(|value| *value.borrow_mut() = url);
}

pub fn get_pool_address() -> String {
    POOL_ADDRESS.with(|value| value.borrow().clone())
}
pub fn set_pool_address(pool_addr: String) {
    POOL_ADDRESS.with(|value| *value.borrow_mut() = pool_addr);
}
