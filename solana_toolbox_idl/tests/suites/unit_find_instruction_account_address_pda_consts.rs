use std::collections::HashMap;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgramRoot;
use solana_toolbox_idl::ToolboxIdlTransactionInstruction;

#[tokio::test]
pub async fn run() {
    // Keys used during the test
    let program_id1 = Pubkey::new_unique();
    let program_id2 = Pubkey::new_unique();
    // Create IDLs on the fly
    let idl1 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "instructions": {
            "my_instruction": {
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
    let idl2 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "instructions": {
            "my_instruction": {
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
    assert_eq!(idl1, idl2);
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
    let instruction = ToolboxIdlTransactionInstruction {
        program_id: program_id1,
        name: "my_instruction".to_string(),
        accounts_addresses: HashMap::new(),
        args: Value::Null,
    };
    // Assert that the accounts can be properly resolved
    assert_eq!(
        pda_const_bytes1,
        idl1.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "const_bytes_without_program",
        )
        .unwrap()
    );
    assert_eq!(
        pda_const_bytes2,
        idl1.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "const_bytes_with_program",
        )
        .unwrap()
    );
    assert_eq!(
        pda_const_string1,
        idl1.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "const_string_without_program",
        )
        .unwrap()
    );
    assert_eq!(
        pda_const_string2,
        idl1.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "const_string_with_program",
        )
        .unwrap()
    );
}
