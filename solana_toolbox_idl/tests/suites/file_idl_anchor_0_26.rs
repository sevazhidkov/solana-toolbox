use std::fs::read_to_string;

use serde_json::Map;
use serde_json::Number;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn file_idl_anchor_0_26() {
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
    let mut instruction_args = Map::new();
    instruction_args
        .insert("globalMarketSeed".into(), Value::String("SEED".into()));
    instruction_args.insert(
        "withdrawalFee".into(),
        Value::Object({
            let mut withdrawal_fee = Map::new();
            withdrawal_fee
                .insert("numerator".into(), Value::Number(Number::from(41)));
            withdrawal_fee
                .insert("denominator".into(), Value::Number(Number::from(42)));
            withdrawal_fee
        }),
    );
    instruction_args.insert(
        "credixFeePercentage".into(),
        Value::Object({
            let mut credix_fee_percentage = Map::new();
            credix_fee_percentage
                .insert("numerator".into(), Value::Number(Number::from(51)));
            credix_fee_percentage
                .insert("denominator".into(), Value::Number(Number::from(52)));
            credix_fee_percentage
        }),
    );
    instruction_args
        .insert("multisig".into(), Value::String(placeholder.to_string()));
    instruction_args.insert(
        "managers".into(),
        Value::Array({
            let mut managers = vec![];
            managers.push(Value::String(placeholder.to_string()));
            managers.push(Value::String(placeholder.to_string()));
            managers
        }),
    );
    instruction_args.insert(
        "passIssuers".into(),
        Value::Array({
            let mut pass_issuers = vec![];
            pass_issuers.push(Value::String(placeholder.to_string()));
            pass_issuers.push(Value::String(placeholder.to_string()));
            pass_issuers
        }),
    );
    instruction_args.insert(
        "withdrawEpochRequestSeconds".into(),
        Value::Number(Number::from(22)),
    );
    instruction_args.insert(
        "withdrawEpochRedeemSeconds".into(),
        Value::Number(Number::from(23)),
    );
    instruction_args.insert(
        "withdrawEpochAvailableLiquiditySeconds".into(),
        Value::Number(Number::from(24)),
    );
    // Generate an instruction
    let instruction_data = idl
        .compile_instruction_data("initializeMarket", &instruction_args)
        .unwrap();
    // Decompile the instruction args and check that they match the original
    assert_eq!(
        instruction_args,
        idl.decompile_instruction_data("initializeMarket", &instruction_data)
            .unwrap()
    );
}
