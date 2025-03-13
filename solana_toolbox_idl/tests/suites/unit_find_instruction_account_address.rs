use std::collections::HashMap;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgramRoot;
use solana_toolbox_idl::ToolboxIdlTransactionInstruction;

#[tokio::test]
pub async fn run() {
    // Keys used during the test
    let dummy_address = Pubkey::new_unique();
    // Create an IDL on the fly
    let idl = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "discriminator": [77, 78],
                "accounts": [
                    {
                        "name": "const_address",
                        "address": dummy_address.to_string()
                    },
                ]
            },
        },
    }))
    .unwrap();
    // The instruction we'll use
    let instruction = ToolboxIdlTransactionInstruction {
        program_id: Pubkey::new_unique(),
        name: "my_instruction".to_string(),
        accounts_addresses: HashMap::new(),
        args: Value::Null,
    };
    // Assert that the accounts can be properly resolved
    assert_eq!(
        dummy_address,
        idl.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "const_address",
        )
        .unwrap()
    );
}
