use crate::{
    ToolboxIdlProgramInstructionAccount, ToolboxIdlProgramTypeFlatFields,
    ToolboxIdlProgramTypeFullFields,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstruction {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<ToolboxIdlProgramInstructionAccount>,
    pub args_type_flat_fields: ToolboxIdlProgramTypeFlatFields,
    pub args_type_full_fields: ToolboxIdlProgramTypeFullFields,
}
