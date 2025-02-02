use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlInstruction;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_string =
        read_to_string("./tests/fixtures/idl_anchor_0_30.json").unwrap();
    let idl = ToolboxIdl::try_from_str(&idl_string).unwrap();
    // Important account addresses
    let program_id = Pubkey::new_unique();
    let payer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let user = Pubkey::new_unique();
    let collateral_mint = Pubkey::new_unique();
    let redeemable_mint = Pubkey::new_unique();
    // Expected values for generatable accounts
    let authority_collateral =
        ToolboxEndpoint::find_spl_associated_token_account(
            &authority,
            &collateral_mint,
        );
    let user_collateral = ToolboxEndpoint::find_spl_associated_token_account(
        &user,
        &collateral_mint,
    );
    let campaign_index = 0u64;
    let campaign = Pubkey::find_program_address(
        &[b"Campaign", &campaign_index.to_le_bytes()],
        &program_id,
    )
    .0;
    let campaign_collateral =
        ToolboxEndpoint::find_spl_associated_token_account(
            &campaign,
            &collateral_mint,
        );
    let pledge = Pubkey::find_program_address(
        &[b"Pledge", campaign.as_ref(), user.as_ref()],
        &program_id,
    )
    .0;
    // Generate all missing IX accounts with just the minimum information
    let campaign_create_accounts_addresses = idl
        .find_instruction_accounts_addresses(
            &ToolboxIdlInstruction {
                program_id,
                name: "campaign_create".to_string(),
                accounts_addresses: HashMap::from([
                    ("payer".to_string(), payer),
                    ("authority".to_string(), authority),
                    ("collateral_mint".to_string(), collateral_mint),
                    ("redeemable_mint".to_string(), redeemable_mint),
                ]),
                args: json!({ "params": { "index": campaign_index } }),
            },
            &HashMap::from_iter([]),
        )
        .unwrap();
    // Check outcome
    assert_eq!(
        campaign,
        *campaign_create_accounts_addresses.get("campaign").unwrap()
    );
    assert_eq!(
        campaign_collateral,
        *campaign_create_accounts_addresses.get("campaign_collateral").unwrap()
    );
    // Generate all missing IX accounts with just the minimum information
    let campaign_extract_accounts_addresses = idl
        .find_instruction_accounts_addresses(
            &ToolboxIdlInstruction {
                program_id,
                name: "campaign_extract".to_string(),
                accounts_addresses: HashMap::from([
                    ("payer".to_string(), payer),
                    ("authority".to_string(), authority),
                    ("authority_collateral".to_string(), authority_collateral),
                    ("campaign".to_string(), campaign),
                ]),
                args: json!({ "params": {} }),
            },
            &HashMap::from_iter([(
                "campaign".to_string(),
                ToolboxIdlAccount {
                    name: "Campaign".to_string(),
                    state: json!({
                        "collateral_mint": collateral_mint.to_string()
                    }),
                },
            )]),
        )
        .unwrap();
    // Check outcome
    assert_eq!(
        campaign_collateral,
        *campaign_extract_accounts_addresses
            .get("campaign_collateral")
            .unwrap()
    );
    // Generate all missing IX accounts with just the minimum information
    let pledge_create_accounts_addresses = idl
        .find_instruction_accounts_addresses(
            &ToolboxIdlInstruction {
                program_id,
                name: "pledge_create".to_string(),
                accounts_addresses: HashMap::from([
                    ("payer".to_string(), payer),
                    ("user".to_string(), user),
                    ("campaign".to_string(), campaign),
                ]),
                args: json!({ "params": {} }),
            },
            &HashMap::from_iter([]),
        )
        .unwrap();
    // Check outcome
    assert_eq!(
        pledge,
        *pledge_create_accounts_addresses.get("pledge").unwrap()
    );
    // Generate all missing IX accounts with just the minimum information
    let pledge_deposit_accounts_addresses = idl
        .find_instruction_accounts_addresses(
            &ToolboxIdlInstruction {
                program_id,
                name: "pledge_deposit".to_string(),
                accounts_addresses: HashMap::from([
                    ("payer".to_string(), payer),
                    ("user".to_string(), user),
                    ("user_collateral".to_string(), user_collateral),
                    ("campaign".to_string(), campaign),
                ]),
                args: json!({ "params": {} }),
            },
            &HashMap::from_iter([(
                "campaign".to_string(),
                ToolboxIdlAccount {
                    name: "Campaign".to_string(),
                    state: json!({
                        "collateral_mint": collateral_mint.to_string()
                    }),
                },
            )]),
        )
        .unwrap();
    // Check outcome
    assert_eq!(
        campaign_collateral,
        *pledge_deposit_accounts_addresses.get("campaign_collateral").unwrap()
    );
    assert_eq!(
        pledge,
        *pledge_deposit_accounts_addresses.get("pledge").unwrap()
    );
}
