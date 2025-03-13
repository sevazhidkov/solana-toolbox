use crate::toolbox_idl_program_type_flat::ToolboxIdlProgramTypeFlat;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFull;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub data_type_flat: ToolboxIdlProgramTypeFlat,
    pub data_type_full: ToolboxIdlProgramTypeFull,
}
