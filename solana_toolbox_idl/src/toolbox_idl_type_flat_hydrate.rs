use std::collections::HashMap;
use std::sync::Arc;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdlTypeFlat {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFull>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFull, ToolboxIdlError> {
        Ok(match self {
            ToolboxIdlTypeFlat::Defined {
                name,
                generics: generics_flat,
            } => {
                let mut generics_full = vec![];
                for generic_flat in generics_flat {
                    generics_full.push(generic_flat.try_hydrate(
                        generics_by_symbol,
                        typedefs,
                        breadcrumbs,
                    )?);
                }
                let typedef = idl_map_get_key_or_else(
                    typedefs,
                    name,
                    &breadcrumbs.idl(),
                )?;
                if generics_full.len() != typedef.generics.len() {
                    return idl_err(
                        "Wrong number of generic parameter",
                        &breadcrumbs.val(),
                    );
                }
                let mut generics_by_symbol = HashMap::new();
                for (generic_name, generic_full) in
                    typedef.generics.iter().zip(generics_full)
                {
                    generics_by_symbol
                        .insert(generic_name.to_string(), generic_full);
                }
                typedef.type_flat.try_hydrate(
                    &generics_by_symbol,
                    typedefs,
                    breadcrumbs,
                )?
            },
            ToolboxIdlTypeFlat::Option {
                content: content_flat,
            } => ToolboxIdlTypeFull::Option {
                content: Box::new(content_flat.try_hydrate(
                    generics_by_symbol,
                    typedefs,
                    breadcrumbs,
                )?),
            },
            ToolboxIdlTypeFlat::Vec { items: items_flat } => {
                ToolboxIdlTypeFull::Vec {
                    items: Box::new(items_flat.try_hydrate(
                        generics_by_symbol,
                        typedefs,
                        breadcrumbs,
                    )?),
                }
            },
            ToolboxIdlTypeFlat::Array {
                items: items_flat,
                length: length_flat,
            } => ToolboxIdlTypeFull::Array {
                items: Box::new(items_flat.try_hydrate(
                    generics_by_symbol,
                    typedefs,
                    breadcrumbs,
                )?),
                length: *idl_ok_or_else(
                    length_flat
                        .try_hydrate(generics_by_symbol, typedefs, breadcrumbs)?
                        .as_const_literal(),
                    "expected a const literal",
                    &breadcrumbs.idl(),
                )?,
            },
            ToolboxIdlTypeFlat::Struct {
                fields: fields_flat,
            } => ToolboxIdlTypeFull::Struct {
                fields: fields_flat.try_hydrate(
                    generics_by_symbol,
                    typedefs,
                    breadcrumbs,
                )?,
            },
            ToolboxIdlTypeFlat::Enum {
                variants: variants_flat,
            } => {
                let mut variants_full = vec![];
                for (variant_name, variant_flat_fields) in variants_flat {
                    variants_full.push((
                        variant_name.to_string(),
                        variant_flat_fields.try_hydrate(
                            generics_by_symbol,
                            typedefs,
                            breadcrumbs,
                        )?,
                    ));
                }
                ToolboxIdlTypeFull::Enum {
                    variants: variants_full,
                }
            },
            ToolboxIdlTypeFlat::Generic { symbol } => idl_map_get_key_or_else(
                generics_by_symbol,
                symbol,
                &breadcrumbs.idl(),
            )?
            .clone(),
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
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFullFields, ToolboxIdlError> {
        Ok(match self {
            ToolboxIdlTypeFlatFields::Named(fields) => {
                let mut fields_full = vec![];
                for (field_name, field_type_flat) in fields {
                    fields_full.push((
                        field_name.to_string(),
                        field_type_flat.try_hydrate(
                            generics_by_symbol,
                            typedefs,
                            breadcrumbs,
                        )?,
                    ));
                }
                ToolboxIdlTypeFullFields::Named(fields_full)
            },
            ToolboxIdlTypeFlatFields::Unamed(fields) => {
                let mut fields_type_full = vec![];
                for field_type_flat in fields {
                    fields_type_full.push(field_type_flat.try_hydrate(
                        generics_by_symbol,
                        typedefs,
                        breadcrumbs,
                    )?);
                }
                ToolboxIdlTypeFullFields::Unamed(fields_type_full)
            },
            ToolboxIdlTypeFlatFields::None => ToolboxIdlTypeFullFields::None,
        })
    }
}
