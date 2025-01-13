use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_anchor::ToolboxAnchorEndpoint;
use solana_toolbox_anchor::ToolboxEndpoint;
use solana_toolbox_anchor::ToolboxEndpointLoggerPrint;

#[tokio::test]
pub async fn devnet_idl_anchor_0_30() {
    // Create the devnet endpoint
    let mut endpoint = ToolboxAnchorEndpoint::from(
        ToolboxEndpoint::new_rpc_with_url_and_commitment(
            "https://api.devnet.solana.com".to_string(),
            CommitmentConfig::confirmed(),
        ),
    );
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrint::new()));
    // Fetch the idl of an anchor program on chain
    let program_id = pubkey!("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j");
    let idl =
        endpoint.get_program_id_anchor_idl(&program_id).await.unwrap().unwrap();
    // Find an account we can read from the endpoint
    let campaign_index = 1u64;
    let campaign_pda = Pubkey::find_program_address(
        &[b"Campaign", &campaign_index.to_le_bytes()],
        &program_id,
    );
    let campaign_address = campaign_pda.0;
    let campaign_bump = campaign_pda.1;
    // Read an account using the IDL directly
    let campaign_data = endpoint
        .get_account_data_anchor_idl_deserialized_json(
            idl,
            "Campaign",
            &campaign_address,
        )
        .await
        .unwrap()
        .unwrap();
    eprintln!("campaign_data:{:#?}", campaign_data);
    // Check that the account was parsed properly and values matches
    assert_eq!(
        u64::from(campaign_bump),
        campaign_data.get("bump").unwrap().as_u64().unwrap()
    );
    assert_eq!(
        campaign_index,
        campaign_data.get("index").unwrap().as_u64().unwrap()
    );
    assert_eq!(
        "Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9",
        campaign_data.get("authority").unwrap().as_str().unwrap()
    );
    assert_eq!(
        "EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3",
        campaign_data.get("collateral_mint").unwrap().as_str().unwrap()
    );
    assert_eq!(
        "2jRbxfErQ8meaTeJfGXVQXwsqT6B5R12kigmYiVFuMq9",
        campaign_data.get("redeemable_mint").unwrap().as_str().unwrap()
    );
}
