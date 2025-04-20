use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullEnumVariant;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldNamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldUnnamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

// TODO (FAR) - support passing missing symbols
impl ToolboxIdlTypeFlat {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFull>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFull> {
        Ok(match self {
            ToolboxIdlTypeFlat::Defined {
                name,
                generics: generics_flat,
            } => {
                let mut generics_full = vec![];
                for (index, generic_flat) in generics_flat.iter().enumerate() {
                    generics_full.push(
                        generic_flat
                            .try_hydrate(generics_by_symbol, typedefs)
                            .with_context(|| {
                                format!("Defined's Generic: {}", index)
                            })?,
                    );
                }
                let typedef = idl_map_get_key_or_else(typedefs, name)
                    .context("Defined type")?;
                if generics_full.len() < typedef.generics.len() {
                    return Err(anyhow!(
                        "Insufficient number of generic parameter: expected: {}, found: {}",
                        typedef.generics.len(),
                        generics_full.len()
                    ));
                }
                let mut generics_by_symbol = HashMap::new();
                for (generic_name, generic_full) in
                    typedef.generics.iter().zip(generics_full)
                {
                    generics_by_symbol
                        .insert(generic_name.to_string(), generic_full);
                }
                typedef
                    .type_flat
                    .try_hydrate(&generics_by_symbol, typedefs)
                    .with_context(|| format!("Defined: {}", name))?
            },
            ToolboxIdlTypeFlat::Generic { symbol } => {
                idl_map_get_key_or_else(generics_by_symbol, symbol)
                    .with_context(|| format!("Generic: {}", symbol))?
                    .clone()
            },
            ToolboxIdlTypeFlat::Option {
                prefix,
                content: content_flat,
            } => ToolboxIdlTypeFull::Option {
                prefix: prefix.clone(),
                content: Box::new(
                    content_flat.try_hydrate(generics_by_symbol, typedefs)?,
                ),
            },
            ToolboxIdlTypeFlat::Vec {
                prefix,
                items: items_flat,
            } => ToolboxIdlTypeFull::Vec {
                prefix: prefix.clone(),
                items: Box::new(
                    items_flat.try_hydrate(generics_by_symbol, typedefs)?,
                ),
            },
            ToolboxIdlTypeFlat::Array {
                items: items_flat,
                length: length_flat,
            } => ToolboxIdlTypeFull::Array {
                items: Box::new(
                    items_flat.try_hydrate(generics_by_symbol, typedefs)?,
                ),
                length: *length_flat
                    .try_hydrate(generics_by_symbol, typedefs)?
                    .as_const_literal()
                    .context("Expected a const literal")?,
            },
            ToolboxIdlTypeFlat::Struct {
                fields: fields_flat,
            } => ToolboxIdlTypeFull::Struct {
                fields: fields_flat
                    .try_hydrate(generics_by_symbol, typedefs)?,
            },
            ToolboxIdlTypeFlat::Enum {
                prefix,
                variants: variants_flat,
            } => {
                let mut variants_full = vec![];
                for variant in variants_flat {
                    variants_full.push(ToolboxIdlTypeFullEnumVariant {
                        name: variant.name.to_string(),
                        code: variant.code,
                        fields: variant
                            .fields
                            .try_hydrate(generics_by_symbol, typedefs)
                            .with_context(|| {
                                format!(
                                    "Variant: {}({})",
                                    variant.name, variant.code
                                )
                            })?,
                    });
                }
                ToolboxIdlTypeFull::Enum {
                    prefix: prefix.clone(),
                    variants: variants_full,
                }
            },
            ToolboxIdlTypeFlat::Padded {
                size_bytes,
                content: content_flat,
            } => ToolboxIdlTypeFull::Padded {
                size_bytes: *size_bytes,
                content: Box::new(
                    content_flat.try_hydrate(generics_by_symbol, typedefs)?,
                ),
            },
            ToolboxIdlTypeFlat::Const { literal } => {
                ToolboxIdlTypeFull::Const { literal: *literal }
            },
            ToolboxIdlTypeFlat::Primitive { primitive } => {
                ToolboxIdlTypeFull::Primitive {
                    primitive: primitive.clone(),
                }
            },
        })
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFull>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFullFields> {
        Ok(match self {
            ToolboxIdlTypeFlatFields::Named(fields) => {
                let mut fields_full = vec![];
                for field in fields {
                    fields_full.push(ToolboxIdlTypeFullFieldNamed {
                        name: field.name.to_string(),
                        type_full: field
                            .type_flat
                            .try_hydrate(generics_by_symbol, typedefs)
                            .with_context(|| {
                                format!("Field: {}", field.name)
                            })?,
                    });
                }
                ToolboxIdlTypeFullFields::Named(fields_full)
            },
            ToolboxIdlTypeFlatFields::Unnamed(fields) => {
                let mut fields_type_full = vec![];
                for (index, field) in fields.iter().enumerate() {
                    fields_type_full.push(ToolboxIdlTypeFullFieldUnnamed {
                        type_full: field
                            .type_flat
                            .try_hydrate(generics_by_symbol, typedefs)
                            .with_context(|| format!("Field: {}", index))?,
                    });
                }
                ToolboxIdlTypeFullFields::Unnamed(fields_type_full)
            },
            ToolboxIdlTypeFlatFields::None => ToolboxIdlTypeFullFields::None,
        })
    }
}
