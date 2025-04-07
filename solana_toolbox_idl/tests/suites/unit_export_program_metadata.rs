use serde_json::json;
use solana_toolbox_idl::ToolboxIdlFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "address": "11111111111111111111111111111111",
        "docs": ["My Program"],
        "name": "my_Program",
        "version": "42.42.42",
        "metadata": {
            "description": "My program description",
            "spec": "222",
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::human()),
        json!({
            "address": "11111111111111111111111111111111",
            "docs": ["My Program"],
            "metadata": {
                "name": "MyProgram",
                "description": "My program description",
                "version": "42.42.42",
                "spec": "222",
            },
            "instructions": {},
            "accounts": {},
            "errors": {},
            "types": {},
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_26()),
        json!({
            "address": "11111111111111111111111111111111",
            "docs": ["My Program"],
            "name": "MyProgram",
            "description": "My program description",
            "version": "42.42.42",
            "spec": "222",
            "instructions": [],
            "accounts": [],
            "errors": [],
            "types": []
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_30()),
        json!({
            "address": "11111111111111111111111111111111",
            "docs": ["My Program"],
            "metadata": {
                "name": "MyProgram",
                "description": "My program description",
                "version": "42.42.42",
                "spec": "222",
            },
            "instructions": [],
            "accounts": [],
            "errors": [],
            "types": []
        })
    );
}
