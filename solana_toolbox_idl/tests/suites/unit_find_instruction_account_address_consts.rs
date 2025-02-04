use std::collections::HashMap;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlInstruction;

#[tokio::test]
pub async fn run() {
    // Keys used during the test
    let dummy_address = Pubkey::new_unique();
    let dummy_program_id1 = Pubkey::new_unique();
    let dummy_program_id2 = Pubkey::new_unique();
    // Create an IDL on the fly
    let idl1 = ToolboxIdl::try_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "discriminator": [77, 78],
                "accounts": [
                    {
                        "name": "const_address",
                        "address": dummy_address.to_string()
                    },
                    {
                        "name": "const_pda_bytes_without_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "value": [41, 00, 00, 00] },
                                { "kind": "const", "value": [42, 00, 00, 00] },
                            ]
                        }
                    },
                    {
                        "name": "const_pda_bytes_with_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "value": [41, 00, 00, 00] },
                                { "kind": "const", "value": [42, 00, 00, 00] },
                            ],
                            "program": {
                                "kind": "const",
                                "value": dummy_program_id2.to_bytes(),
                            }
                        }
                    },
                    {
                        "name": "const_pda_string_without_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "value": "hello" },
                                { "kind": "const", "value": "world" },
                            ]
                        }
                    },
                    {
                        "name": "const_pda_string_with_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "value": "hello"},
                                { "kind": "const", "value": "world" },
                            ],
                            "program": {
                                "kind": "const",
                                "value": dummy_program_id2.to_bytes(),
                            }
                        }
                    },
                ]
            },
        },
    }))
    .unwrap();
    // Pdas based off of bytes seeds
    let dummy_pda_seeds_bytes: &[&[u8]] =
        &[&41u32.to_le_bytes(), &42u32.to_le_bytes()];
    let dummy_pda_bytes1 =
        Pubkey::find_program_address(dummy_pda_seeds_bytes, &dummy_program_id1)
            .0;
    let dummy_pda_bytes2 =
        Pubkey::find_program_address(dummy_pda_seeds_bytes, &dummy_program_id2)
            .0;
    // Pdas based off of string seeds
    let dummy_pda_seeds_string: &[&[u8]] = &[b"hello", b"world"];
    eprintln!("dummy_pda_seeds_string:{:?}", dummy_pda_seeds_string);
    let dummy_pda_string1 = Pubkey::find_program_address(
        dummy_pda_seeds_string,
        &dummy_program_id1,
    )
    .0;
    let dummy_pda_string2 = Pubkey::find_program_address(
        dummy_pda_seeds_string,
        &dummy_program_id2,
    )
    .0;
    // The instruction we'll use
    let instruction = ToolboxIdlInstruction {
        program_id: dummy_program_id1,
        name: "my_instruction".to_string(),
        accounts_addresses: HashMap::new(),
        args: Value::Null,
    };
    // Assert that the accounts can be properly resolved
    assert_eq!(
        dummy_address,
        idl1.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "const_address",
        )
        .unwrap()
    );
    assert_eq!(
        dummy_pda_bytes1,
        idl1.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "const_pda_bytes_without_program",
        )
        .unwrap()
    );
    assert_eq!(
        dummy_pda_bytes2,
        idl1.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "const_pda_bytes_with_program",
        )
        .unwrap()
    );
    assert_eq!(
        dummy_pda_string1,
        idl1.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "const_pda_string_without_program",
        )
        .unwrap()
    );
    assert_eq!(
        dummy_pda_string2,
        idl1.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "const_pda_string_with_program",
        )
        .unwrap()
    );
}
