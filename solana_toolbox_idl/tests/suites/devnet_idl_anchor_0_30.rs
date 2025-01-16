use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::SeedDerivable;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrint;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn devnet_idl_anchor_0_30() {
    // Create the devnet endpoint
    let mut endpoint = ToolboxEndpoint::new_rpc_with_url_and_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrint::new()));
    // Fetch the idl of an anchor program on chain
    let program_id = pubkey!("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j");
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
    let (campaign_data_size, campaign_data_value) = idl
        .get_account(&mut endpoint, "Campaign", &campaign_address)
        .await
        .unwrap()
        .unwrap();
    // Check that the account was parsed properly and values matches
    assert_eq!(675, campaign_data_size);
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
    // Try to generate a custom instruction
    let payer =
        Keypair::from_seed(b"Hello world, this is a dummy payer for devnet")
            .unwrap();
    let user = Keypair::new();
    // Prepare the accounts necessary for the instruction
    let mut instruction_accounts = HashMap::new();
    instruction_accounts.insert("payer".to_string(), payer.pubkey());
    instruction_accounts.insert("user".to_string(), user.pubkey());
    instruction_accounts.insert("campaign".to_string(), campaign_address);
    // Prepare the arguments necessary for the instruction
    let mut instruction_args = Map::new();
    instruction_args.insert("params".into(), Value::Object(Map::new()));
    // Generate the actual instruction
    let instruction = idl
        .generate_instruction(
            &program_id,
            "pledge_create",
            &instruction_accounts,
            &instruction_args,
        )
        .unwrap();
    // Process the instruction to check if it works
    endpoint
        .process_instruction_with_signers(instruction, &payer, &[&user])
        .await
        .unwrap();
}
