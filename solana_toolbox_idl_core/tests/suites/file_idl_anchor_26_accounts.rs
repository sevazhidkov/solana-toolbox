use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl_core::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_26.json").unwrap(),
    )
    .unwrap();
    // Important account addresses
    let program_id = Pubkey::new_unique();
    let owner = Pubkey::new_unique();
    let borrower = Pubkey::new_unique();
    let liquidity_pool_token_account = Pubkey::new_unique();
    let treasury = Pubkey::new_unique();
    let treasury_pool_token_account = Pubkey::new_unique();
    let base_token_mint = Pubkey::new_unique();
    let placeholder = Pubkey::new_unique();
    // Expected accounts addresses
    let global_market_seed = "abcd";
    let global_market_state = Pubkey::find_program_address(
        &[global_market_seed.as_bytes()],
        &program_id,
    )
    .0;
    let market_admins = Pubkey::find_program_address(
        &[global_market_state.as_ref(), b"admins"],
        &program_id,
    )
    .0;
    let program_state =
        Pubkey::find_program_address(&[b"program-state"], &program_id).0;
    let lp_token_mint = Pubkey::find_program_address(
        &[global_market_state.as_ref(), b"lp-token-mint"],
        &program_id,
    )
    .0;
    let signing_authority = Pubkey::find_program_address(
        &[global_market_state.as_ref()],
        &program_id,
    )
    .0;
    let deal_number = 77u16;
    let deal = Pubkey::find_program_address(
        &[
            global_market_state.as_ref(),
            borrower.as_ref(),
            &deal_number.to_le_bytes(),
            b"deal-info",
        ],
        &program_id,
    )
    .0;
    let deal_tranches = Pubkey::find_program_address(
        &[global_market_state.as_ref(), deal.as_ref(), b"tranches"],
        &program_id,
    )
    .0;
    let repayment_schedule = Pubkey::find_program_address(
        &[
            global_market_state.as_ref(),
            deal.as_ref(),
            b"repayment-schedule",
        ],
        &program_id,
    )
    .0;
    // Generate all missing IX accounts with just the minimum information
    let initialize_market_addresses = idl_program
        .instructions
        .get("initialize_market")
        .unwrap()
        .find_addresses(
            &program_id,
            &json!({ "global_market_seed": global_market_seed.to_string() }),
            &HashMap::from([
                ("owner".to_string(), owner),
                (
                    "liquidity_pool_token_account".to_string(),
                    liquidity_pool_token_account,
                ),
                ("treasury".to_string(), treasury),
                (
                    "treasury_pool_token_account".to_string(),
                    treasury_pool_token_account,
                ),
                ("base_token_mint".to_string(), base_token_mint),
                ("associated_token_program".to_string(), placeholder),
                ("rent".to_string(), placeholder),
                ("token_program".to_string(), placeholder),
                ("system_program".to_string(), placeholder),
            ]),
        );
    // Check the outcomes
    assert_eq!(
        *initialize_market_addresses
            .get("global_market_state")
            .unwrap(),
        global_market_state,
    );
    assert_eq!(
        *initialize_market_addresses.get("market_admins").unwrap(),
        market_admins,
    );
    assert_eq!(
        *initialize_market_addresses.get("program_state").unwrap(),
        program_state,
    );
    assert_eq!(
        *initialize_market_addresses
            .get("signing_authority")
            .unwrap(),
        signing_authority,
    );
    assert_eq!(
        *initialize_market_addresses.get("lp_token_mint").unwrap(),
        lp_token_mint,
    );
    // Generate all missing IX accounts with just the minimum information
    let open_deal_addresses = idl_program
        .instructions
        .get("open_deal")
        .unwrap()
        .find_addresses_with_accounts_states(
            &program_id,
            &json!({ "global_market_seed": global_market_seed.to_string() }),
            &HashMap::from([
                ("owner".to_string(), owner),
                ("global_market_state".to_string(), global_market_state),
            ]),
            &HashMap::from_iter([(
                "deal".to_string(),
                json!({
                    "deal_number": deal_number,
                    "borrower": borrower.to_string()
                }),
            )]),
        );
    // Check the outcomes
    assert_eq!(
        *open_deal_addresses.get("market_admins").unwrap(),
        market_admins,
    );
    assert_eq!(*open_deal_addresses.get("deal").unwrap(), deal);
    assert_eq!(
        *open_deal_addresses.get("deal_tranches").unwrap(),
        deal_tranches,
    );
    assert_eq!(
        *open_deal_addresses.get("repayment_schedule").unwrap(),
        repayment_schedule,
    );
}
