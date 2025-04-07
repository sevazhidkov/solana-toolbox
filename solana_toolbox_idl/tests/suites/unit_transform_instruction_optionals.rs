use std::collections::HashMap;
use std::vec;

use serde_json::json;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
                "accounts": [
                    { "name": "acc_0" },
                    { "name": "acc_1_1" },
                    { "name": "acc_2_1", "optional": true },
                    { "name": "acc_3_1", "optional": true },
                    { "name": "acc_4_2" },
                    { "name": "acc_5_3" },
                    { "name": "acc_6_3", "optional": true },
                    { "name": "acc_7_3", "optional": true },
                ],
            },
        },
    }))
    .unwrap();
    // Choose the instruction
    let idl_instruction = idl_program.instructions.get("my_ix").unwrap();
    // Use dummy accounts
    let acc_0 = Pubkey::new_unique();
    let acc_1_1 = Pubkey::new_unique();
    let acc_2_1 = Pubkey::new_unique();
    let acc_3_1 = Pubkey::new_unique();
    let acc_4_2 = Pubkey::new_unique();
    let acc_5_3 = Pubkey::new_unique();
    let acc_6_3 = Pubkey::new_unique();
    let acc_7_3 = Pubkey::new_unique();
    // Check that we we can encode the instruction with none of the optional accounts
    let case_empty_addresses = HashMap::from_iter([
        ("acc_0".to_string(), acc_0),
        ("acc_1_1".to_string(), acc_1_1),
        ("acc_4_2".to_string(), acc_4_2),
        ("acc_5_3".to_string(), acc_5_3),
    ]);
    let case_empty_metas = vec![
        AccountMeta::new_readonly(acc_0, false),
        AccountMeta::new_readonly(acc_1_1, false),
        AccountMeta::new_readonly(acc_4_2, false),
        AccountMeta::new_readonly(acc_5_3, false),
    ];
    assert_eq!(
        idl_instruction
            .encode_addresses(&case_empty_addresses)
            .unwrap(),
        case_empty_metas
    );
    assert_eq!(
        idl_instruction.decode_addresses(&case_empty_metas).unwrap(),
        case_empty_addresses,
    );
    // Check that we we can encode the instruction with all of the optional accounts
    let case_full_addresses = HashMap::from_iter([
        ("acc_0".to_string(), acc_0),
        ("acc_1_1".to_string(), acc_1_1),
        ("acc_2_1".to_string(), acc_2_1),
        ("acc_3_1".to_string(), acc_3_1),
        ("acc_4_2".to_string(), acc_4_2),
        ("acc_5_3".to_string(), acc_5_3),
        ("acc_6_3".to_string(), acc_6_3),
        ("acc_7_3".to_string(), acc_7_3),
    ]);
    let case_full_metas = vec![
        AccountMeta::new_readonly(acc_0, false),
        AccountMeta::new_readonly(acc_1_1, false),
        AccountMeta::new_readonly(acc_2_1, false),
        AccountMeta::new_readonly(acc_3_1, false),
        AccountMeta::new_readonly(acc_4_2, false),
        AccountMeta::new_readonly(acc_5_3, false),
        AccountMeta::new_readonly(acc_6_3, false),
        AccountMeta::new_readonly(acc_7_3, false),
    ];
    assert_eq!(
        idl_instruction
            .encode_addresses(&case_full_addresses)
            .unwrap(),
        case_full_metas
    );
    assert_eq!(
        idl_instruction.decode_addresses(&case_full_metas).unwrap(),
        case_full_addresses,
    );
    // Check that we we can encode the instruction with all of the optional accounts
    let case_partial1_addresses = HashMap::from_iter([
        ("acc_0".to_string(), acc_0),
        ("acc_1_1".to_string(), acc_1_1),
        ("acc_2_1".to_string(), acc_2_1),
        ("acc_4_2".to_string(), acc_4_2),
        ("acc_5_3".to_string(), acc_5_3),
    ]);
    let case_partial1_metas = vec![
        AccountMeta::new_readonly(acc_0, false),
        AccountMeta::new_readonly(acc_1_1, false),
        AccountMeta::new_readonly(acc_2_1, false),
        AccountMeta::new_readonly(acc_4_2, false),
        AccountMeta::new_readonly(acc_5_3, false),
    ];
    assert_eq!(
        idl_instruction
            .encode_addresses(&case_partial1_addresses)
            .unwrap(),
        case_partial1_metas
    );
    assert_eq!(
        idl_instruction
            .decode_addresses(&case_partial1_metas)
            .unwrap(),
        case_partial1_addresses,
    );
    // Check that we we can encode the instruction with all of the optional accounts
    let case_partial3_addresses = HashMap::from_iter([
        ("acc_0".to_string(), acc_0),
        ("acc_1_1".to_string(), acc_1_1),
        ("acc_2_1".to_string(), acc_2_1),
        ("acc_3_1".to_string(), acc_3_1),
        ("acc_4_2".to_string(), acc_4_2),
        ("acc_5_3".to_string(), acc_5_3),
        ("acc_6_3".to_string(), acc_6_3),
    ]);
    let case_partial3_metas = vec![
        AccountMeta::new_readonly(acc_0, false),
        AccountMeta::new_readonly(acc_1_1, false),
        AccountMeta::new_readonly(acc_2_1, false),
        AccountMeta::new_readonly(acc_3_1, false),
        AccountMeta::new_readonly(acc_4_2, false),
        AccountMeta::new_readonly(acc_5_3, false),
        AccountMeta::new_readonly(acc_6_3, false),
    ];
    assert_eq!(
        idl_instruction
            .encode_addresses(&case_partial3_addresses)
            .unwrap(),
        case_partial3_metas
    );
    assert_eq!(
        idl_instruction
            .decode_addresses(&case_partial3_metas)
            .unwrap(),
        case_partial3_addresses,
    );
}
