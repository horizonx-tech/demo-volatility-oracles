mod constants;
mod store;
mod types;
mod utils;

use candid::{candid_method, Principal};
use chainsight_generate::{did_export, manageable};
use constants::{TASK_INTERVAL_SECS, SECS_TO_DELAY};
use ic_cdk::{api::time, query, update};
use store::{set_target_canister, set_call_canister_args, add_snapshot_data, get_target_canister, get_call_canister_args, set_task_interval_secs, get_task_interval_secs};
use types::CallCanisterArgs;
use utils::round_timestamp;

use crate::{constants::PRECISION_FOR_ORACLE, store::set_timer_id};
use crate::store::SnapshotDataStruct;

manageable!();

#[update]
#[candid_method(update)]
fn setup(
    target_canister_id: String,
    data_resource_canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    time_unit: u32,
    back_terms: Option<u8>,
) {
    set_target_canister(Principal::from_text(target_canister_id).unwrap());
    set_call_canister_args(CallCanisterArgs {
        data_resource_canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        time_unit,
        back_terms,
    });
}

#[update]
#[candid_method(update)]
async fn set_task(task_interval_secs: Option<u32>, secs_to_delay: Option<u32>) {
    let task_interval_secs = task_interval_secs.unwrap_or(TASK_INTERVAL_SECS);
    set_task_interval_secs(task_interval_secs);
    let secs_to_delay = secs_to_delay.unwrap_or(SECS_TO_DELAY);

    let current_time_sec = (time() / (1000 * 1000000)) as u32;
    let delay =
        round_timestamp(current_time_sec, task_interval_secs) + task_interval_secs + secs_to_delay
            - current_time_sec;
    let interval = std::time::Duration::from_secs(task_interval_secs as u64);
    ic_cdk::println!("START: set_timer for set_timer_interval");
    ic_cdk::println!("{}", current_time_sec);
    ic_cdk_timers::set_timer(std::time::Duration::from_secs(delay as u64), move || {
        ic_cdk::println!("START: set_timer_interval for call sync_state");
        ic_cdk::println!("{}", (time() / (1000 * 1000000)));

        // set scheduled executions timer for 2nd and later
        let timer_id = ic_cdk_timers::set_timer_interval(interval, || {
            ic_cdk::spawn(async {
                ic_cdk::println!("START: execute update_state by timer");
                ic_cdk::println!("{}", (time() / (1000 * 1000000)));
                match update_state(
                    round_timestamp((time() / (1000 * 1000000)) as u32, get_task_interval_secs()) as u64,
                    get_target_canister(),
                    get_call_canister_args(),
                    PRECISION_FOR_ORACLE,
                )
                .await
                {
                    Ok(msg) => ic_cdk::println!("ok: {:?}", msg),
                    Err(msg) => ic_cdk::println!("err: {:?}", msg),
                };
                ic_cdk::println!("FINISH: execute update_state by timer");
            });
        });
        set_timer_id(timer_id);

        // for 1st
        ic_cdk::spawn(async move {
            match update_state(
                round_timestamp((time() / (1000 * 1000000)) as u32, task_interval_secs) as u64,
                get_target_canister(),
                get_call_canister_args(),
                PRECISION_FOR_ORACLE,
            )
            .await
            {
                Ok(msg) => ic_cdk::println!("ok: {:?}", msg),
                Err(msg) => ic_cdk::println!("err: {:?}", msg),
            };
        });
    });

    // for one previous
    // prerequisite: transfer native token to this canister's address
    let previous_data_time = round_timestamp(current_time_sec, task_interval_secs);
    match update_state(
        previous_data_time as u64,
        get_target_canister(),
        get_call_canister_args(),
        PRECISION_FOR_ORACLE,
    )
    .await
    {
        Ok(msg) => ic_cdk::println!("ok: {:?}", msg),
        Err(msg) => ic_cdk::println!("err: {:?}", msg),
    };
}

async fn update_state(
    execution_time: u64,
    canister_id: Principal,
    call_args: CallCanisterArgs,
    precision_to_sync: u8,
) -> Result<(), String> {
    let result: Result<String, String> =
        call_get_period_range_realized_volatility(canister_id, call_args).await;
    if let Err(msg) = result {
        return Err(format!("error msg by inter-canister call: {:?}", msg));
    }
    let parsed_result = result.unwrap().parse::<f64>();
    if let Err(msg) = parsed_result {
        return Err(format!("error msg by parsing result: {:?}", msg));
    }
    let value = (parsed_result.unwrap() * 10u64.pow(precision_to_sync as u32) as f64) as u128;
    add_snapshot_data(execution_time, value);
    Ok(())
}

async fn call_get_period_range_realized_volatility(
    canister_id: Principal,
    call_args: CallCanisterArgs,
) -> Result<String, String> {
    let res = ic_cdk::api::call::call::<_, (Result<String, String>,)>(
        canister_id,
        "get_period_range_realized_volatility",
        (
            call_args.data_resource_canister_id,
            call_args.token0_decimals,
            call_args.token1_decimals,
            call_args.precision,
            call_args.time_unit,
            call_args.back_terms,
        ),
    )
    .await
    .map_err(|e| format!("Error calling get_period_range_realized_volatility: {:?}", e))?;
    res.0
        .map_err(|e| format!("Error calling get_period_range_realized_volatility: {:?}", e))
}

did_export!("interface");
