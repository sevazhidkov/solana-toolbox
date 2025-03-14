// TODO - The naming on this is not great
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTransactionError {
    pub code: u64,
    pub name: String,
    pub msg: String,
}
