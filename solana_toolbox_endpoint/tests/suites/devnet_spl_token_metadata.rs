use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::SeedDerivable;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Make a payer
    let payer =
        Keypair::from_seed(b"This is a dummy devnet payer for paying rent")
            .unwrap();
    // Make a mint
    let mint_authority = Keypair::new();
    let mint = endpoint
        .process_spl_token_mint_new(&payer, &mint_authority.pubkey(), None, 6)
        .await
        .unwrap();
    // Mint some token to a wallet
    let recipient =
        Pubkey::from_str("CqDmEX6EvNYbZeJYZonKRNNZ9hFiPs8Gs9eek7FEmsty")
            .unwrap();
    let recipient_token = endpoint
        .process_spl_associated_token_account_get_or_init(
            &payer, &recipient, &mint,
        )
        .await
        .unwrap();
    endpoint
        .process_spl_token_mint_to(
            &payer,
            &mint,
            &mint_authority,
            &recipient_token,
            1_000_000,
        )
        .await
        .unwrap();
    // Create matatadata for mint
    let metadata_authority = Keypair::new();
    endpoint
        .process_spl_token_metaplex_metadata_create(
            &payer,
            &mint,
            &mint_authority,
            (
                metadata_authority.pubkey(),
                "SYMBOL".to_string(),
                "NAME".to_string(),
                "URI".to_string(),
            ),
        )
        .await
        .unwrap();
    // Check that the metadata has been uploaded
    assert_eq!(
        endpoint.get_spl_token_metaplex_metadata(&mint).await.unwrap().unwrap(),
        (
            metadata_authority.pubkey(),
            "SYMBOL".to_string(),
            "NAME".to_string(),
            "URI".to_string()
        )
    );
    // Dummy URI with an actual image in there
    let dummy_uri = "https://raw.githubusercontent.com/crypto-vincent/solana-toolbox/refs/heads/master/solana_toolbox_endpoint/tests/fixtures/spl_token_metaplex_metadata.json";
    // Update
    endpoint
        .process_spl_token_metaplex_metadata_update(
            &payer,
            &mint,
            &metadata_authority,
            (
                metadata_authority.pubkey(),
                "SYMBOL2".to_string(),
                "NAME-UPDATED".to_string(),
                dummy_uri.to_string(),
            ),
        )
        .await
        .unwrap();
    // Check that the metadata has been updated
    assert_eq!(
        endpoint.get_spl_token_metaplex_metadata(&mint).await.unwrap().unwrap(),
        (
            metadata_authority.pubkey(),
            "SYMBOL2".to_string(),
            "NAME-UPDATED".to_string(),
            dummy_uri.to_string()
        )
    );
}
