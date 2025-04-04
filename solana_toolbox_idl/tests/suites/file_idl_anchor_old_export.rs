use std::fs::read_to_string;

use solana_toolbox_idl::ToolboxIdlFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_old.json").unwrap(),
    )
    .unwrap();
    // Test that it's equivalent to the original IDL after being exported
    assert_eq!(
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program.export(&ToolboxIdlFormat::Human)
        )
        .unwrap(),
        idl_program,
    );
    assert_eq!(
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program.export(&ToolboxIdlFormat::Anchor26)
        )
        .unwrap(),
        idl_program,
    );
    assert_eq!(
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program.export(&ToolboxIdlFormat::Anchor30)
        )
        .unwrap(),
        idl_program,
    );
}
