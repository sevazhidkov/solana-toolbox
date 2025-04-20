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
            ToolboxIdlTypeFull::Vec { items, .. } => {
                json!([items.explain()])
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                json!([items.explain(), length])
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                json!({ "fields": fields.explain() })
            },
            ToolboxIdlTypeFull::Enum { variants, .. } => {
                let mut json_variants = vec![];
                for variant in variants {
                    if variant.fields == ToolboxIdlTypeFullFields::None {
                        json_variants.push(json!(variant.name));
                    } else {
                        json_variants.push(json!({
                            "name": variant.name,
                            "code": variant.code,
                            "fields": variant.fields.explain()
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
                for field in fields {
                    json_fields.push(json!({
                        "name": field.name,
                        "type": field.type_full.explain(),
                    }));
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let mut json_fields = vec![];
                for field in fields {
                    json_fields.push(field.type_full.explain());
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFullFields::None => {
                json!([])
            },
        }
    }
}
