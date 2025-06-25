use serde_json::json;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlService;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Find an account we can read from the endpoint
    let campaign_index = 0u64;
    let campaign_pda = Pubkey::find_program_address(
        &[b"Campaign", &campaign_index.to_le_bytes()],
        &pubkey!("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j"),
    );
    let campaign = campaign_pda.0;
    let campaign_bump = campaign_pda.1;
    // Read an account using the IDL directly auto-downloaded from the chain
    let campaign_info = ToolboxIdlService::new()
        .get_and_infer_and_decode_account(&mut endpoint, &campaign)
        .await
        .unwrap();
    // Check that the account was parsed properly and values matches
    assert_eq!(
        campaign_info.program.metadata.name,
        Some("psyche_crowd_funding".to_string()),
    );
    assert_eq!(campaign_info.account.name, "Campaign");
    assert_eq!(&campaign_info.state["bump"], &json!(campaign_bump),);
    assert_eq!(&campaign_info.state["index"], &json!(campaign_index),);
    assert_eq!(
        &campaign_info.state["authority"],
        &json!("Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9"),
    );
    assert_eq!(
        &campaign_info.state["collateral_mint"],
        &json!("EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3"),
    );
    assert_eq!(
        &campaign_info.state["redeemable_mint"],
        &json!("3dtmuqjKdL12ptVmDPjAXeYJE9nLgA74ti1Gm2ME9qH9"),
    );
}
