use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatEnumVariant;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFieldNamed;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFieldUnnamed;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullEnumVariant;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldNamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldUnnamed;
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
                let typedef = idl_map_get_key_or_else(typedefs, name)
                    .context("Hydrate Defined: Lookup")?;
                if generics_flat.len() < typedef.generics.len() {
                    return Err(anyhow!(
                        "Insufficient number of generic parameter for {}: expected: {}, found: {}",
                        typedef.name,
                        typedef.generics.len(),
                        generics_flat.len()
                    ));
                }
                let mut generics_full = vec![];
                for (index, generic_flat) in generics_flat.iter().enumerate() {
                    generics_full.push(
                        generic_flat
                            .try_hydrate(generics_by_symbol, typedefs)
                            .with_context(|| {
                                format!(
                                    "Hydrate Defined: {}, Generic[{}]",
                                    name, index
                                )
                            })?,
                    );
                }
                let mut generics_by_symbol = HashMap::new();
                for (generic_name, generic_full) in
                    typedef.generics.iter().zip(generics_full)
                {
                    generics_by_symbol
                        .insert(generic_name.to_string(), generic_full);
                }
                let type_full = typedef
                    .type_flat
                    .try_hydrate(&generics_by_symbol, typedefs)
                    .with_context(|| {
                        format!("Hydrate Defined: {}, Content", name)
                    })?;
                if let Some(serialization) = &typedef.serialization {
                    if serialization == "bytemuck" {
                        let (pod_alignment, pod_size, pod_content) = type_full
                            .bytemuck_typedef(&typedef.name, &typedef.repr)
                            .with_context(|| {
                                format!("Hydrate Defined: {}, Bytemuck", name)
                            })?;
                        return Ok(ToolboxIdlTypeFull::Pod {
                            alignment: pod_alignment,
                            size: pod_size,
                            content: Box::new(pod_content),
                        });
                    }
                }
                ToolboxIdlTypeFull::Typedef {
                    name: typedef.name.clone(),
                    repr: typedef.repr.clone(),
                    content: Box::new(type_full),
                }
            },
            ToolboxIdlTypeFlat::Generic { symbol } => {
                idl_map_get_key_or_else(generics_by_symbol, symbol)
                    .with_context(|| {
                        format!("Hydrate Generic Lookup: {}", symbol)
                    })?
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
                length: usize::try_from(
                    *length_flat
                        .try_hydrate(generics_by_symbol, typedefs)?
                        .as_const_literal()
                        .context("Expected a const literal")?,
                )?,
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
                for variant_flat in variants_flat {
                    variants_full.push(
                        variant_flat
                            .try_hydrate(generics_by_symbol, typedefs)?,
                    );
                }
                ToolboxIdlTypeFull::Enum {
                    prefix: prefix.clone(),
                    variants: variants_full,
                }
            },
            ToolboxIdlTypeFlat::Padded {
                before,
                min_size,
                after,
                content: content_flat,
            } => ToolboxIdlTypeFull::Padded {
                before: *before,
                min_size: *min_size,
                after: *after,
                content: Box::new(
                    content_flat.try_hydrate(generics_by_symbol, typedefs)?,
                ),
            },
            ToolboxIdlTypeFlat::Const { literal } => {
                ToolboxIdlTypeFull::Const { literal: *literal }
            },
            ToolboxIdlTypeFlat::Primitive { primitive } => {
                primitive.clone().into()
            },
        })
    }
}

impl ToolboxIdlTypeFlatEnumVariant {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFull>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFullEnumVariant> {
        let variant_full_fields = self
            .fields
            .try_hydrate(generics_by_symbol, typedefs)
            .with_context(|| {
                format!("Variant: {}({})", self.name, self.code)
            })?;
        Ok(ToolboxIdlTypeFullEnumVariant {
            name: self.name.to_string(),
            code: self.code,
            fields: variant_full_fields,
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
                    fields_full.push(
                        field
                            .try_hydrate(generics_by_symbol, typedefs)
                            .with_context(|| {
                                format!("Hydrate Named Field: {}", field.name)
                            })?,
                    );
                }
                ToolboxIdlTypeFullFields::Named(fields_full)
            },
            ToolboxIdlTypeFlatFields::Unnamed(fields) => {
                let mut fields_type_full = vec![];
                for (index, field) in fields.iter().enumerate() {
                    fields_type_full.push(
                        field
                            .try_hydrate(generics_by_symbol, typedefs)
                            .with_context(|| {
                                format!("Hydrate Unnamed Field: {}", index)
                            })?,
                    );
                }
                ToolboxIdlTypeFullFields::Unnamed(fields_type_full)
            },
            ToolboxIdlTypeFlatFields::None => ToolboxIdlTypeFullFields::None,
        })
    }
}

impl ToolboxIdlTypeFlatFieldNamed {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFull>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFullFieldNamed> {
        Ok(ToolboxIdlTypeFullFieldNamed {
            name: self.name.to_string(),
            content: self.content.try_hydrate(generics_by_symbol, typedefs)?,
        })
    }
}

impl ToolboxIdlTypeFlatFieldUnnamed {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFull>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFullFieldUnnamed> {
        Ok(ToolboxIdlTypeFullFieldUnnamed {
            content: self.content.try_hydrate(generics_by_symbol, typedefs)?,
        })
    }
}
