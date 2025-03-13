use crate::toolbox_idl_program_type_flat::ToolboxIdlProgramTypeFlat;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFull;

// TODO - should we simply expose the account API directly on this struct ?
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub data_type_flat: ToolboxIdlProgramTypeFlat,
    pub data_type_full: ToolboxIdlProgramTypeFull,
}
