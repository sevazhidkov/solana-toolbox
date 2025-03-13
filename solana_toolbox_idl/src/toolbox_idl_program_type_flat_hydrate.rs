use std::collections::HashMap;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_type_flat::ToolboxIdlProgramTypeFlat;
use crate::toolbox_idl_program_type_flat::ToolboxIdlProgramTypeFlatFields;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFull;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFullFields;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdlProgramTypeFlat {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlProgramTypeFull>,
        program_typedefs: &HashMap<String, ToolboxIdlProgramTypedef>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypeFull, ToolboxIdlError> {
        Ok(match self {
            ToolboxIdlProgramTypeFlat::Defined {
                name,
                generics: generics_flat,
            } => {
                let mut generics_full = vec![];
                for generic_flat in generics_flat {
                    generics_full.push(generic_flat.try_hydrate(
                        generics_by_symbol,
                        program_typedefs,
                        breadcrumbs,
                    )?);
                }
                let program_typedef = idl_map_get_key_or_else(
                    program_typedefs,
                    name,
                    &breadcrumbs.idl(),
                )?;
                if generics_full.len() != program_typedef.generics.len() {
                    return idl_err(
                        "Wrong number of generic parameter",
                        &breadcrumbs.val(),
                    );
                }
                let mut generics_by_symbol = HashMap::new();
                for (generic_name, generic_full) in
                    program_typedef.generics.iter().zip(generics_full)
                {
                    generics_by_symbol
                        .insert(generic_name.to_string(), generic_full);
                }
                program_typedef.type_flat.try_hydrate(
                    &generics_by_symbol,
                    program_typedefs,
                    breadcrumbs,
                )?
            },
            ToolboxIdlProgramTypeFlat::Option {
                content: content_flat,
            } => ToolboxIdlProgramTypeFull::Option {
                content: Box::new(content_flat.try_hydrate(
                    generics_by_symbol,
                    program_typedefs,
                    breadcrumbs,
                )?),
            },
            ToolboxIdlProgramTypeFlat::Vec { items: items_flat } => {
                ToolboxIdlProgramTypeFull::Vec {
                    items: Box::new(items_flat.try_hydrate(
                        generics_by_symbol,
                        program_typedefs,
                        breadcrumbs,
                    )?),
                }
            },
            ToolboxIdlProgramTypeFlat::Array {
                items: items_flat,
                length: length_flat,
            } => ToolboxIdlProgramTypeFull::Array {
                items: Box::new(items_flat.try_hydrate(
                    generics_by_symbol,
                    program_typedefs,
                    breadcrumbs,
                )?),
                length: *idl_ok_or_else(
                    length_flat
                        .try_hydrate(
                            generics_by_symbol,
                            program_typedefs,
                            breadcrumbs,
                        )?
                        .as_const_literal(),
                    "expected a const literal",
                    &breadcrumbs.idl(),
                )?,
            },
            ToolboxIdlProgramTypeFlat::Struct {
                fields: fields_flat,
            } => ToolboxIdlProgramTypeFull::Struct {
                fields: fields_flat.try_hydrate(
                    generics_by_symbol,
                    program_typedefs,
                    breadcrumbs,
                )?,
            },
            ToolboxIdlProgramTypeFlat::Enum {
                variants: variants_flat,
            } => {
                let mut variants_full = vec![];
                for (variant_name, variant_flat_fields) in variants_flat {
                    variants_full.push((
                        variant_name.to_string(),
                        variant_flat_fields.try_hydrate(
                            generics_by_symbol,
                            program_typedefs,
                            breadcrumbs,
                        )?,
                    ));
                }
                ToolboxIdlProgramTypeFull::Enum {
                    variants: variants_full,
                }
            },
            ToolboxIdlProgramTypeFlat::Generic { symbol } => {
                idl_map_get_key_or_else(
                    generics_by_symbol,
                    symbol,
                    &breadcrumbs.idl(),
                )?
                .clone()
            },
            ToolboxIdlProgramTypeFlat::Const { literal } => {
                ToolboxIdlProgramTypeFull::Const { literal: *literal }
            },
            ToolboxIdlProgramTypeFlat::Primitive { primitive } => {
                ToolboxIdlProgramTypeFull::Primitive {
                    primitive: primitive.clone(),
                }
            },
        })
    }
}

impl ToolboxIdlProgramTypeFlatFields {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlProgramTypeFull>,
        program_typedefs: &HashMap<String, ToolboxIdlProgramTypedef>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypeFullFields, ToolboxIdlError> {
        Ok(match self {
            ToolboxIdlProgramTypeFlatFields::Named(fields) => {
                let mut fields_full = vec![];
                for (field_name, field_type_flat) in fields {
                    fields_full.push((
                        field_name.to_string(),
                        field_type_flat.try_hydrate(
                            generics_by_symbol,
                            program_typedefs,
                            breadcrumbs,
                        )?,
                    ));
                }
                ToolboxIdlProgramTypeFullFields::Named(fields_full)
            },
            ToolboxIdlProgramTypeFlatFields::Unamed(fields) => {
                let mut fields_type_full = vec![];
                for field_type_flat in fields {
                    fields_type_full.push(field_type_flat.try_hydrate(
                        generics_by_symbol,
                        program_typedefs,
                        breadcrumbs,
                    )?);
                }
                ToolboxIdlProgramTypeFullFields::Unamed(fields_type_full)
            },
            ToolboxIdlProgramTypeFlatFields::None => {
                ToolboxIdlProgramTypeFullFields::None
            },
        })
    }
}
