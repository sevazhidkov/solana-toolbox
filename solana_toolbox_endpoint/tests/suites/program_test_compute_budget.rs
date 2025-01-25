use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::transfer;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint =
        ToolboxEndpoint::new_program_test().await;
    // Prepare a payer
    let payer = Keypair::new();
    endpoint.process_airdrop(&payer.pubkey(), 2_000_000_000).await.unwrap();
    // Unique wallet
    let destination = Keypair::new();
    // Send a custom compute budget instruction
    let instruction =
        transfer(&payer.pubkey(), &destination.pubkey(), 1_000_000_000);
    endpoint
        .process_instructions_with_signers_and_compute(
            &[instruction],
            &payer,
            &[&payer],
            Some(1_000_000),
            Some(42_000_000), // in micro-lamports equals 42 lamports/unit
        )
        .await
        .unwrap();
    // Check the payer's lamport balance
    assert_eq!(
        2_000_000_000 // Original payer airdrop
            - 1_000_000_000 // Transfered lamports
            - 5_000 // Transaction fees
            - 42_000_000, // 1_000_000 units * 42 lamports price/per unit
        endpoint.get_account_lamports(&payer.pubkey()).await.unwrap().unwrap()
    );
}
