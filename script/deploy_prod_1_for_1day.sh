# dfx stop && dfx start --background --clean # for local

# deploy
dfx deploy snapshotter --network mainnet
dfx deploy calculator --network mainnet
dfx deploy relayer --network mainnet

# initialize snapshotter
dfx canister call snapshotter setup '(
    "<YOUR_RPC_KEY_FOR_MAINNET",
    "88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    opt 21600,
)' --network mainnet

dfx canister call snapshotter bulk_save_prices '(
    vec {
        17212082;
        17212381;
        17212669;
        17212963;
        17213259;
        17213559;
        17213859;
        17214157;
        17214450;
        17214749
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter bulk_save_prices '(
    vec {
        17215050;
        17215355;
        17215641;
        17215937;
        17216229;
        17216526;
        17216823;
        17217121;
        17217413;
        17217713
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter bulk_save_prices '(
    vec {
        17218008;
        17218304;
        17218605;
        17218898;
        17219197;
        17219497;
        17219788;
        17220102;
        17220383;
        17220682
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter bulk_save_prices '(
    vec {
        17220977;
        17221288;
        17221569;
        17221877;
        17222163;
        17222460;
        17222760
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter set_task '(
    opt 3600,
    null,
    null
)' --network mainnet

# initialize relayer
dfx canister call relayer setup "(
    \"<YOUR_RPC_KEY_FOR_POLYGON_MUMBAI\",
    80001,
    \"E5f0DA5761B82e14E45021246EE657D07a9BBd23\",
    \"$(dfx canister id calculator --network mainnet)\",
    \"$(dfx canister id snapshotter --network mainnet)\",
    6,
    18,
    10,
    86400,
    null
)" --network mainnet
## prerequisites: transfer native token to relayer
dfx canister call relayer set_task "(
    opt 86400,
    opt 300
)" --network mainnet
