use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_anchor::ToolboxAnchorEndpoint;
use solana_toolbox_anchor::ToolboxEndpoint;

#[tokio::test]
pub async fn program_test_airdrop_transfer_get() {
    // Initialize the endpoint from a non-anchor endpoint
    let mut endpoint = ToolboxAnchorEndpoint::from(
        ToolboxEndpoint::new_program_test_with_builtin_programs(&[]).await,
    );
    // Prepare a payer
    let payer = Keypair::new();
    endpoint.process_airdrop(&payer.pubkey(), 2_000_000_000).await.unwrap();
    // Check that a regular system transfer works
    let receiver = Keypair::new();
    endpoint
        .process_system_transfer(
            &payer,
            &payer,
            &receiver.pubkey(),
            1_000_000_000,
        )
        .await
        .unwrap();
    // Check that we can read accounts
    let receiver_lamports =
        endpoint.get_account_lamports(&receiver.pubkey()).await.unwrap();
    // Check result
    assert_eq!(1_000_000_000, receiver_lamports);
}
