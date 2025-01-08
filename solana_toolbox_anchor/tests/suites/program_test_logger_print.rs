use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_anchor::ToolboxAnchorEndpoint;
use solana_toolbox_anchor::ToolboxEndpoint;
use solana_toolbox_anchor::ToolboxEndpointLoggerPrint;

#[tokio::test]
pub async fn program_test_logger_print() {
    // Initialize the endpoint
    let mut endpoint = ToolboxAnchorEndpoint::from(
        ToolboxEndpoint::new_program_test_with_builtin_programs(&[]).await,
    );
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrint::new()));
    // Prepare a payer
    let payer = Keypair::new();
    endpoint.process_airdrop(&payer.pubkey(), 2_000_000_000).await.unwrap();
    // Unique wallet
    let destination = Keypair::new();
    // Send a simple transfer instruction
    endpoint
        .process_system_transfer(
            &payer,
            &payer,
            &destination.pubkey(),
            1_000_000_000,
        )
        .await
        .unwrap();
    // Read account and check result
    assert_eq!(
        2_000_000_000 // Original payer airdrop
            - 1_000_000_000 // Transfered lamports
            - 5_000, // Transaction fees
        endpoint.get_account_lamports(&payer.pubkey()).await.unwrap()
    );
}
