use std::fs::read;

use solana_sdk::{signature::Keypair, signer::Signer};
use solana_sdk::{pubkey::Pubkey};
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
    let program_bytecode =
        read("./tests/fixtures/bpf_loader_program_minimal.so").unwrap();
    // Controlling wallets
    let program_authority = Keypair::new();
    let spill = Pubkey::new_unique();
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
            &spill,
        )
        .await
        .unwrap();
    // Recreate another buffer
    let program_buffer = endpoint
        .process_program_buffer_new(
            &payer,
            &program_bytecode,
            &program_authority.pubkey(),
        )
        .await
        .unwrap();
    // Use the re-created buffer to deploy the program
    endpoint
        .process_program_deploy(
            &payer,
            &program_id,
            &program_buffer,
            &program_authority,
            program_bytecode.len(),
        )
        .await
        .unwrap();
    // Fetch the program and check that it matches expected bytecode
    assert_eq!(
        program_bytecode,
        endpoint
            .get_program_bytecode_from_program_id(&program_id.pubkey())
            .await
            .unwrap()
            .unwrap()
    );
    // Wait a block to be able to re-deploy
    endpoint.forward_clock_slot(1).await.unwrap();
    // Recreate another buffer
    let program_buffer = endpoint
        .process_program_buffer_new(
            &payer,
            &program_bytecode,
            &program_authority.pubkey(),
        )
        .await
        .unwrap();
    // Use the re-created buffer to upgrade the program
    endpoint
        .process_program_upgrade(
            &payer,
            &program_id.pubkey(),
            &program_buffer,
            &program_authority,
            &spill,
        )
        .await
        .unwrap();
    // Fetch the program and check that it matches expected bytecode
    assert_eq!(
        program_bytecode,
        endpoint
            .get_program_bytecode_from_program_id(&program_id.pubkey())
            .await
            .unwrap()
            .unwrap()
    );
}
