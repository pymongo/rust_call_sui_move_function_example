use sui_keys::keystore::AccountKeystore;
use sui_sdk::{
    rpc_types::{SuiObjectDataOptions, SuiTransactionBlockResponseOptions},
    types::{
        base_types::{ObjectID, SuiAddress},
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{
            Argument, CallArg, Command, ObjectArg, ProgrammableMoveCall, Transaction,
            TransactionData,
        },
        Identifier,
    },
};

#[tokio::main]
async fn main() {
    let is_testnet = true;
    let my_addr = "0x0eadd1cb51d89736bdc676e775ea575ee11210c5dc68a399df62c994a5429ee0";
    let object_id = "0x6d08e394bcc4dec6a8349f1ffb4e5630c0cd55df1ba9882cfe66dfa5b1f7d130";
    let sui = sui_sdk::SuiClientBuilder::default()
        .build(format!(
            "https://fullnode.{}.sui.io:443",
            if is_testnet { "testnet" } else { "mainnet" }
        ))
        .await
        .unwrap();
    let sender: SuiAddress = my_addr.parse().unwrap();

    // find_gas_coin: we need to find the coin we will use as gas

    // 2) create a programmable transaction builder to add commands and create a PTB
    // A PTB can perform up to 1,024 unique operations in a single execution
    // https://docs.sui.io/concepts/transactions/prog-txn-blocks
    let mut ptb = ProgrammableTransactionBuilder::new();

    // Create an Argument::Input for Pure 6 value of type u64
    let object_id: ObjectID = object_id.parse().unwrap();
    // https://docs.sui.io/concepts/object-model
    // object fields: object_id,version(),last_tx_digest(作用类似于eth账户的nonce),owner_addr
    // https://suiscan.xyz/testnet/object/0x6d08e394bcc4dec6a8349f1ffb4e5630c0cd55df1ba9882cfe66dfa5b1f7d130
    // let version = SequenceNumber::from_u64(627544);
    let obj = sui
        .read_api()
        .get_object_with_options(object_id, SuiObjectDataOptions::bcs_lossless())
        .await
        .unwrap()
        .data
        .unwrap();
    //  Base64 string representing the object digest
    // let input_argument = CallArg::Object(ObjectArg::ImmOrOwnedObject((object_id, version, object_last_tx)));
    // The object digest is the hash of the object's contents and metadata

    let input_argument = CallArg::Object(ObjectArg::ImmOrOwnedObject((
        obj.object_id,
        obj.version,
        obj.digest,
    )));
    // Add this input to the builder
    ptb.input(input_argument).unwrap();
    ptb.input(CallArg::Pure(bcs::to_bytes(&"item 4").unwrap()))
        .unwrap();
    // 3) add a move call to the PTB
    // Replace the pkg_id with the package id you want to call
    let pkg_id = "0x702815e66354365ec77e0ea708912725be4e8e0407b041e11f3b3f733c2a4a53";
    let package = ObjectID::from_hex_literal(pkg_id).unwrap();
    let module = Identifier::new("todo_list").unwrap();
    let function = Identifier::new("add").unwrap();
    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module,
        function,
        type_arguments: vec![],
        arguments: vec![Argument::Input(0), Argument::Input(1)],
    })));

    // build the transaction block by calling finish on the ptb
    let builder = ptb.finish();

    // 1SUI as budget
    let gas_budget = 10u64.pow(9) / 10;
    let gas_price = sui.read_api().get_reference_gas_price().await.unwrap();
    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![sui_sdk_example::gas_coin_obj_ref(is_testnet)],
        builder,
        gas_budget,
        gas_price,
    );

    // 4) sign transaction
    // let serialize_tx_data = bcs::to_bytes(&tx_data).unwrap();
    // use blake2::Digest;
    // let mut hasher = blake2::Blake2b::<sha2::digest::typenum::U32>::new();
    // hasher.update(&serialize_tx_data);
    // let tx_data_digest = hasher.finalize();
    // let private_key = fastcrypto::ed25519::Ed25519KeyPair::from_str("TODO").unwrap();
    let keystore = sui_sdk_example::get_keystore();
    let signature = keystore
        .sign_secure(
            &sender,
            &tx_data,
            shared_crypto::intent::Intent::sui_transaction(),
        )
        .unwrap();
    // let signature = Signature::new_hashed(&tx_data_digest, &key);

    // 5) execute the transaction
    print!("Executing the transaction...");
    let transaction_response = sui
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await
        .unwrap();
    println!("{}", transaction_response);
}
