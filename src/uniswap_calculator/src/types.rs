use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct CandidPrice {
    pub sqrt_price_x96: String,
    pub observation_index: u16,
    pub block_timestamp: u32,
}
