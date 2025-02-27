use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Create the endpoint pointing to mainnet
    let mut endpoint = ToolboxEndpoint::new_mainnet().await;
    // Check the USDC mint
    let usdc_mint =
        Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
            .unwrap();
    let usdc_metaplex_metadata =
        endpoint.get_spl_token_metaplex_metadata(&usdc_mint).await.unwrap();
    assert_eq!(
        usdc_metaplex_metadata.unwrap(),
        (
            Pubkey::from_str("2wmVCSfPxGPjrnMMn7rchp4uaeoTqN39mXFC2zhPdri9")
                .unwrap(),
            "USDC".to_string(),
            "USD Coin".to_string(),
            "".to_string()
        )
    );
    // Check the SBR mint
    let sbr_mint =
        Pubkey::from_str("Saber2gLauYim4Mvftnrasomsv6NvAuncvMEZwcLpD1")
            .unwrap();
    let sbr_metaplex_metadata =
        endpoint.get_spl_token_metaplex_metadata(&sbr_mint).await.unwrap();
    assert_eq!(
        sbr_metaplex_metadata.unwrap(),
        (
            Pubkey::from_str("GyktbGXbH9kvxP8RGfWsnFtuRgC7QCQo2WBqpo3ryk7L")
                .unwrap(),
            "SBR".to_string(),
            "Saber Protocol Token".to_string(),
            "".to_string()
        )
    );
}
