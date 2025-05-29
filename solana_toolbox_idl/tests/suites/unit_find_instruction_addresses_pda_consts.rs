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
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [77, 78],
                "accounts": [
                    {
                        "name": "const_bytes_without_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "type": ["u8"], "value": [41, 0, 0, 0] },
                                { "kind": "const", "value": [42, 0, 0, 0] },
                            ]
                        }
                    },
                    {
                        "name": "const_bytes_with_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "type": ["u8"], "value": [41, 0, 0, 0] },
                                { "kind": "const", "value": [42, 0, 0, 0] },
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
                                { "kind": "const", "type": "string", "value": "hello" },
                                { "kind": "const", "type": "string", "value": "world" },
                            ]
                        }
                    },
                    {
                        "name": "const_string_with_program",
                        "pda": {
                            "seeds": [
                                { "kind": "const", "type": "string", "value": "hello"},
                                { "kind": "const", "type": "string", "value": "world" },
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
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [77, 78],
                "accounts": [
                    {
                        "name": "const_bytes_without_program",
                        "pda": {
                            "seeds": [[41, 0, 0, 0], [42, 0, 0, 0]],
                        }
                    },
                    {
                        "name": "const_bytes_with_program",
                        "pda": {
                            "seeds": [[41, 0, 0, 0], [42, 0, 0, 0]],
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
    // Assert that the accounts can be properly resolved
    let instruction_addresses = idl_program1
        .instructions
        .get("my_ix")
        .unwrap()
        .find_addresses(&program_id1, &Value::Null, &HashMap::new());
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
