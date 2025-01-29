use std::fs::read_to_string;

use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_string =
        read_to_string("./tests/fixtures/idl_anchor_0_30.json").unwrap();
    let idl = ToolboxIdl::try_from_str(&idl_string).unwrap();
    // Lookup error by code
    let program_error = idl.program_errors.get(&6002).unwrap();
    assert_eq!("CampaignFundingPhaseHasEnded", program_error.name);
    assert_eq!("The campaign funding phase has ended", program_error.msg);
    // Lookup instructions and print them
    for program_instruction in idl.program_instructions.values() {
        program_instruction.print();
    }
    // Lookup accounts and print them
    for program_account in idl.program_accounts.values() {
        program_account.print();
    }
    // Lookup errors and print them
    for program_error in idl.program_errors.values() {
        program_error.print();
    }
}
