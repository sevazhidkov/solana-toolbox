use serde_json::json;
use solana_sdk::pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlService;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Actually fetch our account using the auto-resolved IDL on-chain
    let address = pubkey!("FdoXZqdMysWbzB8j5bK6U5J1Dczsos1vGwQi5Tur2mwk");
    let decoded = ToolboxIdlService::new()
        .get_and_infer_and_decode_account(&mut endpoint, &address)
        .await
        .unwrap();
    // Check that the account was parsed properly and values matches
    assert_eq!(
        &decoded.state["state"]["metadata"]["vocab_size"],
        &json!(129280)
    );
    assert_eq!(
        &decoded.state["state"]["coordinator"]["config"]["min_clients"],
        &json!(24)
    );
}
