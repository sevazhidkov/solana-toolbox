use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

impl ToolboxIdlTypeFull {
    pub fn flattened(&self) -> ToolboxIdlTypeFlat {
        match self {
            ToolboxIdlTypeFull::Option {
                prefix_bytes,
                content: content_full,
            } => ToolboxIdlTypeFlat::Option {
                prefix_bytes: *prefix_bytes,
                content: content_full.flattened().into(),
            },
            ToolboxIdlTypeFull::Vec { items: items_full } => {
                ToolboxIdlTypeFlat::Vec {
                    items: items_full.flattened().into(),
                }
            },
            ToolboxIdlTypeFull::Array {
                items: items_full,
                length,
            } => ToolboxIdlTypeFlat::Array {
                items: items_full.flattened().into(),
                length: ToolboxIdlTypeFlat::Const { literal: *length }.into(),
            },
            ToolboxIdlTypeFull::Struct {
                fields: fields_full,
            } => ToolboxIdlTypeFlat::Struct {
                fields: fields_full.flattened(),
            },
            ToolboxIdlTypeFull::Enum {
                variants: variants_full,
            } => {
                let mut variants_flat = vec![];
                for (variant_name, variant_fields) in variants_full {
                    variants_flat.push((
                        variant_name.to_string(),
                        None,
                        variant_fields.flattened(),
                    ));
                }
                ToolboxIdlTypeFlat::Enum {
                    variants: variants_flat,
                }
            },
            ToolboxIdlTypeFull::Padded {
                size_bytes,
                content: content_full,
            } => ToolboxIdlTypeFlat::Padded {
                size_bytes: *size_bytes,
                content: content_full.flattened().into(),
            },
            ToolboxIdlTypeFull::Const { literal } => {
                ToolboxIdlTypeFlat::Const { literal: *literal }
            },
            ToolboxIdlTypeFull::Primitive { primitive } => {
                ToolboxIdlTypeFlat::Primitive {
                    primitive: primitive.clone(),
                }
            },
        }
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn flattened(&self) -> ToolboxIdlTypeFlatFields {
        match self {
            ToolboxIdlTypeFullFields::Named(fields_full) => {
                let mut fields_flat = vec![];
                for (field_name, field_type_full) in fields_full {
                    fields_flat.push((
                        field_name.to_string(),
                        None,
                        field_type_full.flattened(),
                    ));
                }
                ToolboxIdlTypeFlatFields::Named(fields_flat)
            },
            ToolboxIdlTypeFullFields::Unnamed(fields_full) => {
                let mut fields_flat = vec![];
                for field_type_full in fields_full {
                    fields_flat.push((None, field_type_full.flattened()));
                }
                ToolboxIdlTypeFlatFields::Unnamed(fields_flat)
            },
            ToolboxIdlTypeFullFields::None => ToolboxIdlTypeFlatFields::None,
        }
    }
}
