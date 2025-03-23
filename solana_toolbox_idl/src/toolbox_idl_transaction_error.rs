// TODO - The naming on this is not great (maybe execution/outcome/?)
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTransactionError {
    pub code: u64,
    pub name: String,
    pub msg: String,
}

impl Default for ToolboxIdlTransactionError {
    fn default() -> ToolboxIdlTransactionError {
        ToolboxIdlTransactionError {
            code: 0xFFFFFFFF,
            name: "UnknownError".to_string(),
            msg: "Unknown error has happened and couldnt be parsed".to_string(),
        }
    }
}
