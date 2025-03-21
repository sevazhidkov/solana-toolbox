use std::collections::HashMap;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Keys used during the test
    let dummy_address = Pubkey::new_unique();
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
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
    // Assert that the accounts can be properly resolved
    let instruction_addresses = idl_program
        .instructions
        .get("my_ix")
        .unwrap()
        .find_addresses(&Pubkey::new_unique(), &Value::Null, &HashMap::new());
    assert_eq!(
        *instruction_addresses.get("const_address").unwrap(),
        dummy_address,
    );
}
