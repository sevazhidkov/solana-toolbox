use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramType {
    pub name: String,
    pub typedef: ToolboxIdlProgramTypedef,
}

impl ToolboxIdlProgramType {
    pub fn print(&self) {
        println!("----");
        println!("type.name: {}", self.name);
        println!("type.typedef: {}", self.typedef.describe());
    }
}
