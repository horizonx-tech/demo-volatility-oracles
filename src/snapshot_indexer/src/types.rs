use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CallCanisterArgs {
    pub data_resource_canister_id: String,
    pub token0_decimals: u8,
    pub token1_decimals: u8,
    pub precision: u8,
    pub time_unit: u32,
    pub back_terms: Option<u8>,
}