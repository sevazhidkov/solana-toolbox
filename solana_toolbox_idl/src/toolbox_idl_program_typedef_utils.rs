use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_program_typedef_primitive::ToolboxIdlProgramTypedefPrimitiveKind;

impl ToolboxIdlProgramTypedef {
    pub fn describe(&self) -> String {
        match self {
            ToolboxIdlProgramTypedef::Defined { name } => format!("@{}", name),
            ToolboxIdlProgramTypedef::Option { content_typedef } => {
                format!("Option<{}>", content_typedef.describe())
            },
            ToolboxIdlProgramTypedef::Vec { items_typedef } => {
                format!("Vec<{}>", items_typedef.describe())
            },
            ToolboxIdlProgramTypedef::Array { length, items_typedef } => {
                format!("[{}; {}]", items_typedef.describe(), length)
            },
            ToolboxIdlProgramTypedef::Struct { fields } => {
                format!(
                    "Struct({})",
                    fields
                        .iter()
                        .map(|field| field.0.clone())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            },
            ToolboxIdlProgramTypedef::Enum { variants } => {
                format!("Enum({})", variants.join(","))
            },
            ToolboxIdlProgramTypedef::Primitive { kind } => {
                kind.as_str().to_string()
            },
        }
    }

    pub fn as_struct_fields(
        &self
    ) -> Option<&Vec<(String, ToolboxIdlProgramTypedef)>> {
        match self {
            ToolboxIdlProgramTypedef::Struct { fields } => Some(fields),
            _ => None,
        }
    }

    pub fn as_primitive_kind(
        &self
    ) -> Option<&ToolboxIdlProgramTypedefPrimitiveKind> {
        match self {
            ToolboxIdlProgramTypedef::Primitive { kind } => Some(kind),
            _ => None,
        }
    }
}
