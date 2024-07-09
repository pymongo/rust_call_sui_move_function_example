use futures::stream::StreamExt;
use sui_sdk::{rpc_types::EventFilter, types::base_types::ObjectID};

/// https://docs.sui.io/sui-api-ref#suix_subscribeevent
/// https://suiscan.xyz/mainnet/object/0x8faab90228e4c4df91c41626bbaefa19fc25c514405ac64de54578dec9e6f5ee/contracts
#[tokio::main]
async fn main() {
    // let is_testnet = false;
    let cetus_package_id = "0x8faab90228e4c4df91c41626bbaefa19fc25c514405ac64de54578dec9e6f5ee";
    let cetus_package_id = ObjectID::from_hex_literal(&cetus_package_id).unwrap();
    let ws = sui_sdk::SuiClientBuilder::default()
        .ws_url("wss://rpc.mainnet.sui.io:443")
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    let mut subscribe = ws
        .event_api()
        .subscribe_event(EventFilter::Package(cetus_package_id))
        .await
        .unwrap();
    while let Some(res) = subscribe.next().await {
        let res = res.unwrap();
        println!("{res:?}");
    }
    dbg!("EOF");
}
