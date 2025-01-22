use std::fs::read_to_string;

use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_string =
        read_to_string("./tests/fixtures/dummy_idl_anchor_0_26.json").unwrap();
    let idl = ToolboxIdl::try_from_str(&idl_string).unwrap();
    // Lookup error by code
    assert_eq!("MarketIsFrozen", idl.lookup_error_by_code(6004).unwrap().name);
    assert_eq!(
        "This market is currently frozen. Please try again later.",
        idl.lookup_error_by_code(6004).unwrap().msg,
    );
    // Lookup instructions and print them
    let lookup_instructions = idl.lookup_instructions().unwrap();
    for lookup_instruction in lookup_instructions {
        lookup_instruction.print();
    }
    // Lookup accounts and print them
    let lookup_accounts = idl.lookup_accounts().unwrap();
    for lookup_account in lookup_accounts {
        lookup_account.print();
    }
    // Lookup types and print them
    let lookup_types = idl.lookup_types().unwrap();
    for lookup_type in lookup_types {
        lookup_type.print();
    }
    // Lookup errors and print them
    let lookup_errors = idl.lookup_errors().unwrap();
    for lookup_error in lookup_errors {
        lookup_error.print();
    }
}
