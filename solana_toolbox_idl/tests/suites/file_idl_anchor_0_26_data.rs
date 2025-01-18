use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl = ToolboxIdl::try_from_str(
        &read_to_string("./tests/fixtures/dummy_idl_anchor_0_26.json").unwrap(),
    )
    .unwrap();
    // Generate a custom dummy key
    let placeholder = Pubkey::new_from_array([
        77, 77, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 77, 77,
    ]);
    // Prepare instruction args
    let instruction_args = json!({
        "globalMarketSeed": "SEED",
        "withdrawalFee": {
            "numerator": 41,
            "denominator": 42,
        },
        "credixFeePercentage": {
            "numerator": 51,
            "denominator": 52,
        },
        "multisig": placeholder.to_string(),
        "managers": [
            placeholder.to_string(),
            placeholder.to_string(),
        ],
        "passIssuers": [
            placeholder.to_string(),
            placeholder.to_string(),
            placeholder.to_string(),
        ],
        "withdrawEpochRequestSeconds": 22,
        "withdrawEpochRedeemSeconds": 23,
        "withdrawEpochAvailableLiquiditySeconds": 24,
    });
    // Compile the instruction data
    let instruction_data = &idl
        .compile_instruction_data(
            "initializeMarket",
            instruction_args.as_object().unwrap(),
        )
        .unwrap()[..];
    // Decompile the instruction args and check that they match the original
    assert_eq!(
        instruction_args.as_object().unwrap(),
        &idl.decompile_instruction_data("initializeMarket", &instruction_data)
            .unwrap()
    );
    // Prepare an account contents
    let account_value = json!({
        "baseTokenMint": placeholder.to_string(),
        "lpTokenMint": placeholder.to_string(),
        "poolOutstandingCredit": 5_000_000_000u64,
        "treasuryPoolTokenAccount": placeholder.to_string(),
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
        "withdrawEpochRequestSeconds": 424242_01,
        "withdrawEpochRedeemSeconds": 424242_02,
        "withdrawEpochAvailableLiquiditySeconds": 424242_03,
        "latestWithdrawEpochIdx": 424242_04,
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
        idl.decompile_account("GlobalMarketState", &account_data).unwrap()
    );
}
