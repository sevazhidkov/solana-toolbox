use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlProgramError;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_from_value(&json!({
        "errors": [
            {
                "name": "MyError",
                "code": 42,
                "msg": "",
            },
        ],
    }))
    .unwrap();
    let idl2 = ToolboxIdl::try_from_value(&json!({
        "errors": [
            {
                "name": "MyError",
                "code": 42,
            },
        ],
    }))
    .unwrap();
    let idl3 = ToolboxIdl::try_from_value(&json!({
        "errors": {
            "MyError": {
                "code": 42,
                "msg": "",
            },
        },
    }))
    .unwrap();
    let idl4 = ToolboxIdl::try_from_value(&json!({
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
        idl1.program_errors.get(&42).unwrap(),
        &ToolboxIdlProgramError {
            name: "MyError".to_string(),
            code: 42,
            msg: "".to_string(),
        }
    )
}
