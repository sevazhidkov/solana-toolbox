use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlFormat;
use solana_toolbox_idl_core::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "address": "11111111111111111111111111111111",
        "docs": ["My Program"],
        "name": "my_Program",
        "version": "42.42.41",
        "metadata": {
            "description": "My program description",
            "spec": "222",
            "version": "42.42.42",
        },
    }))
    .unwrap();
    // Expected parsed info
    let metadata = json!({
        "address": "11111111111111111111111111111111",
        "name": "my_Program",
        "description": "My program description",
        "docs": ["My Program"],
        "version": "42.42.42",
        "spec": "222",
    });
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::human()),
        json!({
            "metadata": metadata,
            "instructions": {},
            "accounts": {},
            "events": {},
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
            "name": "my_Program",
            "description": "My program description",
            "version": "42.42.42",
            "spec": "222",
            "metadata": metadata,
            "instructions": [],
            "accounts": [],
            "events": [],
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
            "name": "my_Program",
            "description": "My program description",
            "version": "42.42.42",
            "spec": "222",
            "metadata": metadata,
            "instructions": [],
            "accounts": [],
            "events": [],
            "errors": [],
            "types": []
        })
    );
}
