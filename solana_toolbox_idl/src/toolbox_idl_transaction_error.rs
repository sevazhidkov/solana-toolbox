// TODO (MEDIUM) - The naming on this is not great (maybe execution/outcome/?)
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTransactionError {
    pub name: String,
    pub code: u64,
    pub msg: String,
}

impl Default for ToolboxIdlTransactionError {
    fn default() -> ToolboxIdlTransactionError {
        ToolboxIdlTransactionError {
            name: "UnknownError".to_string(),
            code: 0xFFFFFFFF,
            msg: "Unknown error has happened and couldnt be parsed".to_string(),
        }
    }
}
