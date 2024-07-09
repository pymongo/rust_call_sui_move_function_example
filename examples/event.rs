use futures::stream::StreamExt;
use sui_sdk::{rpc_types::EventFilter, types::base_types::ObjectID};

/// https://docs.sui.io/sui-api-ref#suix_subscribeevent
/// https://suiscan.xyz/mainnet/object/0x8faab90228e4c4df91c41626bbaefa19fc25c514405ac64de54578dec9e6f5ee/contracts
#[tokio::main]
async fn main() {
    // let is_testnet = false;
    let cetus_package_id = "0x8faab90228e4c4df91c41626bbaefa19fc25c514405ac64de54578dec9e6f5ee";
    let cetus_package_id = ObjectID::from_hex_literal(&cetus_package_id).unwrap();
    let sui_client = sui_sdk::SuiClientBuilder::default()
        .ws_url("wss://sui-mainnet.blockvision.org:443/v1/TODO")
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    // println!("WS version {:?}", sui_client.api_version());
    /*
    filter 不能用 PackageId
    参考文档 https://docs.sui.io/guides/developer/sui-101/using-events#filtering-event-queries
    // RpcError(Call(Custom(ErrorObject { code: InvalidParams, message: "Invalid params", data: None })))
    */
    // let filter = EventFilter::MoveEventModule {
    //     package: cetus_package_id,
    //     module: "router".parse().unwrap()
    // };
    sui_client.check_api_version().unwrap();
    let mut subscribe = sui_client
        .event_api()
        .subscribe_event(EventFilter::Package(cetus_package_id))
        .await
        .unwrap();
    // let data: Vec<_> = ws.event_api().get_events_stream(filter, None, true).collect().await;
    // println!("{:?}", data);
    
    while let Some(res) = subscribe.next().await {
        let res = res.unwrap();
        println!("{res:?}");
    }
    dbg!("EOF");
}
