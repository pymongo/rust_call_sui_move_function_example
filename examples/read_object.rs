#[tokio::main]
async fn main() {
    let is_testnet = true;
    let object_id = "0x6d08e394bcc4dec6a8349f1ffb4e5630c0cd55df1ba9882cfe66dfa5b1f7d130";
    let sui = sui_sdk::SuiClientBuilder::default()
        .build(format!(
            "https://fullnode.{}.sui.io:443",
            if is_testnet { "testnet" } else { "mainnet" }
        ))
        .await
        .unwrap();
    // get_owned_objects
    let r = sui
        .read_api()
        .get_object_with_options(
            object_id.parse().unwrap(),
            sui_sdk::rpc_types::SuiObjectDataOptions::full_content(),
        )
        .await
        .unwrap();
    let content = r.data.unwrap().content.unwrap();
    dbg!(content);
}
