use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

impl ToolboxIdlTypeFull {
    pub fn explain(&self) -> Value {
        match self {
            ToolboxIdlTypeFull::Option { content, .. } => {
                json!({ "option": content.explain() })
            },
            ToolboxIdlTypeFull::Vec { items } => {
                json!([items.explain()])
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                json!([items.explain(), length])
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                json!({ "fields": fields.explain() })
            },
            ToolboxIdlTypeFull::Enum { variants } => {
                let mut json_variants = vec![];
                for (variant_name, variant_fields) in variants {
                    if variant_fields == &ToolboxIdlTypeFullFields::None {
                        json_variants.push(json!(variant_name));
                    } else {
                        json_variants.push(json!({
                            "name": variant_name,
                            "fields": variant_fields.explain()
                        }));
                    }
                }
                json!({ "variants": json_variants })
            },
            ToolboxIdlTypeFull::Padded { content, .. } => content.explain(),
            ToolboxIdlTypeFull::Const { literal } => {
                json!(literal)
            },
            ToolboxIdlTypeFull::Primitive { primitive } => {
                json!(primitive.as_str())
            },
        }
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn explain(&self) -> Value {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut json_fields = vec![];
                for (field_name, field_type) in fields {
                    json_fields.push(json!({
                        "name": field_name,
                        "type": field_type.explain(),
                    }));
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFullFields::Unamed(fields) => {
                let mut json_fields = vec![];
                for field_type in fields {
                    json_fields.push(field_type.explain());
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFullFields::None => {
                json!([])
            },
        }
    }
}
