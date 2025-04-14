use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

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
                prefix_bytes,
                content: content_flat,
            } => ToolboxIdlTypeFull::Option {
                prefix_bytes: *prefix_bytes,
                content: Box::new(
                    content_flat.try_hydrate(generics_by_symbol, typedefs)?,
                ),
            },
            ToolboxIdlTypeFlat::Vec {
                prefix_bytes,
                items: items_flat,
            } => ToolboxIdlTypeFull::Vec {
                prefix_bytes: *prefix_bytes,
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
                prefix_bytes,
                variants: variants_flat,
            } => {
                let mut variants_full = vec![];
                for (variant_name, _variant_docs, variant_flat_fields) in
                    variants_flat
                {
                    variants_full.push((
                        variant_name.to_string(),
                        variant_flat_fields
                            .try_hydrate(generics_by_symbol, typedefs)
                            .with_context(|| {
                                format!("Variant: {}", variant_name)
                            })?,
                    ));
                }
                ToolboxIdlTypeFull::Enum {
                    prefix_bytes: *prefix_bytes,
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
                for (field_name, _field_docs, field_type_flat) in fields {
                    fields_full.push((
                        field_name.to_string(),
                        field_type_flat
                            .try_hydrate(generics_by_symbol, typedefs)
                            .with_context(|| {
                                format!("Field: {}", field_name)
                            })?,
                    ));
                }
                ToolboxIdlTypeFullFields::Named(fields_full)
            },
            ToolboxIdlTypeFlatFields::Unnamed(fields) => {
                let mut fields_type_full = vec![];
                for (index, (_field_docs, field_type_flat)) in
                    fields.iter().enumerate()
                {
                    fields_type_full.push(
                        field_type_flat
                            .try_hydrate(generics_by_symbol, typedefs)
                            .with_context(|| format!("Field: {}", index))?,
                    );
                }
                ToolboxIdlTypeFullFields::Unnamed(fields_type_full)
            },
            ToolboxIdlTypeFlatFields::None => ToolboxIdlTypeFullFields::None,
        })
    }
}
