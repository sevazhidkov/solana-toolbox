use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Check that we can fetch slot's timestamp
    endpoint.get_slot_unix_timestamp(1).await.unwrap();
    endpoint.forward_clock_slot(4).await.unwrap();
    endpoint.get_slot_unix_timestamp(5).await.unwrap();
}
