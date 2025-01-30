use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_program_typedef_primitive::ToolboxIdlProgramTypedefPrimitive;

impl ToolboxIdlProgramTypedef {
    pub fn describe(&self) -> String {
        match self {
            ToolboxIdlProgramTypedef::Defined { name, generics } => {
                if generics.is_empty() {
                    format!("@{}", name)
                } else {
                    format!(
                        "@{}<{}>",
                        name,
                        generics
                            .iter()
                            .map(|generic| generic.describe())
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                }
            },
            ToolboxIdlProgramTypedef::Option { content_typedef } => {
                format!("Option<{}>", content_typedef.describe())
            },
            ToolboxIdlProgramTypedef::Vec { items_typedef } => {
                format!("Vec<{}>", items_typedef.describe())
            },
            ToolboxIdlProgramTypedef::Array { length, items_typedef } => {
                format!("[{};{}]", items_typedef.describe(), length)
            },
            ToolboxIdlProgramTypedef::Struct { fields } => {
                format!(
                    "Struct{{{}}}",
                    fields
                        .iter()
                        .map(|field| {
                            format!("{}:{}", field.0, field.1.describe())
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                )
            },
            ToolboxIdlProgramTypedef::Enum { variants } => {
                format!(
                    "Enum{{{}}}",
                    variants
                        .iter()
                        .map(|variant| {
                            if variant.1.is_empty() {
                                variant.0.to_string()
                            } else {
                                format!(
                                    "{}({})",
                                    variant.0,
                                    variant
                                        .1
                                        .iter()
                                        .map(|field| field.describe())
                                        .collect::<Vec<_>>()
                                        .join(",")
                                )
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("/")
                )
            },
            ToolboxIdlProgramTypedef::Primitive(primitive) => {
                primitive.as_str().to_string()
            },
            ToolboxIdlProgramTypedef::Const { value } => {
                format!("const({})", value)
            },
            ToolboxIdlProgramTypedef::Generic { symbol } => {
                format!("generic({})", symbol)
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

    pub fn as_primitive(&self) -> Option<&ToolboxIdlProgramTypedefPrimitive> {
        match self {
            ToolboxIdlProgramTypedef::Primitive(primitive) => Some(primitive),
            _ => None,
        }
    }
}
