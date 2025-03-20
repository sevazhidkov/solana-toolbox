use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;

impl ToolboxIdlTypeFlat {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        match self {
            ToolboxIdlTypeFlat::Defined { name, generics } => {
                if !generics.is_empty() {
                    let mut json_generics = vec![];
                    for generic in generics {
                        if backward_compatibility {
                            json_generics.push(json!({
                                "kind": "type",
                                "type": generic.export(backward_compatibility),
                            }));
                        } else {
                            json_generics
                                .push(generic.export(backward_compatibility));
                        }
                    }
                    json!({ "defined": { "name": name, "generics": json_generics }})
                } else if backward_compatibility {
                    // TODO - in anchor 0.26, the format is {defined:name}
                    json!({ "defined": { "name": name }})
                } else {
                    json!(name)
                }
            },
            ToolboxIdlTypeFlat::Generic { symbol } => {
                json!({ "generic": symbol })
            },
            ToolboxIdlTypeFlat::Option {
                prefix_bytes,
                content,
            } => {
                if *prefix_bytes == 4 {
                    json!({ "option32": content.export(backward_compatibility) })
                } else {
                    json!({ "option": content.export(backward_compatibility) })
                }
            },
            ToolboxIdlTypeFlat::Vec { items } => {
                if backward_compatibility {
                    json!({ "vec": items.export(backward_compatibility) })
                } else {
                    json!([items.export(backward_compatibility)])
                }
            },
            ToolboxIdlTypeFlat::Array { items, length } => {
                if backward_compatibility {
                    json!({ "array": [
                        items.export(backward_compatibility),
                        length.export(backward_compatibility)
                    ]})
                } else {
                    json!([
                        items.export(backward_compatibility),
                        length.export(backward_compatibility)
                    ])
                }
            },
            ToolboxIdlTypeFlat::Struct { fields } => {
                if backward_compatibility {
                    json!({
                        "kind": "struct",
                        "fields": fields.export(backward_compatibility)
                    })
                } else {
                    json!({ "fields": fields.export(backward_compatibility) })
                }
            },
            ToolboxIdlTypeFlat::Enum { variants } => {
                let mut json_variants = vec![];
                for (variant_name, variant_fields) in variants {
                    if variant_fields == &ToolboxIdlTypeFlatFields::None {
                        if backward_compatibility {
                            json_variants.push(json!({ "name": variant_name }));
                        } else {
                            json_variants.push(json!(variant_name));
                        }
                    } else {
                        json_variants.push(json!({
                            "name": variant_name,
                            "fields": variant_fields.export(backward_compatibility)
                        }));
                    }
                }
                if backward_compatibility {
                    json!({
                        "kind": "enum",
                        "variants": json_variants
                    })
                } else {
                    json!({ "variants": json_variants })
                }
            },
            ToolboxIdlTypeFlat::Padded {
                size_bytes,
                content,
            } => {
                json!({
                    "padded": {
                        "size": size_bytes,
                        "type": content.export(backward_compatibility)
                    }
                })
            },
            ToolboxIdlTypeFlat::Const { literal } => {
                if backward_compatibility {
                    json!({
                        "kind": "const",
                        "value": literal.to_string(),
                    })
                } else {
                    json!(literal)
                }
            },
            ToolboxIdlTypeFlat::Primitive { primitive } => {
                json!(primitive.as_str()) // TODO - in anchor 0.26, some names are different
            },
        }
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        match self {
            ToolboxIdlTypeFlatFields::Named(fields) => {
                let mut json_fields = vec![];
                for (field_name, field_type) in fields {
                    json_fields.push(json!({
                        "name": field_name,
                        "type": field_type.export(backward_compatibility),
                    }));
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFlatFields::Unamed(fields) => {
                let mut json_fields = vec![];
                for field_type in fields {
                    if backward_compatibility {
                        json_fields.push(json!({
                            "type": field_type.export(backward_compatibility)
                        }));
                    } else {
                        json_fields
                            .push(field_type.export(backward_compatibility));
                    }
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFlatFields::None => {
                json!([])
            },
        }
    }
}
