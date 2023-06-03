# dfx stop && dfx start --background --clean # for local

# deploy
dfx deploy snapshotter_2
dfx deploy relayer_2

# initialize snapshotter
dfx canister call snapshotter_2 setup '(
    "<YOUR_RPC_KEY_FOR_MAINNET",
    "88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    opt 604800,
)' --network mainnet
dfx canister call snapshotter_2 bulk_save_prices '(
    vec {
        16815369;
        16822503;
        16829615;
        16836713;
        16843835;
        16850948;
        16858069;
        16865183;
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter_2 bulk_save_prices '(
    vec {
        16872318;
        16879430;
        16886553;
        16893661;
        16900789;
        16907908;
        16915030;
        16922142;
        16929250;
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter_2 bulk_save_prices '(
    vec {
        16936384;
        16943493;
        16950604;
        16957736;
        16964816;
        16971894;
        16978915;
        16985960;
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter_2 bulk_save_prices '(
    vec {
        16992952;
        17000010;
        17007072;
        17014117;
        17021181;
        17028259;
        17035258;
        17041766;
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter_2 bulk_save_prices '(
    vec {
        17048777;
        17055793;
        17062845;
        17069912;
        17076967;
        17084049;
        17091086;
        17098174;
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter_2 bulk_save_prices '(
    vec {
        17105313;
        17112444;
        17119603;
        17126696;
        17133820;
        17140939;
        17148051;
        17155179;
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter_2 bulk_save_prices '(
    vec {
        17162288;
        17169398;
        17176523;
        17183620;
        17190743;
        17197869;
        17205006;
        17212082;
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter_2 bulk_save_prices '(
    vec {
        17219196;
    },
    null,
    null
)' --network mainnet
dfx canister call snapshotter_2 set_task '(
    opt 86400,
    null,
    null
)' --network mainnet

# initialize relayer
## prerequisites: transfer native token to relayer
dfx canister call relayer_2 setup "(
    \"<YOUR_RPC_KEY_FOR_POLYGON_MUMBAI\",
    80001,
    \"E5f0DA5761B82e14E45021246EE657D07a9BBd23\",
    \"$(dfx canister id calculator --network mainnet)\",
    \"$(dfx canister id snapshotter_2 --network mainnet)\",
    6,
    18,
    10,
    2419200,
    null
)" --network mainnet
dfx canister call relayer_2 set_task "(
    opt 2419200,
    opt 300
)" --network mainnet
