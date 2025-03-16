use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::{ToolboxIdlBreadcrumbs, ToolboxIdlProgram};

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_0_26.json").unwrap(),
    )
    .unwrap();
    // Prepare instruction args
    let idl_instruction =
        idl_program.get_idl_instruction("initializeMarket").unwrap();
    let instruction_payload = json!({
        "globalMarketSeed": "SEED",
        "withdrawalFee": {
            "numerator": 41,
            "denominator": 42,
        },
        "credixFeePercentage": {
            "numerator": 51,
            "denominator": 52,
        },
        "multisig": Pubkey::new_unique().to_string(),
        "managers": [
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
        ],
        "passIssuers": [
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
        ],
        "withdrawEpochRequestSeconds": 22,
        "withdrawEpochRedeemSeconds": 23,
        "withdrawEpochAvailableLiquiditySeconds": 24,
    });
    // Compile / decompile the instruction args and check that they match the original
    assert_eq!(
        instruction_payload,
        idl_instruction
            .decompile_payload(
                &idl_instruction
                    .compile_payload(
                        &instruction_payload,
                        &ToolboxIdlBreadcrumbs::default() // TODO - this should not be needed
                    )
                    .unwrap(),
                &ToolboxIdlBreadcrumbs::default()
            )
            .unwrap()
    );
    // Prepare an account contents
    let idl_account = idl_program.get_idl_account("GlobalMarketState").unwrap();
    let account_state = json!({
        "baseTokenMint": Pubkey::new_unique().to_string(),
        "lpTokenMint": Pubkey::new_unique().to_string(),
        "poolOutstandingCredit": 5_000_000_000u64,
        "treasuryPoolTokenAccount": Pubkey::new_unique().to_string(),
        "signingAuthorityBump": 4,
        "bump": 5,
        "credixFeePercentage": {
            "numerator": 51,
            "denominator": 52,
        },
        "withdrawalFee": {
            "numerator": 41,
            "denominator": 42,
        },
        "frozen": true,
        "seed": "Hello World !",
        "poolSizeLimitPercentage": {
            "numerator": 61,
            "denominator": 62,
        },
        "withdrawEpochRequestSeconds": 0x42_42_42_01,
        "withdrawEpochRedeemSeconds": 0x42_42_42_02,
        "withdrawEpochAvailableLiquiditySeconds": 0x42_42_42_03,
        "latestWithdrawEpochIdx": 0x42_42_42_04,
        "latestWithdrawEpochEnd": -42,
        "lockedLiquidity": 777_777,
        "totalRedeemedBaseAmount": 888_888,
        "hasWithdrawEpochs": true,
        "redeemAuthorityBump": 9,
    });
    // Decompile the account content and check that it matches the original
    assert_eq!(
        account_state,
        idl_account
            .decompile(&idl_account.compile(&account_state).unwrap())
            .unwrap()
    );
    // Prepare an account contents
    let idl_account = idl_program.get_idl_account("ProgramState").unwrap();
    let account_state = json!({
        "credixMultisigKey": Pubkey::new_unique().to_string(),
        "credixManagers": [
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
        "credixTreasury": Pubkey::new_unique().to_string(),
    });
    // Decompile the account content and check that it matches the original
    assert_eq!(
        account_state,
        idl_account
            .decompile(&idl_account.compile(&account_state).unwrap())
            .unwrap()
    );
}
