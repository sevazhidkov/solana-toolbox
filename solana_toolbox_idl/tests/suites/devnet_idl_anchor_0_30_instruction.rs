use std::collections::HashMap;

use serde_json::json;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::SeedDerivable;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;
use solana_toolbox_idl::ToolboxIdlProgramRoot;
use solana_toolbox_idl::ToolboxIdlTransactionInstruction;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Fetch the idl of an anchor program on chain
    let program_id = pubkey!("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j");
    let idl = ToolboxIdlProgramRoot::get_for_program_id(&mut endpoint, &program_id)
        .await
        .unwrap()
        .unwrap();
    // Find an account from another instruction so that we can re-use it
    let campaign_index = 3u64;
    let campaign = idl
        .find_instruction_account_address(
            &ToolboxIdlTransactionInstruction {
                program_id,
                name: "campaign_create".to_string(),
                accounts_addresses: HashMap::from_iter([]),
                args: json!({ "params": { "index": campaign_index } }),
            },
            &HashMap::from_iter([]),
            "campaign",
        )
        .unwrap();
    // Make sure the proper account has been properly resolved
    assert_eq!(
        campaign,
        Pubkey::find_program_address(
            &[b"Campaign", &campaign_index.to_le_bytes()],
            &program_id,
        )
        .0
    );
    // Addresses we'll be using for our instructions
    let payer =
        Keypair::from_seed(b"Hello world, this is a dummy payer for devnet")
            .unwrap();
    let collateral_mint =
        pubkey!("EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3");
    let user = Keypair::new();
    let user_collateral = endpoint
        .process_spl_associated_token_account_get_or_init(
            &payer,
            &user.pubkey(),
            &collateral_mint,
        )
        .await
        .unwrap();
    // Generate the actual instructions while resolving missing accounts
    let instruction_pledge_create = idl
        .resolve_instruction(
            &mut endpoint,
            &ToolboxIdlTransactionInstruction {
                program_id,
                name: "pledge_create".to_string(),
                accounts_addresses: HashMap::from_iter([
                    ("payer".to_string(), payer.pubkey()),
                    ("user".to_string(), user.pubkey()),
                    ("campaign".to_string(), campaign),
                ]),
                args: json!({ "params": {} }),
            },
        )
        .await
        .unwrap();
    let instruction_pledge_deposit = idl
        .resolve_instruction(
            &mut endpoint,
            &ToolboxIdlTransactionInstruction {
                program_id,
                name: "pledge_deposit".to_string(),
                accounts_addresses: HashMap::from_iter([
                    ("payer".to_string(), payer.pubkey()),
                    ("user".to_string(), user.pubkey()),
                    ("user_collateral".to_string(), user_collateral),
                    ("campaign".to_string(), campaign),
                ]),
                args: json!({ "params": { "collateral_amount": 0 } }),
            },
        )
        .await
        .unwrap();
    // Process the instructions to check if it works
    endpoint
        .process_instructions_with_signers(
            &payer,
            &[instruction_pledge_create, instruction_pledge_deposit],
            &[&user],
        )
        .await
        .unwrap();
}
