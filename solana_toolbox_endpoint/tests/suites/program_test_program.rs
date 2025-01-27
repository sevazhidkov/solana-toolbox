use std::fs::read;

use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Make a payer
    let payer = Keypair::new();
    endpoint.process_airdrop(&payer.pubkey(), 10_000_000_000).await.unwrap();
    // Program details
    let program_id = Keypair::new();
    let program_authority = Keypair::new();
    let program_bytecode =
        read("./tests/fixtures/bpf_loader_program_minimal.so").unwrap();
    // Create a buffer
    let program_buffer = endpoint
        .process_program_buffer_new(
            &payer,
            &program_bytecode,
            &program_authority.pubkey(),
        )
        .await
        .unwrap();
    // Close the buffer
    endpoint
        .process_program_buffer_close(
            &payer,
            &program_buffer,
            &program_authority,
            &payer.pubkey(),
        )
        .await
        .unwrap();
    // Create the program while it doesnt exist yet
    endpoint
        .process_program_deploy(
            &payer,
            &program_id,
            &program_authority,
            &program_bytecode,
        )
        .await
        .unwrap();
    // Fetch the program and check that it matches expected bytecode
    assert_eq!(
        program_bytecode,
        endpoint
            .get_program_data(&program_id.pubkey())
            .await
            .unwrap()
            .unwrap()
            .bytecode
    );
    // Wait a slot to be able to interact with the program again
    endpoint.forward_clock_slot(1).await.unwrap();
    // Upgrade the program
    endpoint
        .process_program_upgrade(
            &payer,
            &program_id.pubkey(),
            &program_authority,
            &program_bytecode,
            &payer.pubkey(),
        )
        .await
        .unwrap();
    // Fetch the program and check that it matches expected bytecode
    assert_eq!(
        program_bytecode,
        endpoint
            .get_program_data(&program_id.pubkey())
            .await
            .unwrap()
            .unwrap()
            .bytecode
    );
    // Wait a slot to be able to interact with the program again
    endpoint.forward_clock_slot(1).await.unwrap();
    // Close the whole program
    endpoint
        .process_program_close(
            &payer,
            &program_id.pubkey(),
            &program_authority,
            &payer.pubkey(),
        )
        .await
        .unwrap();
}
