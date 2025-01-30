use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl = ToolboxIdl::try_from_value(&json!({
        "instructions": {},
        "accounts": {},
        "types": {
            "Checkpoint": {
                "variants": [
                    { "name": "Dummy" },
                    { "name": "Ephemeral" },
                    { "name": "Hub", "fields": [{ "defined": "HubRepo" }] },
                    { "name": "P2P", "fields": [{ "defined": "HubRepo" }] },
                ]
            },
            "HubRepo": {
                "fields": [
                    { "name": "repo_id", "type": ["u8", 4] },
                    { "name": "revision", "type": { "option": ["u8", 4] } },
                ]
            },
        },
        "errors": {},
    }))
    .unwrap();
    // Lookup instructions and print them
    for program_instruction in idl.program_instructions.values() {
        program_instruction.print();
    }
    // Lookup accounts and print them
    for program_account in idl.program_accounts.values() {
        program_account.print();
    }
    // Lookup types and print them
    for program_type in idl.program_types.values() {
        program_type.print();
    }
    // Lookup errors and print them
    for program_error in idl.program_errors.values() {
        program_error.print();
    }
    panic!("LOL");
}
