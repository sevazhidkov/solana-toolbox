use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Prepare a payer
    let payer = Keypair::new();
    endpoint.request_airdrop(&payer.pubkey(), 1_000_000_000).await.unwrap();
    // Create a mint
    let mint_authority = Keypair::new();
    let mint_decimals = 9;
    let mint = endpoint
        .process_spl_token_mint_new(
            &payer,
            &mint_authority.pubkey(),
            None,
            mint_decimals,
        )
        .await
        .unwrap();
    // Create user1 and ATA
    let user1 = Keypair::new();
    let user1_account = endpoint
        .process_spl_associated_token_account_get_or_init(
            &payer,
            &user1.pubkey(),
            &mint,
        )
        .await
        .unwrap();
    // Create user2 and regular TA
    let user2: Keypair = Keypair::new();
    let user2_account = endpoint
        .process_spl_token_account_new(&payer, &user2.pubkey(), &mint)
        .await
        .unwrap();
    // Mint to user1
    endpoint
        .process_spl_token_mint_to(
            &payer,
            &mint,
            &mint_authority,
            &user1_account,
            ToolboxEndpoint::convert_ui_amount_to_spl_token_amount(
                42.0,
                mint_decimals,
            ),
        )
        .await
        .unwrap();
    // Transfer from user1 to user2
    endpoint
        .process_spl_token_transfer(
            &payer,
            &user1,
            &user1_account,
            &user2_account,
            ToolboxEndpoint::convert_ui_amount_to_spl_token_amount(
                30.0,
                mint_decimals,
            ),
        )
        .await
        .unwrap();
    // Transfer back from user1 to user2
    endpoint
        .process_spl_token_transfer(
            &payer,
            &user2,
            &user2_account,
            &user1_account,
            ToolboxEndpoint::convert_ui_amount_to_spl_token_amount(
                8.0,
                mint_decimals,
            ),
        )
        .await
        .unwrap();
    // Check balances
    assert_eq!(
        20_000_000_000,
        endpoint
            .get_spl_token_account(&user1_account)
            .await
            .unwrap()
            .unwrap()
            .amount,
    );
    assert_eq!(
        22_000_000_000,
        endpoint
            .get_spl_token_account(&user2_account)
            .await
            .unwrap()
            .unwrap()
            .amount,
    );
    assert_eq!(
        42_000_000_000,
        endpoint.get_spl_token_mint(&mint).await.unwrap().unwrap().supply,
    );
}
