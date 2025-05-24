use serde_json::json;
use solana_toolbox_idl::ToolboxIdlError;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
        "errors": [
            {
                "name": "MyError",
                "code": 42,
                "msg": "",
            },
        ],
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
        "errors": [
            {
                "name": "MyError",
                "code": 42,
            },
        ],
    }))
    .unwrap();
    let idl_program3 = ToolboxIdlProgram::try_parse(&json!({
        "errors": {
            "MyError": {
                "code": 42,
                "msg": "",
            },
        },
    }))
    .unwrap();
    let idl_program4 = ToolboxIdlProgram::try_parse(&json!({
        "errors": {
            "MyError": 42,
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl_program1, idl_program2);
    assert_eq!(idl_program1, idl_program3);
    assert_eq!(idl_program1, idl_program4);
    // Assert that the content is correct
    assert_eq!(
        *idl_program1.errors.get("MyError").unwrap(),
        ToolboxIdlError {
            name: "MyError".to_string(),
            code: 42,
            msg: "".to_string(),
        }
        .into()
    )
}
