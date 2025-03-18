use std::fs::read_to_string;

use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_0_30.json").unwrap(),
    )
    .unwrap();
    // Test that it's equivalent to the original IDL after being exported
    eprintln!(
        "idl_program.export(false): {}",
        serde_json::to_string_pretty(
            &idl_program
                .export(false)
                .pointer("/instructions/campaign_create")
        )
        .unwrap()
    );
    let idl_program2 =
        ToolboxIdlProgram::try_parse_from_value(&idl_program.export(false))
            .unwrap();
    for idl_instruction in idl_program.instructions.values() {
        let idl_instruction2 = idl_program2
            .instructions
            .get(&idl_instruction.name)
            .unwrap();
        eprintln!("idl_instruction.name: {}", idl_instruction.name);
        assert_eq!(idl_instruction, idl_instruction2);
    }
    for idl_account in idl_program.accounts.values() {
        let idl_account2 =
            idl_program2.accounts.get(&idl_account.name).unwrap();
        eprintln!("idl_account.name: {}", idl_account.name);
        assert_eq!(idl_account, idl_account2);
    }
    for idl_typedef in idl_program.typedefs.values() {
        let idl_typedef2 =
            idl_program2.typedefs.get(&idl_typedef.name).unwrap();
        eprintln!("idl_typedef.name: {}", idl_typedef.name);
        assert_eq!(idl_typedef, idl_typedef2);
    }
    for idl_error in idl_program.errors.values() {
        let idl_error2 = idl_program2.errors.get(&idl_error.name).unwrap();
        eprintln!("idl_error.name: {}", idl_error.name);
        assert_eq!(idl_error, idl_error2);
    }
    assert_eq!(idl_program, idl_program2,);
    assert_eq!(
        idl_program,
        ToolboxIdlProgram::try_parse_from_value(&idl_program.export(true))
            .unwrap(),
    );
}
