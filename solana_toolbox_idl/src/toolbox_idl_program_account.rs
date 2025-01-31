use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub type_flat: ToolboxIdlTypeFlat,
    pub type_full: ToolboxIdlTypeFull,
}

impl ToolboxIdlProgramAccount {
    pub fn print(&self) {
        println!("----");
        println!("account.name: {}", self.name);
        println!("account.discriminator: {:?}", self.discriminator);
        println!("account.type: {}", self.type_flat.describe());
    }
}
