use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;
use solana_toolbox_idl::ToolboxIdlResolver;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Find an account we can read from the endpoint
    let campaign_index = 0u64;
    let campaign_pda = Pubkey::find_program_address(
        &[b"Campaign", &campaign_index.to_le_bytes()],
        &pubkey!("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j"),
    );
    let campaign = campaign_pda.0;
    let campaign_bump = campaign_pda.1;
    // Read an account using the IDL directly auto-downloaded from the chain
    let campaign_details = ToolboxIdlResolver::new()
        .resolve_account_details(&mut endpoint, &campaign)
        .await
        .unwrap()
        .unwrap();
    // Check that the account was parsed properly and values matches
    assert_eq!("Campaign", campaign_details.0.name);
    assert_eq!(
        u64::from(campaign_bump),
        campaign_details.1.get("bump").unwrap().as_u64().unwrap()
    );
    assert_eq!(
        campaign_index,
        campaign_details.1.get("index").unwrap().as_u64().unwrap()
    );
    assert_eq!(
        "Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9",
        campaign_details
            .1
            .get("authority")
            .unwrap()
            .as_str()
            .unwrap()
    );
    assert_eq!(
        "EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3",
        campaign_details
            .1
            .get("collateral_mint")
            .unwrap()
            .as_str()
            .unwrap()
    );
    assert_eq!(
        "3dtmuqjKdL12ptVmDPjAXeYJE9nLgA74ti1Gm2ME9qH9",
        campaign_details
            .1
            .get("redeemable_mint")
            .unwrap()
            .as_str()
            .unwrap()
    );
}
