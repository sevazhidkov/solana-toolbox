use std::fs::read_to_string;

use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_string =
        read_to_string("./tests/fixtures/dummy_idl_anchor_0_30.json").unwrap();
    let idl = ToolboxIdl::try_from_str(&idl_string).unwrap();
    // Lookup error by code
    assert_eq!(
        "CampaignFundingPhaseHasEnded",
        idl.lookup_error_by_code(6002).unwrap().name,
    );
    assert_eq!(
        "The campaign funding phase has ended",
        idl.lookup_error_by_code(6002).unwrap().msg,
    );
    // Lookup instructions and print them
    let instructions = idl.lookup_instructions().unwrap();
    for instruction in instructions {
        instruction.print();
    }
}
