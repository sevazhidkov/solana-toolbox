use crate::toolbox_idl_program_type_flat::ToolboxIdlProgramTypeFlat;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramTypedef {
    pub name: String,
    pub generics: Vec<String>,
    pub type_flat: ToolboxIdlProgramTypeFlat,
}
