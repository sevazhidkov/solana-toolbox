use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_0_26.json").unwrap(),
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
        .get_idl_instruction("initializeMarket")
        .unwrap()
        .find_addresses(
            &program_id,
            &HashMap::from([
                ("owner".to_string(), owner),
                (
                    "liquidityPoolTokenAccount".to_string(),
                    liquidity_pool_token_account,
                ),
                ("treasury".to_string(), treasury),
                (
                    "treasuryPoolTokenAccount".to_string(),
                    treasury_pool_token_account,
                ),
                ("baseTokenMint".to_string(), base_token_mint),
                ("associatedTokenProgram".to_string(), placeholder),
                ("rent".to_string(), placeholder),
                ("tokenProgram".to_string(), placeholder),
                ("systemProgram".to_string(), placeholder),
            ]),
            &json!({ "globalMarketSeed": global_market_seed.to_string() }),
        );
    // Check the outcomes
    assert_eq!(
        global_market_state,
        *initialize_market_addresses
            .get("globalMarketState")
            .unwrap()
    );
    assert_eq!(
        market_admins,
        *initialize_market_addresses.get("marketAdmins").unwrap()
    );
    assert_eq!(
        program_state,
        *initialize_market_addresses.get("programState").unwrap()
    );
    assert_eq!(
        signing_authority,
        *initialize_market_addresses.get("signingAuthority").unwrap()
    );
    assert_eq!(
        lp_token_mint,
        *initialize_market_addresses.get("lpTokenMint").unwrap()
    );
    // Generate all missing IX accounts with just the minimum information
    let open_deal_addresses = idl_program
        .get_idl_instruction("openDeal")
        .unwrap()
        .find_addresses_with_snapshots(
            &program_id,
            &HashMap::from([
                ("owner".to_string(), owner),
                ("globalMarketState".to_string(), global_market_state),
            ]),
            &json!({ "globalMarketSeed": global_market_seed.to_string() }),
            &HashMap::from_iter([(
                "deal".to_string(),
                (
                    idl_program
                        .get_idl_account("Deal")
                        .unwrap()
                        .content_type_full
                        .clone(),
                    json!({
                        "dealNumber": deal_number,
                        "borrower": borrower.to_string()
                    }),
                ),
            )]),
        );
    // Check the outcomes
    assert_eq!(
        market_admins,
        *open_deal_addresses.get("marketAdmins").unwrap()
    );
    assert_eq!(deal, *open_deal_addresses.get("deal").unwrap());
    assert_eq!(
        deal_tranches,
        *open_deal_addresses.get("dealTranches").unwrap()
    );
    assert_eq!(
        repayment_schedule,
        *open_deal_addresses.get("repaymentSchedule").unwrap()
    );
}
