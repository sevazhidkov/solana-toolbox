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
        Keypair::from_seed(b"This is a dummy devnet payer used for simulation")
            .unwrap();
    // Make a mint
    let mint_authority = Keypair::new();
    let mint = endpoint
        .process_spl_token_mint_new(&payer, &mint_authority.pubkey(), None, 6)
        .await
        .unwrap();
    // Create matatadata for mint
    let metadata_authority = Keypair::new();
    endpoint
        .process_spl_token_metadata_metaplex_create(
            &payer,
            &mint,
            &mint_authority,
            &metadata_authority.pubkey(),
            "SYMBOL".to_string(),
            "NAME".to_string(),
            "URI".to_string(),
        )
        .await
        .unwrap();
    // Check that the metadata has been uploaded
    assert_eq!(
        endpoint.get_spl_token_metadata_metaplex(&mint).await.unwrap().unwrap(),
        (
            metadata_authority.pubkey(),
            "SYMBOL".to_string(),
            "NAME".to_string(),
            "URI".to_string()
        )
    );
    // Dummy URI with an actual image in there
    let dummy_uri = "https://raw.githubusercontent.com/neonlabsorg/mint-fungible-spl/refs/heads/main/metadata/updateToken.json";
    // Update
    endpoint
        .process_spl_token_metadata_metaplex_update(
            &payer,
            &mint,
            &metadata_authority,
            "VVVV".to_string(),
            "VVVV".to_string(),
            dummy_uri.to_string(),
        )
        .await
        .unwrap();
    // Check that the metadata has been updated
    assert_eq!(
        endpoint.get_spl_token_metadata_metaplex(&mint).await.unwrap().unwrap(),
        (
            metadata_authority.pubkey(),
            "VVVV".to_string(),
            "VVVV".to_string(),
            dummy_uri.to_string()
        )
    );
    eprintln!("mint: {:?}", mint);
    panic!("LOL");
}
