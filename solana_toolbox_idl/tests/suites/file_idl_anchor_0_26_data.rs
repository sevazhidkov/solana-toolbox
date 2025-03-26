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
    // Prepare instruction args
    let idl_instruction =
        idl_program.instructions.get("initialize_market").unwrap();
    let instruction_payload = json!({
        "global_market_seed": "SEED",
        "withdrawal_fee": {
            "numerator": 41,
            "denominator": 42,
        },
        "credix_fee_percentage": {
            "numerator": 51,
            "denominator": 52,
        },
        "multisig": Pubkey::new_unique().to_string(),
        "managers": [
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
        ],
        "pass_issuers": [
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
        ],
        "withdraw_epoch_request_seconds": 22,
        "withdraw_epoch_redeem_seconds": 23,
        "withdraw_epoch_available_liquidity_seconds": 24,
    });
    // Encode/decode the instruction args and check that they match the original
    assert_eq!(
        idl_instruction
            .decode_payload(
                &idl_instruction
                    .encode_payload(&instruction_payload)
                    .unwrap(),
            )
            .unwrap(),
        instruction_payload,
    );
    // Prepare an account contents
    let idl_account = idl_program.accounts.get("GlobalMarketState").unwrap();
    let account_state = json!({
        "base_token_mint": Pubkey::new_unique().to_string(),
        "lp_token_mint": Pubkey::new_unique().to_string(),
        "pool_outstanding_credit": 5_000_000_000u64,
        "treasury_pool_token_account": Pubkey::new_unique().to_string(),
        "signing_authority_bump": 4,
        "bump": 5,
        "credix_fee_percentage": {
            "numerator": 51,
            "denominator": 52,
        },
        "withdrawal_fee": {
            "numerator": 41,
            "denominator": 42,
        },
        "frozen": true,
        "seed": "Hello World !",
        "pool_size_limit_percentage": {
            "numerator": 61,
            "denominator": 62,
        },
        "withdraw_epoch_request_seconds": 0x42_42_42_01,
        "withdraw_epoch_redeem_seconds": 0x42_42_42_02,
        "withdraw_epoch_available_liquidity_seconds": 0x42_42_42_03,
        "latest_withdraw_epoch_idx": 0x42_42_42_04,
        "latest_withdraw_epoch_end": -42,
        "locked_liquidity": 777_777,
        "total_redeemed_base_amount": 888_888,
        "has_withdraw_epochs": true,
        "redeem_authority_bump": 9,
    });
    // Decode the account content and check that it matches the original
    assert_eq!(
        idl_account
            .decode(&idl_account.encode(&account_state).unwrap())
            .unwrap(),
        account_state,
    );
    // Prepare an account contents
    let idl_account = idl_program.accounts.get("ProgramState").unwrap();
    let account_state = json!({
        "credix_multisig_key": Pubkey::new_unique().to_string(),
        "credix_managers": [
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
        ],
        "credix_treasury": Pubkey::new_unique().to_string(),
    });
    // Decode the account content and check that it matches the original
    assert_eq!(
        idl_account
            .decode(&idl_account.encode(&account_state).unwrap())
            .unwrap(),
        account_state,
    );
}
