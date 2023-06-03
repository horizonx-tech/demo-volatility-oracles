pub fn round_timestamp(timestamp: u32, unit: u32) -> u32 {
    timestamp / unit * unit
}
