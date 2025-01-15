use std::collections::HashMap;

use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::SeedDerivable;
use solana_sdk::signer::Signer;
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
    let program_idl =
        endpoint.get_program_id_anchor_idl(&program_id).await.unwrap().unwrap();
    // Find an account we can read from the endpoint
    let campaign_index = 0u64;
    let campaign_pda = Pubkey::find_program_address(
        &[b"Campaign", &campaign_index.to_le_bytes()],
        &program_id,
    );
    let campaign_address = campaign_pda.0;
    let campaign_bump = campaign_pda.1;
    // Read an account using the IDL directly
    let (campaign_data_length, campaign_data_json) = endpoint
        .get_account_data_anchor_idl_account_deserialized(
            &program_idl,
            &campaign_address,
            "Campaign",
        )
        .await
        .unwrap()
        .unwrap();
    // Check that the account was parsed properly and values matches
    assert_eq!(675, campaign_data_length);
    assert_eq!(
        u64::from(campaign_bump),
        campaign_data_json.get("bump").unwrap().as_u64().unwrap()
    );
    assert_eq!(
        campaign_index,
        campaign_data_json.get("index").unwrap().as_u64().unwrap()
    );
    assert_eq!(
        "Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9",
        campaign_data_json.get("authority").unwrap().as_str().unwrap()
    );
    assert_eq!(
        "EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3",
        campaign_data_json.get("collateral_mint").unwrap().as_str().unwrap()
    );
    assert_eq!(
        "3dtmuqjKdL12ptVmDPjAXeYJE9nLgA74ti1Gm2ME9qH9",
        campaign_data_json.get("redeemable_mint").unwrap().as_str().unwrap()
    );
    // Try to generate a custom instruction
    let payer =
        Keypair::from_seed(b"Hello world, this is a dummy payer for devnet...")
            .unwrap();
    let user = Keypair::new();
    let mut account_addresses = HashMap::new();
    account_addresses.insert("payer".to_string(), payer.pubkey());
    account_addresses.insert("user".to_string(), user.pubkey());
    account_addresses.insert("campaign".to_string(), campaign_address);
    let instruction = endpoint
        .generate_anchor_idl_instruction(
            &program_idl,
            "pledge_create",
            &account_addresses,
        )
        .unwrap();
    endpoint
        .process_instruction_with_signers(instruction, &payer, &[&user])
        .await
        .unwrap();
}
