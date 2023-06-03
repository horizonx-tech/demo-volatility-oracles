mod constants;
mod types;
mod utils;

use candid::{Principal, candid_method};
use chainsight_generate::{did_export, manageable};
use constants::{DAY_SECONDS, HOUR_SECONDS, WEEK_SECONDS};
use ic_cdk::{update, query, api::call};
use ic_web3::types::U256;
use types::CandidPrice;
use utils::{calculate_realized_volatility, current_time_sec, round_timestamp};

manageable!();

#[update]
#[candid_method(update)]
async fn get_realized_volatility(
    canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    from: Option<u32>,
    to: Option<u32>,
) -> Result<String, String> {
    let canister_id = Principal::from_text(canister_id).unwrap();
    let result = calculate_realized_volatility_for_prices(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        from,
        to,
    )
    .await?;
    Ok(result.to_string())
}

#[update]
#[candid_method(update)]
async fn get_period_range_realized_volatility(
    canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    time_unit: u32,
    back_terms: Option<u8>,
) -> Result<String, String> {
    let mut rounded_current_time = round_timestamp(current_time_sec(), time_unit);
    if let Some(value) = back_terms {
        rounded_current_time -= value as u32 * time_unit;
    }
    ic_cdk::println!("from: {}", rounded_current_time - time_unit);
    ic_cdk::println!("to: {}", rounded_current_time);
    get_realized_volatility(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        Some(rounded_current_time - time_unit),
        Some(rounded_current_time),
    )
    .await
}

#[update]
#[candid_method(update)]
async fn get_last_4week_realized_volatility(
    canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    back_terms: Option<u8>,
) -> Result<String, String> {
    get_period_range_realized_volatility(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        4 * WEEK_SECONDS,
        back_terms,
    )
    .await
}

#[update]
#[candid_method(update)]
async fn get_last_day_realized_volatility(
    canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    back_terms: Option<u8>,
) -> Result<String, String> {
    get_period_range_realized_volatility(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        DAY_SECONDS,
        back_terms,
    )
    .await
}

#[update]
#[candid_method(update)]
async fn get_last_hour_realized_volatility(
    canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    back_terms: Option<u8>,
) -> Result<String, String> {
    get_period_range_realized_volatility(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        HOUR_SECONDS,
        back_terms,
    )
    .await
}

async fn calculate_realized_volatility_for_prices(
    canister_id: Principal,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    from: Option<u32>,
    to: Option<u32>,
) -> Result<f64, String> {
    let exchange_rates = calculate_exchange_rates_for_prices(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        from,
        to,
    )
    .await?;

    let rv = calculate_realized_volatility(&exchange_rates);

    Ok(rv)
}

async fn calculate_exchange_rates_for_prices(
    canister_id: Principal,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    from: Option<u32>,
    to: Option<u32>,
) -> Result<Vec<U256>, String> {
    let prices = call_prices(canister_id, from, to).await?;
    let mut exchange_rates = Vec::with_capacity(prices.len());
    for price in prices {
        let sqrt_price_x96 = U256::from_dec_str(&price.sqrt_price_x96).map_err(|e| {
            format!(
                "Error parsing sqrt_price_x96: {:?} for price: {:?}",
                e, price
            )
        })?;

        let exchange_rate = utils::calculate_exchange_rate(
            sqrt_price_x96,
            token0_decimals,
            token1_decimals,
            precision,
        );
        exchange_rates.push(exchange_rate);
    }
    Ok(exchange_rates)
}

async fn call_prices(
    canister_id: Principal,
    from: Option<u32>,
    to: Option<u32>,
) -> Result<Vec<CandidPrice>, String> {
    let res =
        call::call::<_, (Result<Vec<CandidPrice>, String>,)>(canister_id, "get_prices", (from, to))
            .await
            .map_err(|e| format!("Error calling get_prices: {:?}", e))?;
    res.0
        .map_err(|e| format!("Error calling get_prices: {:?}", e))
}

did_export!("interface");
