use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypedef {
    pub name: String,
    pub generics: Vec<String>,
    pub type_flat: ToolboxIdlTypeFlat,
}
