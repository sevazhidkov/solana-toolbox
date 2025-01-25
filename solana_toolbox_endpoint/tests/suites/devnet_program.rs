use solana_sdk::pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use std::fs::read;

#[tokio::test]
pub async fn run() {
    // Create the endpoint pointing to devnet
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Fetch a program's bytecode
    let program_id = pubkey!("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j");
    let program_bytecode = endpoint
        .get_program_bytecode_from_program_id(&program_id)
        .await
        .unwrap()
        .unwrap();
    // Check that the bytecode match the expected value
    assert_eq!(
        read("./tests/fixtures/UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j.so")
            .unwrap(),
        program_bytecode
    );
}
