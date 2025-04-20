use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatEnumVariant;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFieldNamed;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFieldUnamed;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

impl ToolboxIdlTypeFull {
    pub fn flattened(&self) -> ToolboxIdlTypeFlat {
        match self {
            ToolboxIdlTypeFull::Option {
                prefix,
                content: content_full,
            } => ToolboxIdlTypeFlat::Option {
                prefix: prefix.clone(),
                content: content_full.flattened().into(),
            },
            ToolboxIdlTypeFull::Vec {
                prefix,
                items: items_full,
            } => ToolboxIdlTypeFlat::Vec {
                prefix: prefix.clone(),
                items: items_full.flattened().into(),
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
                prefix,
                variants: variants_full,
            } => {
                let mut variants_flat = vec![];
                for variant_full in variants_full {
                    variants_flat.push(ToolboxIdlTypeFlatEnumVariant {
                        name: variant_full.name.to_string(),
                        code: variant_full.code,
                        docs: None,
                        fields: variant_full.fields.flattened(),
                    });
                }
                ToolboxIdlTypeFlat::Enum {
                    prefix: prefix.clone(),
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
                for field in fields_full {
                    fields_flat.push(ToolboxIdlTypeFlatFieldNamed {
                        name: field.name.to_string(),
                        docs: None,
                        type_flat: field.type_full.flattened(),
                    });
                }
                ToolboxIdlTypeFlatFields::Named(fields_flat)
            },
            ToolboxIdlTypeFullFields::Unnamed(fields_full) => {
                let mut fields_flat = vec![];
                for field in fields_full {
                    fields_flat.push(ToolboxIdlTypeFlatFieldUnamed {
                        docs: None,
                        type_flat: field.type_full.flattened(),
                    });
                }
                ToolboxIdlTypeFlatFields::Unnamed(fields_flat)
            },
            ToolboxIdlTypeFullFields::None => ToolboxIdlTypeFlatFields::None,
        }
    }
}
