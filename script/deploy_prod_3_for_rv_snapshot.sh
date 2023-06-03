# dfx stop && dfx start --background --clean # for local

# deploy
dfx deploy rv_snapshot --network mainnet
dfx deploy rv_snapshot_2 --network mainnet

# initialize rv_snapshot (for daily rv)
dfx canister call rv_snapshot setup "(
    \"$(dfx canister id calculator --network mainnet)\",
    \"$(dfx canister id snapshotter --network mainnet)\",
    6,
    18,
    10,
    86400,
    null
)" --network mainnet
dfx canister call rv_snapshot set_task "(
    opt 86400,
    opt 300
)" --network mainnet

# initialize rv_snapshot (for 4week rv)
dfx canister call rv_snapshot_2 setup "(
    \"$(dfx canister id calculator --network mainnet)\",
    \"$(dfx canister id snapshotter_2 --network mainnet)\",
    6,
    18,
    10,
    2419200,
    null
)" --network mainnet
dfx canister call rv_snapshot_2 set_task "(
    opt 2419200,
    opt 300
)" --network mainnet
