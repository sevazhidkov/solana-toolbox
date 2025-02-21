use std::str::FromStr;

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct TokenMetadata {
    //    pub discriminator: u8,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[tokio::test]
pub async fn run() {
    // Create the endpoint pointing to mainnet
    let mut endpoint = ToolboxEndpoint::new_mainnet().await;
    // Check the USDC mint
    let usdc_mint =
        Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
            .unwrap();
    let usdc_metadata_metaplex =
        endpoint.get_spl_token_metadata_metaplex(&usdc_mint).await.unwrap();
    assert_eq!(
        usdc_metadata_metaplex.unwrap(),
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
    let sbr_metadata_metaplex =
        endpoint.get_spl_token_metadata_metaplex(&sbr_mint).await.unwrap();
    assert_eq!(
        sbr_metadata_metaplex.unwrap(),
        (
            Pubkey::from_str("GyktbGXbH9kvxP8RGfWsnFtuRgC7QCQo2WBqpo3ryk7L")
                .unwrap(),
            "SBR".to_string(),
            "Saber Protocol Token".to_string(),
            "".to_string()
        )
    );
}
