use std::collections::HashMap;

use serde_json::json;
use serde_json::Map;
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
                "args": [{ "name": "arg", "type": {"defined": "MyArg"} }]
            }
        },
        "types": {
            "MyArg": {
                "kind": "struct",
                "fields": [{ "name": "info", "type": "u64" }]
            }
        },
        "accounts": {},
        "errors": {},
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
        args: Map::from_iter([("arg".to_string(), json!({ "info": 42 }))]),
    };
    // Check that we can use the manual IDL to compile/decompile our IX
    assert_eq!(
        instruction,
        idl.decompile_instruction(
            &idl.compile_instruction(&instruction,).unwrap()
        )
        .unwrap()
    );
}
