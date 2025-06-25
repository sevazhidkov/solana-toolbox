use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlError {
    pub name: String,
    pub docs: Option<Value>,
    pub code: u64,
    pub msg: Option<String>,
}

impl Default for ToolboxIdlError {
    fn default() -> ToolboxIdlError {
        ToolboxIdlError {
            name: "UnknownError".to_string(),
            docs: None,
            code: 0xFFFFFFFF,
            msg: None,
        }
    }
}
