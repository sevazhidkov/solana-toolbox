use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

// TODO (SHORT) - add tests for this
impl ToolboxIdlTypeFull {
    pub fn summarize(&self) -> String {
        match self {
            ToolboxIdlTypeFull::Option { content, .. } => {
                format!("(null|{})", content.summarize())
            },
            ToolboxIdlTypeFull::Vec { items } => {
                format!("[{}..]", items.summarize())
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                format!("[{}x{}]", length, items.summarize())
            },
            ToolboxIdlTypeFull::Struct { fields } => fields.summarize(),
            ToolboxIdlTypeFull::Enum { variants } => {
                let mut cases = vec![];
                for (variant_name, variant_fields) in variants {
                    if variant_fields == &ToolboxIdlTypeFullFields::None {
                        cases.push(variant_name.to_string());
                    } else {
                        cases.push(format!(
                            "[\"{}\",{}]",
                            variant_name,
                            variant_fields.summarize()
                        ));
                    }
                }
                cases.join("|")
            },
            ToolboxIdlTypeFull::Padded { content, .. } => content.summarize(),
            ToolboxIdlTypeFull::Const { .. } => "?".to_string(),
            ToolboxIdlTypeFull::Primitive { primitive } => {
                primitive.as_str().to_string()
            },
        }
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn summarize(&self) -> String {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut items = vec![];
                for (field_name, field_type) in fields {
                    items.push(format!(
                        "{}:{}",
                        field_name,
                        field_type.summarize()
                    ));
                }
                format!("{{{}}}", items.join(","))
            },
            ToolboxIdlTypeFullFields::Unamed(fields) => {
                let mut items = vec![];
                for field_type in fields {
                    items.push(field_type.summarize());
                }
                format!("[{}]", items.join(","))
            },
            ToolboxIdlTypeFullFields::None => "null".to_string(),
        }
    }
}
