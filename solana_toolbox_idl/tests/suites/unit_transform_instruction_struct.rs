use std::collections::HashMap;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlInstruction;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl = ToolboxIdl::try_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "accounts": [{ "name": "payer", "signer": true }],
                "args": [
                    { "name": "arg1", "type": {"defined": "MyArg"} },
                    { "name": "arg2", "type": "i32" },
                ]
            }
        },
        "types": {
            "MyArg": {
                "fields": [
                    { "name": "id", "type": "u64" },
                    { "name": "data", "type": {"vec": "u64"} },
                ]
            }
        },
    }))
    .unwrap();
    // Prepare an instruction
    let payer = Keypair::new();
    let instruction = ToolboxIdlInstruction {
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
            "arg2": -32,
        }),
    };
    // Check that we can use the manual IDL to compile/decompile our IX
    assert_eq!(
        instruction,
        idl.decompile_instruction(
            &idl.compile_instruction(&instruction).unwrap()
        )
        .unwrap()
    );
}
