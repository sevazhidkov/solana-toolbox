#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramError {
    pub code: u64,
    pub name: String,
    pub msg: String,
}
