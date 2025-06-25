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
            },
        ],
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
        "errors": {
            "MyError": 42,
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl_program1, idl_program2);
    // Assert that the content is correct
    assert_eq!(
        *idl_program1.errors.get("MyError").unwrap(),
        ToolboxIdlError {
            name: "MyError".to_string(),
            docs: None,
            code: 42,
            msg: None,
        }
        .into()
    )
}
