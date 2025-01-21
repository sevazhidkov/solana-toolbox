use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_string =
        read_to_string("./tests/fixtures/dummy_idl_anchor_0_26.json").unwrap();
    let idl = ToolboxIdl::try_from_str(&idl_string).unwrap();
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
        &[global_market_state.as_ref(), deal.as_ref(), b"repayment-schedule"],
        &program_id,
    )
    .0;
    // Generate all missing IX accounts with just the minimum information
    let initialize_market_accounts = idl
        .fill_instruction_accounts_addresses(
            &program_id,
            "initializeMarket",
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
            json!({}).as_object().unwrap(),
            json!({
                "globalMarketSeed": global_market_seed.to_string(),
            })
            .as_object()
            .unwrap(),
        )
        .unwrap();
    // Check the outcomes
    eprintln!("initialize_market_accounts: {:#?}", initialize_market_accounts);
    assert_eq!(
        global_market_state,
        *initialize_market_accounts.get("globalMarketState").unwrap()
    );
    assert_eq!(
        market_admins,
        *initialize_market_accounts.get("marketAdmins").unwrap()
    );
    assert_eq!(
        program_state,
        *initialize_market_accounts.get("programState").unwrap()
    );
    assert_eq!(
        signing_authority,
        *initialize_market_accounts.get("signingAuthority").unwrap()
    );
    assert_eq!(
        lp_token_mint,
        *initialize_market_accounts.get("lpTokenMint").unwrap()
    );
    // Generate all missing IX accounts with just the minimum information
    let open_deal_accounts = idl
        .fill_instruction_accounts_addresses(
            &program_id,
            "openDeal",
            &HashMap::from([
                ("owner".to_string(), owner),
                ("globalMarketState".to_string(), global_market_state),
            ]),
            json!({
                "deal": {
                    "dealNumber": deal_number,
                    "borrower": borrower.to_string()
                },
            })
            .as_object()
            .unwrap(),
            json!({
                "globalMarketSeed": global_market_seed.to_string(),
            })
            .as_object()
            .unwrap(),
        )
        .unwrap();
    // Check the outcomes
    eprintln!("open_deal_accounts: {:#?}", open_deal_accounts);
    assert_eq!(market_admins, *open_deal_accounts.get("marketAdmins").unwrap());
    assert_eq!(deal, *open_deal_accounts.get("deal").unwrap());
    assert_eq!(deal_tranches, *open_deal_accounts.get("dealTranches").unwrap());
    assert_eq!(
        repayment_schedule,
        *open_deal_accounts.get("repaymentSchedule").unwrap()
    );
}
