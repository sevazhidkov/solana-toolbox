use std::fs::read_to_string;

use solana_toolbox_idl::ToolboxIdlFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_30.json").unwrap(),
    )
    .unwrap();
    // Test that it's equivalent to the original IDL after being exported
    assert_eq!(
        idl_program,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program.export(&ToolboxIdlFormat::human())
        )
        .unwrap(),
    );
    assert_eq!(
        idl_program,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program.export(&ToolboxIdlFormat::anchor_26())
        )
        .unwrap(),
    );
    assert_eq!(
        idl_program,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program.export(&ToolboxIdlFormat::anchor_30())
        )
        .unwrap(),
    );
}
