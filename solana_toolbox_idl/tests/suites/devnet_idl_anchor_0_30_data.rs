use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrint;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_rpc_with_url_and_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrint::new()));
    // Fetch the idl of an anchor program on chain
    let program_id =
        Pubkey::from_str_const("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j");
    let idl = ToolboxIdl::get_for_program_id(&mut endpoint, &program_id)
        .await
        .unwrap()
        .unwrap();
    // Find an account we can read from the endpoint
    let campaign_index = 0u64;
    let campaign_pda = Pubkey::find_program_address(
        &[b"Campaign", &campaign_index.to_le_bytes()],
        &program_id,
    );
    let campaign_address = campaign_pda.0;
    let campaign_bump = campaign_pda.1;
    // Read an account using the IDL directly
    let campaign_data_value = idl
        .get_account(&mut endpoint, "Campaign", &campaign_address)
        .await
        .unwrap()
        .unwrap();
    eprintln!("campaign_data_value: {:?}", campaign_data_value);
    // Check that the account was parsed properly and values matches
    assert_eq!(
        u64::from(campaign_bump),
        campaign_data_value.get("bump").unwrap().as_u64().unwrap()
    );
    assert_eq!(
        campaign_index,
        campaign_data_value.get("index").unwrap().as_u64().unwrap()
    );
    assert_eq!(
        "Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9",
        campaign_data_value.get("authority").unwrap().as_str().unwrap()
    );
    assert_eq!(
        "EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3",
        campaign_data_value.get("collateral_mint").unwrap().as_str().unwrap()
    );
    assert_eq!(
        "3dtmuqjKdL12ptVmDPjAXeYJE9nLgA74ti1Gm2ME9qH9",
        campaign_data_value.get("redeemable_mint").unwrap().as_str().unwrap()
    );
}
