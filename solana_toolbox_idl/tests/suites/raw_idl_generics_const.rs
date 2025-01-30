use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl = ToolboxIdl::try_from_value(&json!({
        "instructions": {},
        "accounts": {},
        "types": {
            "Bloom": {
                "generics": [
                    {
                      "kind": "const",
                      "name": "K",
                      "type": "usize"
                    }
                ],
                "type": {
                    "fields": [
                        { "name": "keys", "type": ["u64", {"generic": "K"}] },
                    ]
                }
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
    eprintln!("IDL:{:#?}", idl);
    panic!("LOL");
    //let dada = let
}
