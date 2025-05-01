use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Create the endpoint pointing to devnet
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Check that we can fetch unix timestamps of slots
    assert_eq!(
        endpoint.get_slot_unix_timestamp(331437116).await.unwrap(),
        1728376873
    );
    assert_eq!(
        endpoint.get_slot_unix_timestamp(356222939).await.unwrap(),
        1737689741
    );
    assert_eq!(
        endpoint.get_slot_unix_timestamp(364175225).await.unwrap(),
        1740748248
    );
}
