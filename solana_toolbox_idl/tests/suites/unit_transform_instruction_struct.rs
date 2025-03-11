use std::collections::HashMap;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlTransactionInstruction;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl = ToolboxIdl::try_parse_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "discriminator": [77, 78],
                "accounts": [{ "name": "payer", "signer": true }],
                "args": [
                    { "name": "arg1", "type": {"defined": "MyArg"} },
                    { "name": "arg2", "type": "i16" },
                ]
            }
        },
        "types": {
            "MyArg": {
                "fields": [
                    { "name": "id", "type": "u16" },
                    { "name": "data", "type": {"vec": "u8"} },
                ]
            }
        },
    }))
    .unwrap();
    // Prepare an instruction
    let payer = Keypair::new();
    let instruction = ToolboxIdlTransactionInstruction {
        program_id: Pubkey::new_unique(),
        name: "my_instruction".to_string(),
        accounts_addresses: HashMap::from_iter([(
            "payer".to_string(),
            payer.pubkey(),
        )]),
        args: json!({
            "arg1": {
                "id": 42,
                "data": [1, 2, 3]
            },
            "arg2": -2,
        }),
    };
    // Check that we can use the manual IDL to compile/decompile our IX
    let ix = idl.compile_instruction(&instruction).unwrap();
    assert_eq!(vec![77, 78, 42, 0, 3, 0, 0, 0, 1, 2, 3, 254, 255], ix.data);
    assert_eq!(instruction, idl.decompile_instruction(&ix).unwrap());
}
