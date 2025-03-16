use std::collections::HashMap;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Keys used during the test
    let program_id1 = Pubkey::new_unique();
    let program_id2 = Pubkey::new_unique();
    // Create IDLs on the fly
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [77, 78],
                "accounts": [
                    {
                        "name": "const_bytes_without_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "value": [41, 00, 00, 00] },
                                { "kind": "const", "value": [42, 00, 00, 00] },
                            ]
                        }
                    },
                    {
                        "name": "const_bytes_with_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "value": [41, 00, 00, 00] },
                                { "kind": "const", "value": [42, 00, 00, 00] },
                            ],
                            "program": {
                                "kind": "const",
                                "value": program_id2.to_bytes(),
                            }
                        }
                    },
                    {
                        "name": "const_string_without_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "value": "hello" },
                                { "kind": "const", "value": "world" },
                            ]
                        }
                    },
                    {
                        "name": "const_string_with_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "value": "hello"},
                                { "kind": "const", "value": "world" },
                            ],
                            "program": {
                                "kind": "const",
                                "value": program_id2.to_bytes(),
                            }
                        }
                    },
                ]
            },
        },
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [77, 78],
                "accounts": [
                    {
                        "name": "const_bytes_without_program",
                        "pda": {
                            "seeds": [[41, 00, 00, 00], [42, 00, 00, 00]],
                        }
                    },
                    {
                        "name": "const_bytes_with_program",
                        "pda": {
                            "seeds": [[41, 00, 00, 00], [42, 00, 00, 00]],
                            "program": { "value": program_id2.to_bytes() }
                        }
                    },
                    {
                        "name": "const_string_without_program",
                        "pda": {
                            "seeds": ["hello", "world"]
                        }
                    },
                    {
                        "name": "const_string_with_program",
                        "pda": {
                            "seeds": ["hello", "world"],
                            "program": { "value": program_id2.to_bytes() }
                        }
                    },
                ]
            },
        },
    }))
    .unwrap();
    // Make sure the IDLs are equivalent
    assert_eq!(idl_program1, idl_program2);
    // Pdas based off of const bytes seeds
    let pda_seeds_const_bytes: &[&[u8]] =
        &[&41u32.to_le_bytes(), &42u32.to_le_bytes()];
    let pda_const_bytes1 =
        Pubkey::find_program_address(pda_seeds_const_bytes, &program_id1).0;
    let pda_const_bytes2 =
        Pubkey::find_program_address(pda_seeds_const_bytes, &program_id2).0;
    // Pdas based off of const string seeds
    let pda_seeds_const_string: &[&[u8]] = &[b"hello", b"world"];
    let pda_const_string1 =
        Pubkey::find_program_address(pda_seeds_const_string, &program_id1).0;
    let pda_const_string2 =
        Pubkey::find_program_address(pda_seeds_const_string, &program_id2).0;
    // The instruction we'll use
    let idl_instruction = idl_program1.get_idl_instruction("my_ix").unwrap();
    // Assert that the accounts can be properly resolved
    let instruction_addresses = idl_instruction.find_addresses(
        &program_id1,
        &HashMap::new(),
        &Value::Null,
    );
    assert_eq!(
        *instruction_addresses
            .get("const_bytes_without_program")
            .unwrap(),
        pda_const_bytes1,
    );
    assert_eq!(
        *instruction_addresses
            .get("const_bytes_with_program")
            .unwrap(),
        pda_const_bytes2,
    );
    assert_eq!(
        *instruction_addresses
            .get("const_string_without_program")
            .unwrap(),
        pda_const_string1,
    );
    assert_eq!(
        *instruction_addresses
            .get("const_string_with_program")
            .unwrap(),
        pda_const_string2,
    );
}
