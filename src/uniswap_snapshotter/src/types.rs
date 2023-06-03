use candid::CandidType;
use ic_web3::{
    contract::{tokens::Detokenize, Error},
    ethabi::Token,
    types::U256,
};
pub struct Slot0(
    pub U256,
    pub i32,
    pub u16,
    pub u16,
    pub u16,
    pub u8,
    pub bool,
);

impl Detokenize for Slot0 {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self(
            tokens.get(0).unwrap().clone().into_uint().unwrap(),
            tokens.get(1).unwrap().clone().into_int().unwrap().as_u128() as i32,
            tokens.get(2).unwrap().clone().into_uint().unwrap().as_u32() as u16,
            tokens.get(3).unwrap().clone().into_uint().unwrap().as_u32() as u16,
            tokens.get(4).unwrap().clone().into_uint().unwrap().as_u32() as u16,
            tokens.get(5).unwrap().clone().into_uint().unwrap().as_u32() as u8,
            tokens.get(6).unwrap().clone().into_bool().unwrap(),
        ))
    }
}

impl Slot0 {
    pub fn to_candid(&self) -> CandidSlot0 {
        CandidSlot0 {
            sqrt_price_x96: self.0.to_string(),
            tick: self.1,
            observation_index: self.2,
            observation_cardinality: self.3,
            observation_cardinality_next: self.4,
            fee_protocol: self.5,
            unlocked: self.6,
        }
    }
}

#[derive(CandidType)]
pub struct CandidSlot0 {
    sqrt_price_x96: String,
    tick: i32,
    observation_index: u16,
    observation_cardinality: u16,
    observation_cardinality_next: u16,
    fee_protocol: u8,
    unlocked: bool,
}

pub struct Observation(pub u32, pub i64, pub U256, pub bool);

impl Detokenize for Observation {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self(
            tokens.get(0).unwrap().clone().into_uint().unwrap().as_u32(),
            tokens.get(1).unwrap().clone().into_int().unwrap().as_u128() as i64,
            tokens.get(2).unwrap().clone().into_uint().unwrap(),
            tokens.get(3).unwrap().clone().into_bool().unwrap(),
        ))
    }
}

impl Observation {
    pub fn to_candid(&self) -> CandidObservation {
        CandidObservation {
            block_timestamp: self.0,
            tick_cumulative: self.1,
            liquidity_cumulative: self.2.to_string(),
            initialized: self.3,
        }
    }
}

#[derive(CandidType)]
pub struct CandidObservation {
    pub block_timestamp: u32,
    pub tick_cumulative: i64,
    pub liquidity_cumulative: String,
    pub initialized: bool,
}

#[derive(Clone, Debug)]
pub struct Price {
    pub sqrt_price_x96: U256,
    pub observation_index: u16,
    pub block_timestamp: u32,
}
impl Price {
    pub fn to_candid(&self) -> CandidPrice {
        CandidPrice {
            sqrt_price_x96: self.sqrt_price_x96.to_string(),
            observation_index: self.observation_index,
            block_timestamp: self.block_timestamp,
        }
    }
}
#[derive(CandidType, Debug, PartialEq)]
pub struct CandidPrice {
    pub sqrt_price_x96: String,
    pub observation_index: u16,
    pub block_timestamp: u32,
}
