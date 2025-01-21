use std::fs::read_to_string;

use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_string =
        read_to_string("./tests/fixtures/dummy_idl_anchor_0_26.json").unwrap();
    let idl = ToolboxIdl::try_from_str(&idl_string).unwrap();
    // Lookup error by code
    assert_eq!("MarketIsFrozen", idl.lookup_error_by_code(6004).unwrap().name,);
    assert_eq!(
        "This market is currently frozen. Please try again later.",
        idl.lookup_error_by_code(6004).unwrap().msg,
    );
    // Lookup instructions and print them
    let instructions = idl.lookup_instructions().unwrap();
    for instruction in instructions {
        instruction.print();
    }
}
