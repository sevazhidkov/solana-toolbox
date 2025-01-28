use std::fs::read_to_string;

use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_string =
        read_to_string("./tests/fixtures/idl_anchor_0_26.json").unwrap();
    let idl = ToolboxIdl::try_from_str(&idl_string).unwrap();
    // Lookup error by code
    let program_error = idl.program_errors.get(&6004).unwrap();
    assert_eq!("MarketIsFrozen", program_error.name);
    assert_eq!(
        "This market is currently frozen. Please try again later.",
        program_error.msg,
    );
    /*
    // Lookup instructions and print them
    for lookup_instruction in idl.lookup_instructions().unwrap() {
        lookup_instruction.print();
    }
    // Lookup accounts and print them
    for lookup_account in idl.lookup_accounts().unwrap() {
        lookup_account.print();
    }
    // Lookup types and print them
    for lookup_type in idl.lookup_types().unwrap() {
        lookup_type.print();
    }
    // Lookup errors and print them
    for program_error in idl.program_errors.values() {
        program_error.print();
    }
    */
}
