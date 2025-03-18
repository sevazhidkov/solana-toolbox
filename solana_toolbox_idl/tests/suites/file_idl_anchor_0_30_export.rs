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
        "idl_program.as_json(false): {}",
        serde_json::to_string_pretty(
            &idl_program
                .as_json(false)
                .pointer("/instructions/campaign_create")
        )
        .unwrap()
    );
    assert_eq!(
        idl_program,
        ToolboxIdlProgram::try_parse_from_value(&idl_program.as_json(false))
            .unwrap(),
    );
    assert_eq!(
        idl_program,
        ToolboxIdlProgram::try_parse_from_value(&idl_program.as_json(true))
            .unwrap(),
    );
}
