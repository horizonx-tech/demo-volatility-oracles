pub const ORACLE_ABI: &[u8] = include_bytes!("../../abi/Oracle.json");
pub const ORACLE_FUNC_NAME: &str = "updateState";

// parameters
pub const MAX_RESP_TO_READ_SCALAR: u64 = 300;
pub const MAX_RESP_TO_SEND_TX: u64 = 500;
pub const TASK_INTERVAL_SECS: u32 = 24 * 60 * 60;
pub const SECS_TO_DELAY: u32 = 5 * 60;
pub const PRECISION_FOR_ORACLE: u8 = 18;
