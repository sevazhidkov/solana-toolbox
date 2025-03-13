use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgramError;
use solana_toolbox_idl::ToolboxIdlProgramRoot;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "errors": [
            {
                "name": "MyError",
                "code": 42,
                "msg": "",
            },
        ],
    }))
    .unwrap();
    let idl2 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "errors": [
            {
                "name": "MyError",
                "code": 42,
            },
        ],
    }))
    .unwrap();
    let idl3 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "errors": {
            "MyError": {
                "code": 42,
                "msg": "",
            },
        },
    }))
    .unwrap();
    let idl4 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "errors": {
            "MyError": 42,
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl1, idl2);
    assert_eq!(idl1, idl3);
    assert_eq!(idl1, idl4);
    // Assert that the content is correct
    assert_eq!(
        idl1.errors.get("MyError").unwrap(),
        &ToolboxIdlProgramError {
            name: "MyError".to_string(),
            code: 42,
            msg: "".to_string(),
        }
    )
}
