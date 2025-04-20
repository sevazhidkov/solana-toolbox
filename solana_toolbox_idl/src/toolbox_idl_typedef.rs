use serde_json::Value;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypedef {
    pub name: String,
    pub docs: Option<Value>,
    pub repr: Option<String>,
    pub generics: Vec<String>,
    pub type_flat: ToolboxIdlTypeFlat,
}
