use std::collections::HashMap;

use serde_json::json;
use solana_sdk::commitment_config::CommitmentConfig;
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
    let instruction_args = json!({
        "params": {},
    });
    // Generate the actual instruction
    let instruction = idl
        .generate_instruction(
            &program_id,
            "pledge_create",
            &instruction_accounts,
            instruction_args.as_object().unwrap(),
        )
        .unwrap();
    // Process the instruction to check if it works
    endpoint
        .process_instruction_with_signers(instruction, &payer, &[&user])
        .await
        .unwrap();
}
