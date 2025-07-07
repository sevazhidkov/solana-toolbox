use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl_core::find_spl_associated_token_account;
use solana_toolbox_idl_core::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_30.json").unwrap(),
    )
    .unwrap();
    // Important account addresses
    let program_id = Pubkey::new_unique();
    let payer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let user = Pubkey::new_unique();
    let collateral_mint = Pubkey::new_unique();
    let redeemable_mint = Pubkey::new_unique();
    // Expected values for generatable accounts
    let authority_collateral =
        find_spl_associated_token_account(&authority, &collateral_mint);
    let user_collateral =
        find_spl_associated_token_account(&user, &collateral_mint);
    let campaign_index = 0u64;
    let campaign = Pubkey::find_program_address(
        &[b"Campaign", &campaign_index.to_le_bytes()],
        &program_id,
    )
    .0;
    let campaign_collateral =
        find_spl_associated_token_account(&campaign, &collateral_mint);
    let pledge = Pubkey::find_program_address(
        &[b"Pledge", campaign.as_ref(), user.as_ref()],
        &program_id,
    )
    .0;
    // Generate all missing IX accounts with just the minimum information
    let campaign_create_addresses = idl_program
        .instructions
        .get("campaign_create")
        .unwrap()
        .find_addresses(
            &program_id,
            &json!({ "params": { "index": campaign_index } }),
            &HashMap::from([
                ("payer".to_string(), payer),
                ("authority".to_string(), authority),
                ("collateral_mint".to_string(), collateral_mint),
                ("redeemable_mint".to_string(), redeemable_mint),
            ]),
        );
    // Check outcome
    assert_eq!(
        campaign,
        *campaign_create_addresses.get("campaign").unwrap()
    );
    assert_eq!(
        campaign_collateral,
        *campaign_create_addresses
            .get("campaign_collateral")
            .unwrap()
    );
    // Generate all missing IX accounts with just the minimum information
    let campaign_extract_addresses = idl_program
        .instructions
        .get("campaign_extract")
        .unwrap()
        .find_addresses_with_accounts_states(
            &program_id,
            &json!({ "params": { "index": campaign_index } }),
            &HashMap::from([
                ("payer".to_string(), payer),
                ("authority".to_string(), authority),
                ("authority_collateral".to_string(), authority_collateral),
                ("campaign".to_string(), campaign),
            ]),
            &HashMap::from_iter([(
                "campaign".to_string(),
                json!({
                    "collateral_mint": collateral_mint.to_string()
                }),
            )]),
        );
    // Check outcome
    assert_eq!(
        campaign_collateral,
        *campaign_extract_addresses
            .get("campaign_collateral")
            .unwrap()
    );
    // Generate all missing IX accounts with just the minimum information
    let pledge_create_addresses = idl_program
        .instructions
        .get("pledge_create")
        .unwrap()
        .find_addresses(
            &program_id,
            &json!({}),
            &HashMap::from([
                ("payer".to_string(), payer),
                ("user".to_string(), user),
                ("campaign".to_string(), campaign),
            ]),
        );
    // Check outcome
    assert_eq!(pledge, *pledge_create_addresses.get("pledge").unwrap());
    // Generate all missing IX accounts with just the minimum information
    let pledge_deposit_addresses = idl_program
        .instructions
        .get("pledge_deposit")
        .unwrap()
        .find_addresses_with_accounts_states(
            &program_id,
            &json!({}),
            &HashMap::from([
                ("payer".to_string(), payer),
                ("user".to_string(), user),
                ("user_collateral".to_string(), user_collateral),
                ("campaign".to_string(), campaign),
            ]),
            &HashMap::from_iter([(
                "campaign".to_string(),
                json!({
                    "collateral_mint": collateral_mint.to_string()
                }),
            )]),
        );
    // Check outcome
    assert_eq!(
        campaign_collateral,
        *pledge_deposit_addresses.get("campaign_collateral").unwrap()
    );
    assert_eq!(pledge, *pledge_deposit_addresses.get("pledge").unwrap());
}
