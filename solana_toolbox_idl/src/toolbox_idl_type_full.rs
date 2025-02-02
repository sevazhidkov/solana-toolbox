use crate::toolbox_idl_primitive::ToolboxIdlPrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFull {
    Option { content: Box<ToolboxIdlTypeFull> },
    Vec { items: Box<ToolboxIdlTypeFull> },
    Array { items: Box<ToolboxIdlTypeFull>, length: usize },
    Struct { fields: Vec<(String, ToolboxIdlTypeFull)> },
    Enum { variants: Vec<(String, Vec<(String, ToolboxIdlTypeFull)>)> },
    Const { literal: usize },
    Primitive { primitive: ToolboxIdlPrimitive },
}

impl ToolboxIdlTypeFull {
    pub fn describe(&self) -> String {
        match self {
            ToolboxIdlTypeFull::Option { content } => {
                format!("Option<{}>", content.describe())
            },
            ToolboxIdlTypeFull::Vec { items } => {
                format!("Vec<{}>", items.describe())
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                format!("[{};{}]", items.describe(), length)
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                format!(
                    "Struct({})",
                    ToolboxIdlTypeFull::describe_fields(fields)
                )
            },
            ToolboxIdlTypeFull::Enum { variants } => {
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
                                    ToolboxIdlTypeFull::describe_fields(
                                        &variant.1
                                    )
                                )
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("/")
                )
            },
            ToolboxIdlTypeFull::Primitive { primitive } => {
                primitive.as_str().to_string()
            },
            ToolboxIdlTypeFull::Const { literal } => {
                format!("{}", literal)
            },
        }
    }

    fn describe_fields(fields: &Vec<(String, ToolboxIdlTypeFull)>) -> String {
        fields
            .iter()
            .map(|field| format!("{}:{}", field.0, field.1.describe()))
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn as_struct_fields(
        &self
    ) -> Option<&Vec<(String, ToolboxIdlTypeFull)>> {
        match self {
            ToolboxIdlTypeFull::Struct { fields } => Some(fields),
            _ => None,
        }
    }

    pub fn as_const_literal(&self) -> Option<&usize> {
        match self {
            ToolboxIdlTypeFull::Const { literal } => Some(literal),
            _ => None,
        }
    }

    pub fn as_primitive(&self) -> Option<&ToolboxIdlPrimitive> {
        match self {
            ToolboxIdlTypeFull::Primitive { primitive } => Some(primitive),
            _ => None,
        }
    }
}
