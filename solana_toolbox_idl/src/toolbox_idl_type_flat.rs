use crate::toolbox_idl_primitive::ToolboxIdlPrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFlat {
    Defined {
        name: String,
        generics: Vec<ToolboxIdlTypeFlat>,
    },
    Generic {
        symbol: String,
    },
    Option {
        content: Box<ToolboxIdlTypeFlat>,
    },
    Vec {
        items: Box<ToolboxIdlTypeFlat>,
    },
    Array {
        items: Box<ToolboxIdlTypeFlat>,
        length: Box<ToolboxIdlTypeFlat>,
    },
    Struct {
        fields: Vec<(String, ToolboxIdlTypeFlat)>,
    },
    Enum {
        variants: Vec<(String, Vec<(String, ToolboxIdlTypeFlat)>)>,
    },
    Const {
        literal: usize, // TODO - what other kind of consts can be supported ?
    },
    Primitive {
        primitive: ToolboxIdlPrimitive,
    },
}

impl ToolboxIdlTypeFlat {
    pub fn describe(&self) -> String {
        match self {
            ToolboxIdlTypeFlat::Defined { name, generics } => {
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
            ToolboxIdlTypeFlat::Generic { symbol } => {
                format!("#{}", symbol)
            },
            ToolboxIdlTypeFlat::Option { content } => {
                format!("Option<{}>", content.describe())
            },
            ToolboxIdlTypeFlat::Vec { items } => {
                format!("Vec<{}>", items.describe())
            },
            ToolboxIdlTypeFlat::Array { items, length } => {
                format!("[{};{}]", items.describe(), length.describe())
            },
            ToolboxIdlTypeFlat::Struct { fields } => {
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
            ToolboxIdlTypeFlat::Enum { variants } => {
                format!(
                    "Enum{{{}}}",
                    variants
                        .iter()
                        .map(|variant| {
                            if variant.1.is_empty() {
                                variant.0.to_string()
                            } else {
                                format!(
                                    "{}[{}]",
                                    variant.0,
                                    variant
                                        .1
                                        .iter()
                                        .map(|field| {
                                            format!(
                                                "{}:{}",
                                                field.0,
                                                field.1.describe()
                                            )
                                        })
                                        .collect::<Vec<_>>()
                                        .join(",")
                                )
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("/")
                )
            },
            ToolboxIdlTypeFlat::Const { literal } => {
                format!("{}", literal)
            },
            ToolboxIdlTypeFlat::Primitive { primitive } => {
                primitive.as_str().to_string()
            },
        }
    }
}
