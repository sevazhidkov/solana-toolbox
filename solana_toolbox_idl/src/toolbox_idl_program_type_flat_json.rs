use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_program_type_flat::ToolboxIdlProgramTypeFlat;
use crate::toolbox_idl_program_type_flat::ToolboxIdlProgramTypeFlatFields;

impl ToolboxIdlProgramTypeFlat {
    pub fn as_json(&self, backward_compatibility: bool) -> Value {
        match self {
            ToolboxIdlProgramTypeFlat::Defined { name, generics } => {
                if !generics.is_empty() {
                    let mut json_generics = vec![];
                    for generic in generics {
                        if backward_compatibility {
                            json_generics.push(json!({
                                "kind": "type",
                                "type": generic.as_json(backward_compatibility),
                            }));
                        } else {
                            json_generics
                                .push(generic.as_json(backward_compatibility));
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
            ToolboxIdlProgramTypeFlat::Option { content } => {
                json!({ "option": content.as_json(backward_compatibility) })
            },
            ToolboxIdlProgramTypeFlat::Vec { items } => {
                if backward_compatibility {
                    json!({ "vec": items.as_json(backward_compatibility) })
                } else {
                    json!([items.as_json(backward_compatibility)])
                }
            },
            ToolboxIdlProgramTypeFlat::Array { items, length } => {
                if backward_compatibility {
                    json!({ "array": [
                        items.as_json(backward_compatibility),
                        length.as_json(backward_compatibility)
                    ]})
                } else {
                    json!([
                        items.as_json(backward_compatibility),
                        length.as_json(backward_compatibility)
                    ])
                }
            },
            ToolboxIdlProgramTypeFlat::Struct { fields } => {
                if backward_compatibility {
                    json!({
                        "kind": "struct",
                        "fields": fields.as_json(backward_compatibility)
                    })
                } else {
                    json!({ "fields": fields.as_json(backward_compatibility) })
                }
            },
            ToolboxIdlProgramTypeFlat::Enum { variants } => {
                let mut json_variants = vec![];
                for (variant_name, variant_fields) in variants {
                    if variant_fields == &ToolboxIdlProgramTypeFlatFields::None
                    {
                        if backward_compatibility {
                            json_variants.push(json!({ "name": variant_name }));
                        } else {
                            json_variants.push(json!(variant_name));
                        }
                    } else {
                        json_variants.push(json!({
                            "name": variant_name,
                            "fields": variant_fields.as_json(backward_compatibility)
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
            ToolboxIdlProgramTypeFlat::Generic { symbol } => {
                json!({ "generic": symbol })
            },
            ToolboxIdlProgramTypeFlat::Const { literal } => {
                if backward_compatibility {
                    json!({
                        "kind": "const",
                        "value": literal.to_string(),
                    })
                } else {
                    json!(literal)
                }
            },
            ToolboxIdlProgramTypeFlat::Primitive { primitive } => {
                json!(primitive.as_str()) // TODO - in anchor 0.26, some names are different
            },
        }
    }
}

impl ToolboxIdlProgramTypeFlatFields {
    pub fn as_json(&self, backward_compatibility: bool) -> Value {
        match self {
            ToolboxIdlProgramTypeFlatFields::Named(fields) => {
                let mut json_fields = vec![];
                for (field_name, field_type) in fields {
                    json_fields.push(json!({
                        "name": field_name,
                        "type": field_type.as_json(backward_compatibility),
                    }));
                }
                json!(json_fields)
            },
            ToolboxIdlProgramTypeFlatFields::Unamed(fields) => {
                let mut json_fields = vec![];
                for field_type in fields {
                    if backward_compatibility {
                        json_fields.push(json!({
                            "type": field_type.as_json(backward_compatibility)
                        }));
                    } else {
                        json_fields
                            .push(field_type.as_json(backward_compatibility));
                    }
                }
                json!(json_fields)
            },
            ToolboxIdlProgramTypeFlatFields::None => {
                json!([])
            },
        }
    }
}
