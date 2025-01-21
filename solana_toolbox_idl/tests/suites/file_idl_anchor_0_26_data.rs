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
    // Prepare instruction args
    let instruction_args_value = json!({
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
    // Compile the instruction data
    let instruction_data = &idl
        .compile_instruction_data(
            "initializeMarket",
            instruction_args_value.as_object().unwrap(),
        )
        .unwrap()[..];
    // Decompile the instruction args and check that they match the original
    assert_eq!(
        instruction_args_value.as_object().unwrap(),
        &idl.decompile_instruction_data("initializeMarket", instruction_data)
            .unwrap()
    );
    // Prepare an account contents
    let account_value = json!({
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
    // Compile the account data
    let account_data =
        &idl.compile_account("GlobalMarketState", &account_value).unwrap()[..];
    // Decompile the account content and check that it matches the original
    assert_eq!(
        (account_data.len(), account_value),
        idl.decompile_account("GlobalMarketState", account_data).unwrap()
    );
    // Prepare an account contents
    let account_value = json!({
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
    // Compile the account data
    let account_data =
        &idl.compile_account("ProgramState", &account_value).unwrap()[..];
    // Decompile the account content and check that it matches the original
    assert_eq!(
        (account_data.len(), account_value),
        idl.decompile_account("ProgramState", account_data).unwrap()
    );
}
