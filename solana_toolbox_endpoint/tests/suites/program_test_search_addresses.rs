use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Generate a bunch of accounts we'll use to generate an history
    let payer = Keypair::new();
    endpoint.request_airdrop(&payer.pubkey(), 1_000_000_000).await.unwrap();
    let mut users = vec![];
    for _ in 0..10 {
        users.push(Keypair::new());
    }
    // Generate a mint
    let collateral_mint_authority = Keypair::new();
    let collateral_mint = endpoint
        .process_spl_token_mint_new(
            &payer,
            &collateral_mint_authority.pubkey(),
            None,
            6,
        )
        .await
        .unwrap();
    // Mint some token to our users
    let mut users_collaterals = vec![];
    for i in 0..10 {
        let user_collateral = endpoint
            .process_spl_associated_token_account_get_or_init(
                &payer,
                &users[i].pubkey(),
                &collateral_mint,
            )
            .await
            .unwrap();
        endpoint
            .process_spl_token_mint_to(
                &payer,
                &collateral_mint,
                &collateral_mint_authority,
                &user_collateral,
                1_000_000 + u64::try_from(i).unwrap(),
            )
            .await
            .unwrap();
        users_collaterals.push(user_collateral);
    }
    // Check that the token program contains all expected accounts
    let token_program_addresses =
        endpoint.search_addresses(&spl_token::ID, None, &[]).await.unwrap();
    assert_eq!(token_program_addresses.len(), 11);
    assert!(token_program_addresses.contains(&collateral_mint));
    for user_collateral in &users_collaterals {
        assert!(token_program_addresses.contains(user_collateral));
    }
    // Find token accounts by data size
    let token_accounts_addresses = endpoint
        .search_addresses(&spl_token::ID, Some(165), &[])
        .await
        .unwrap();
    assert_eq!(token_accounts_addresses.len(), 10);
    for user_collateral in &users_collaterals {
        assert!(token_accounts_addresses.contains(user_collateral));
    }
    // Find token accounts by mint
    let token_accounts_addresses = endpoint
        .search_addresses(
            &spl_token::ID,
            None,
            &[(0, &collateral_mint.as_ref())],
        )
        .await
        .unwrap();
    assert_eq!(token_accounts_addresses.len(), 10);
    for i in 0..10 {
        assert!(token_accounts_addresses.contains(&users_collaterals[i]))
    }
    // Find mint by size and content
    let token_mints_addresses = endpoint
        .search_addresses(
            &spl_token::ID,
            Some(82),
            &[(4, collateral_mint_authority.pubkey().as_ref())],
        )
        .await
        .unwrap();
    assert_eq!(token_mints_addresses.len(), 1);
    assert!(token_mints_addresses.contains(&collateral_mint));
    // Find token account by owner
    for i in 0..10 {
        let user = &users[i];
        let user_addresses = endpoint
            .search_addresses(
                &spl_token::ID,
                None,
                &[(0, &collateral_mint.as_ref()), (32, user.pubkey().as_ref())],
            )
            .await
            .unwrap();
        assert_eq!(user_addresses.len(), 1);
        assert!(user_addresses.contains(&users_collaterals[i]))
    }
    // Find token account by balance
    for i in 0..10 {
        let amount = 1_000_000 + u64::try_from(i).unwrap();
        let user_addresses = endpoint
            .search_addresses(
                &spl_token::ID,
                Some(165),
                &[(64, &amount.to_le_bytes())],
            )
            .await
            .unwrap();
        assert_eq!(user_addresses.len(), 1);
        assert!(user_addresses.contains(&users_collaterals[i]))
    }
}
