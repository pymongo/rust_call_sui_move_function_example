#![allow(unused_imports)]
use std::str::FromStr;

use sui_keys::keystore::AccountKeystore;
use sui_sdk::{
    rpc_types::{SuiObjectDataOptions, SuiTransactionBlockResponseOptions},
    types::{
        base_types::{ObjectID, SuiAddress},
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{
            Argument, CallArg, Command, ObjectArg, ProgrammableMoveCall, Transaction,
            TransactionData, TransactionKind,
        },
        type_input::{StructInput, TypeInput},
        Identifier, TypeTag,
    },
};

#[tokio::main]
async fn main() {
    let is_testnet = true;
    // let my_addr = "0x520c89c6c78c566eed0ebf24f854a8c22d8fdd06a6f16ad01f108dad7f1baaea";
    // let object_id = "0xcbf4748a965d469ea3a36cf0ccc5743b96c2d0ae6dee0762ed3eca65fac07f7e";
    let sui = sui_sdk::SuiClientBuilder::default()
        .build(format!(
            "https://fullnode.{}.sui.io:443",
            if is_testnet { "testnet" } else { "mainnet" }
        ))
        .await
        .unwrap();
    dbg!(sui.check_api_version().unwrap());

    // find_gas_coin: we need to find the coin we will use as gas

    // 2) create a programmable transaction builder to add commands and create a PTB
    // A PTB can perform up to 1,024 unique operations in a single execution
    // https://docs.sui.io/concepts/transactions/prog-txn-blocks

    // Create an Argument::Input for Pure 6 value of type u64
    // let object_id: ObjectID = object_id.parse().unwrap();
    // https://docs.sui.io/concepts/object-model
    // object fields: object_id,version(),last_tx_digest(作用类似于eth账户的nonce),owner_addr
    // https://suiscan.xyz/testnet/object/0x6d08e394bcc4dec6a8349f1ffb4e5630c0cd55df1ba9882cfe66dfa5b1f7d130
    // let version = SequenceNumber::from_u64(627544);
    // let obj = sui
    //     .read_api()
    //     .get_object_with_options(object_id, SuiObjectDataOptions::bcs_lossless())
    //     .await
    //     .unwrap()
    //     .data
    //     .unwrap();
    //  Base64 string representing the object digest
    // let input_argument = CallArg::Object(ObjectArg::ImmOrOwnedObject((object_id, version, object_last_tx)));
    // The object digest is the hash of the object's contents and metadata

    // let input_argument = CallArg::Object(ObjectArg::ImmOrOwnedObject((
    //     obj.object_id,
    //     obj.version,
    //     obj.digest,
    // )));
    // Add this input to the builder
    // ptb.input(input_argument).unwrap();
    // ptb.input(CallArg::Pure(bcs::to_bytes(&"item 4").unwrap()))
    //     .unwrap();
    // 3) add a move call to the PTB
    // Replace the pkg_id with the package id you want to call
    let pkg_id = "0xcbf4748a965d469ea3a36cf0ccc5743b96c2d0ae6dee0762ed3eca65fac07f7e";
    let package = ObjectID::from_hex_literal(pkg_id).unwrap();
    let module = Identifier::new("pool").unwrap();
    let function = Identifier::new("get_level2_range").unwrap();
    let price_low = 100000u64;
    let price_high = 10000000u64;
    let is_bid = true;
    let poolobj = sui
        .read_api()
        .get_object_with_options(
            "0x520c89c6c78c566eed0ebf24f854a8c22d8fdd06a6f16ad01f108dad7f1baaea"
                .parse()
                .unwrap(),
            SuiObjectDataOptions::bcs_lossless(),
        )
        .await
        .unwrap()
        .data
        .unwrap();
    let clock_object = sui
        .read_api()
        .get_object_with_options("0x6".parse().unwrap(), SuiObjectDataOptions::bcs_lossless())
        .await
        .unwrap()
        .data
        .unwrap();
    // ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject((
    //     poolobj.object_id,
    //     poolobj.version,
    //     poolobj.digest,
    // ))))
    // .unwrap();

    // let mut builder = Transaction::new(sender, gas_budget);

    /*
    public fun get_level2_range<BaseAsset, QuoteAsset>(
        self: &Pool<BaseAsset, QuoteAsset>,
        price_low: u64,
        price_high: u64,
        is_bid: bool,
        clock: &Clock,
    ): (vector<u64>, vector<u64>) {*/    
    let mut ptb = ProgrammableTransactionBuilder::new();    
    // let a1 = ptb
        // .obj(ObjectArg::ImmOrOwnedObject((
        //     poolobj.object_id,
        //     poolobj.version,
        //     poolobj.digest,
        // )))
    //     .unwrap();
    // let a2 = ptb.pure(price_low).unwrap();
    // let a3 = ptb.pure(price_high).unwrap();
    // let a4 = ptb.pure(is_bid).unwrap();
    // let a5 = ptb
    //     .obj(ObjectArg::ImmOrOwnedObject((
    //         clock_object.object_id,
    //         clock_object.version,
    //         clock_object.digest,
    //     )))
    //     .unwrap();
    let type_args = vec![
        // TypeTag::from_str("0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI").unwrap(),
        TypeInput::Struct(Box::new(StructInput { 
            address: "0x2".parse().unwrap(), 
            module: "sui".to_string(), 
            name: "SUI".to_string(), 
            type_params: Vec::new() 
        })),
        // TypeTag::from_str("0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDC::DBUSDC").unwrap()
        TypeInput::Struct(Box::new(StructInput { 
            address: "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7".parse().unwrap(), 
            module: "DBUSDC".to_string(), 
            name: "DBUSDC".to_string(), 
            type_params: Vec::new() 
        })),
    ];
    ptb.move_call(
        package,
        module,
        function,
        vec![
            TypeTag::from_str("0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI").unwrap(),
            TypeTag::from_str("0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDC::DBUSDC").unwrap()
        ],
        vec![
            CallArg::Object(ObjectArg::ImmOrOwnedObject((
                poolobj.object_id,
                poolobj.version,
                poolobj.digest,
            ))),
            CallArg::Pure(bcs::to_bytes(&price_low).unwrap()),
            CallArg::Pure(bcs::to_bytes(&price_high).unwrap()),
            CallArg::Pure(bcs::to_bytes(&is_bid).unwrap()),
            CallArg::Object(ObjectArg::ImmOrOwnedObject((
                clock_object.object_id,
                clock_object.version,
                clock_object.digest,
            ))),
        ],
    ).unwrap();
    // ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    //     package,
    //     module: "pool".to_string(),
    //     function: "get_level2_range".to_string(),
        // type_arguments: vec![
        //     // TypeTag::from_str("0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI").unwrap(),
        //     TypeInput::Struct(Box::new(StructInput { 
        //         address: "0x2".parse().unwrap(), 
        //         module: "sui".to_string(), 
        //         name: "SUI".to_string(), 
        //         type_params: Vec::new() 
        //     })),
        //     // TypeTag::from_str("0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDC::DBUSDC").unwrap()
        //     TypeInput::Struct(Box::new(StructInput { 
        //         address: "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7".parse().unwrap(), 
        //         module: "DBUSDC".to_string(), 
        //         name: "DBUSDC".to_string(), 
        //         type_params: Vec::new() 
        //     })),
        // ],
    //     arguments: vec![
    //         a1,
    //         a2,
    //         a3,
    //         a4,
    //         a5,
    //     ],
    //     // arguments: vec![
    //     //     Argument::Input(1),
    //     //     Argument::Input(2),                 // 第二个参数: 价格低限
    //     //     Argument::Input(3),                // 第三个参数: 价格高限
    //     //     // Argument::Input(3),                    // 第四个参数: 是否为买单
    //     //     // Argument::Input(4),     // 第五个参数: 时钟对象 ID
    //     // ],
    // })));
    // build the transaction block by calling finish on the ptb
    let ptbok = ptb.finish();
    // dbg!(&ptbok.inputs);
    let txbytes = bcs::to_bytes(&ptbok).unwrap();
    dbg!(txbytes.len());
    println!("{}", hex::encode(&txbytes));
    return;

    // 1SUI as budget
    // let gas_budget = 10u64.pow(9) / 10;
    // let gas_price = sui.read_api().get_reference_gas_price().await.unwrap();
    // // create the transaction data that will be sent to the network
    // let tx_data = TransactionData::new_programmable(
    //     sender,
    //     vec![sui_sdk_example::gas_coin_obj_ref(is_testnet)],
    //     ptbok,
    //     gas_budget,
    //     gas_price,
    // );
    // dbg!(Transaction::from_data(tx_data, vec![]).to_tx_bytes_and_signatures());
    let r = sui
        .read_api()
        .dev_inspect_transaction_block(
            my_addr.parse().unwrap(),
            TransactionKind::ProgrammableTransaction(ptbok),
            None,
            None,
            None,
        )
        .await
        .unwrap();
    if let Some(err) = r.error {
        panic!("{}", err);
    }
    let rr = &r.results.unwrap()[0].return_values;
    dbg!(rr);

    // 4) sign transaction
    // let serialize_tx_data = bcs::to_bytes(&tx_data).unwrap();
    // use blake2::Digest;
    // let mut hasher = blake2::Blake2b::<sha2::digest::typenum::U32>::new();
    // hasher.update(&serialize_tx_data);
    // let tx_data_digest = hasher.finalize();
    // let private_key = fastcrypto::ed25519::Ed25519KeyPair::from_str("TODO").unwrap();
    // let keystore = sui_sdk_example::get_keystore();
    // let signature = keystore
    //     .sign_secure(
    //         &sender,
    //         &tx_data,
    //         shared_crypto::intent::Intent::sui_transaction(),
    //     )
    //     .unwrap();
    // // let signature = Signature::new_hashed(&tx_data_digest, &key);

    // // 5) execute the transaction
    // let transaction_response = sui
    //     .quorum_driver_api()
    //     .execute_transaction_block(
    //         Transaction::from_data(tx_data, vec![signature]),
    //         SuiTransactionBlockResponseOptions::full_content(),
    //         Some(ExecuteTransactionRequestType::WaitForLocalExecution),
    //     )
    //     .await
    //     .unwrap();
    // println!("{}", transaction_response);
}
