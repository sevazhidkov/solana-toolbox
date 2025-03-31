use serde_json::json;
use solana_toolbox_idl::ToolboxIdlFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "name": "my_Program",
        "version": "42.42.42",
        "metadata": {
            "docs": ["My Program"],
            "description": "My program description"
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Human),
        json!({
            "metadata": {
                "docs": ["My Program"],
                "name": "MyProgram",
                "version": "42.42.42",
                "description": "My program description",
            },
            "instructions": {},
            "accounts": {},
            "errors": {},
            "types": {},
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Anchor26),
        json!({
            "name": "MyProgram",
            "docs": ["My Program"],
            "description": "My program description",
            "version": "42.42.42",
            "accounts": [],
            "errors": [],
            "instructions": [],
            "types": []
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Anchor30),
        json!({
            "metadata": {
                "docs": ["My Program"],
                "name": "MyProgram",
                "version": "42.42.42",
                "description": "My program description",
            },
            "accounts": [],
            "errors": [],
            "instructions": [],
            "types": []
        })
    );
}
