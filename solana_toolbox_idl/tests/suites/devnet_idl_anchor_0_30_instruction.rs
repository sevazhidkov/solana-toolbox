use std::collections::HashMap;

use serde_json::json;
use serde_json::Map;
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
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_rpc_with_url_and_commitment(
        "https://api.devnet.solana.com",
        CommitmentConfig::confirmed(),
    );
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrint::default()));
    // Fetch the idl of an anchor program on chain
    let program_id = pubkey!("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j");
    let idl = ToolboxIdl::get_for_program_id(&mut endpoint, &program_id)
        .await
        .unwrap()
        .unwrap();
    // Find an account we can read from the endpoint
    let campaign_index = 0u64;
    let campaign = idl
        .resolve_instruction_account_address(
            "campaign",
            &program_id,
            "campaign_create",
            &HashMap::new(),
            &Map::new(),
            json!({ "params": { "index": campaign_index }})
                .as_object()
                .unwrap(),
        )
        .unwrap();
    // Make sure the proper account has been resolved
    assert_eq!(
        campaign,
        Pubkey::find_program_address(
            &[b"Campaign", &campaign_index.to_le_bytes()],
            &program_id,
        )
        .0
    );
    // Try to generate a custom instruction
    let payer =
        Keypair::from_seed(b"Hello world, this is a dummy payer for devnet")
            .unwrap();
    let user = Keypair::new();
    // Prepare the accounts necessary for the instruction
    let mut instruction_accounts_addresses = HashMap::new();
    instruction_accounts_addresses.insert("payer".to_string(), payer.pubkey());
    instruction_accounts_addresses.insert("user".to_string(), user.pubkey());
    instruction_accounts_addresses.insert("campaign".to_string(), campaign);
    // Prepare the arguments necessary for the instruction
    let instruction_args_value = json!({
        "params": {},
    });
    // Resolve missing instruction accounts
    let instruction_accounts_addresses = idl
        .fill_instruction_accounts_addresses(
            &program_id,
            "pledge_create",
            &instruction_accounts_addresses,
            &Map::new(),
            instruction_args_value.as_object().unwrap(),
        )
        .unwrap();
    // Generate the actual instruction
    let instruction = idl
        .generate_instruction(
            &program_id,
            "pledge_create",
            &instruction_accounts_addresses,
            instruction_args_value.as_object().unwrap(),
        )
        .unwrap();
    // Process the instruction to check if it works
    endpoint
        .process_instruction_with_signers(instruction, &payer, &[&user])
        .await
        .unwrap();
}
