#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlError {
    pub name: String,
    pub code: u64,
    pub msg: String,
}

impl Default for ToolboxIdlError {
    fn default() -> ToolboxIdlError {
        ToolboxIdlError {
            name: "UnknownError".to_string(),
            code: 0xFFFFFFFF,
            msg: "Unknown error has happened and couldnt be parsed".to_string(),
        }
    }
}
