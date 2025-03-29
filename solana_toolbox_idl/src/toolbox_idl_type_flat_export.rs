use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_format::ToolboxIdlFormat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;

impl ToolboxIdlTypeFlat {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        match self {
            ToolboxIdlTypeFlat::Defined { name, generics } => {
                if !generics.is_empty() {
                    let mut json_generics = vec![];
                    for generic in generics {
                        if format.can_skip_kind_key() {
                            json_generics.push(generic.export(format));
                        } else {
                            json_generics.push(json!({
                                "kind": "type",
                                "type": generic.export(format),
                            }));
                        }
                    }
                    return json!({ "defined": { "name": name, "generics": json_generics }});
                }
                if format.can_shortcut_defined_name_to_string_if_no_generic() {
                    return json!(name);
                }
                if format.can_skip_defined_name_object_wrapping() {
                    return json!({ "defined": name });
                }
                json!({ "defined": { "name": name }})
            },
            ToolboxIdlTypeFlat::Generic { symbol } => {
                json!({ "generic": symbol })
            },
            ToolboxIdlTypeFlat::Option {
                prefix_bytes,
                content,
            } => {
                if *prefix_bytes == 4 {
                    json!({ "option32": content.export(format) })
                } else {
                    json!({ "option": content.export(format) })
                }
            },
            ToolboxIdlTypeFlat::Vec { items } => {
                if format.can_shortcut_vec_and_array_notation() {
                    return json!([items.export(format)]);
                }
                json!({ "vec": items.export(format) })
            },
            ToolboxIdlTypeFlat::Array { items, length } => {
                if format.can_shortcut_vec_and_array_notation() {
                    return json!([
                        items.export(format),
                        length.export(format)
                    ]);
                }
                json!({ "array": [
                    items.export(format),
                    length.export(format)
                ]})
            },
            ToolboxIdlTypeFlat::Struct { fields } => {
                if format.can_skip_kind_key() {
                    return json!({ "fields": fields.export(format) });
                }
                json!({
                    "kind": "struct",
                    "fields": fields.export(format)
                })
            },
            ToolboxIdlTypeFlat::Enum { variants } => {
                let mut json_variants = vec![];
                for (variant_name, variant_fields) in variants {
                    if variant_fields == &ToolboxIdlTypeFlatFields::None {
                        if format
                            .can_shortcut_enum_variant_to_string_if_no_field()
                        {
                            json_variants.push(json!(variant_name));
                        } else {
                            json_variants.push(json!({ "name": variant_name }));
                        }
                    } else {
                        json_variants.push(json!({
                            "name": variant_name,
                            "fields": variant_fields.export(format)
                        }));
                    }
                }
                if format.can_skip_kind_key() {
                    return json!({ "variants": json_variants });
                }
                json!({
                    "kind": "enum",
                    "variants": json_variants
                })
            },
            ToolboxIdlTypeFlat::Padded {
                size_bytes,
                content,
            } => {
                json!({
                    "padded": {
                        "size": size_bytes,
                        "type": content.export(format)
                    }
                })
            },
            ToolboxIdlTypeFlat::Const { literal } => {
                if format.can_skip_kind_key() {
                    return json!(literal);
                }
                json!({
                    "kind": "const",
                    "value": literal.to_string(),
                })
            },
            ToolboxIdlTypeFlat::Primitive { primitive } => {
                json!(primitive.as_str()) // TODO (FAR) - in anchor 0.26, some names are different
            },
        }
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        match self {
            ToolboxIdlTypeFlatFields::Named(fields) => {
                let mut json_fields = vec![];
                for (field_name, field_type) in fields {
                    json_fields.push(json!({
                        "name": field_name,
                        "type": field_type.export(format),
                    }));
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFlatFields::Unamed(fields) => {
                let mut json_fields = vec![];
                for field_type in fields {
                    if format.can_skip_type_object_wrapping() {
                        json_fields.push(field_type.export(format));
                    } else {
                        json_fields.push(json!({
                            "type": field_type.export(format)
                        }));
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
