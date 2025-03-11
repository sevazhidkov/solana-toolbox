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

// TODO - should this be an api on the type flat instead ?
impl ToolboxIdlProgramTypeFull {
    pub fn try_hydrate(
        program_typedefs: &HashMap<String, ToolboxIdlProgramTypedef>,
        generics_by_symbol: &HashMap<String, ToolboxIdlProgramTypeFull>,
        program_type_flat: &ToolboxIdlProgramTypeFlat,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypeFull, ToolboxIdlError> {
        Ok(match program_type_flat {
            ToolboxIdlProgramTypeFlat::Defined {
                name,
                generics: generics_flat,
            } => {
                let mut generics_full = vec![];
                for generic_flat in generics_flat {
                    generics_full.push(ToolboxIdlProgramTypeFull::try_hydrate(
                        program_typedefs,
                        generics_by_symbol,
                        generic_flat,
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
                ToolboxIdlProgramTypeFull::try_hydrate(
                    program_typedefs,
                    &generics_by_symbol,
                    &program_typedef.type_flat,
                    breadcrumbs,
                )?
            },
            ToolboxIdlProgramTypeFlat::Option {
                content: content_flat,
            } => ToolboxIdlProgramTypeFull::Option {
                content: Box::new(ToolboxIdlProgramTypeFull::try_hydrate(
                    program_typedefs,
                    generics_by_symbol,
                    content_flat,
                    breadcrumbs,
                )?),
            },
            ToolboxIdlProgramTypeFlat::Vec { items: items_flat } => {
                ToolboxIdlProgramTypeFull::Vec {
                    items: Box::new(ToolboxIdlProgramTypeFull::try_hydrate(
                        program_typedefs,
                        generics_by_symbol,
                        items_flat,
                        breadcrumbs,
                    )?),
                }
            },
            ToolboxIdlProgramTypeFlat::Array {
                items: items_flat,
                length: length_flat,
            } => ToolboxIdlProgramTypeFull::Array {
                items: Box::new(ToolboxIdlProgramTypeFull::try_hydrate(
                    program_typedefs,
                    generics_by_symbol,
                    items_flat,
                    breadcrumbs,
                )?),
                length: *idl_ok_or_else(
                    ToolboxIdlProgramTypeFull::try_hydrate(
                        program_typedefs,
                        generics_by_symbol,
                        length_flat,
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
                fields: ToolboxIdlProgramTypeFullFields::try_hydrate(
                    program_typedefs,
                    generics_by_symbol,
                    fields_flat,
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
                        ToolboxIdlProgramTypeFullFields::try_hydrate(
                            program_typedefs,
                            generics_by_symbol,
                            variant_flat_fields,
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

// TODO - some naming of variable in there needs some work probably
impl ToolboxIdlProgramTypeFullFields {
    pub fn try_hydrate(
        program_typedefs: &HashMap<String, ToolboxIdlProgramTypedef>,
        generics_by_symbol: &HashMap<String, ToolboxIdlProgramTypeFull>,
        type_flat_fields: &ToolboxIdlProgramTypeFlatFields,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypeFullFields, ToolboxIdlError> {
        Ok(match type_flat_fields {
            ToolboxIdlProgramTypeFlatFields::Named(fields) => {
                let mut fields_full = vec![];
                for (field_name, type_flat) in fields {
                    fields_full.push((
                        field_name.to_string(),
                        ToolboxIdlProgramTypeFull::try_hydrate(
                            program_typedefs,
                            generics_by_symbol,
                            type_flat,
                            breadcrumbs,
                        )?,
                    ));
                }
                ToolboxIdlProgramTypeFullFields::Named(fields_full)
            },
            ToolboxIdlProgramTypeFlatFields::Unamed(fields) => {
                let mut fields_type_full = vec![];
                for field_type_flat in fields {
                    fields_type_full.push(
                        ToolboxIdlProgramTypeFull::try_hydrate(
                            program_typedefs,
                            generics_by_symbol,
                            field_type_flat,
                            breadcrumbs,
                        )?,
                    );
                }
                ToolboxIdlProgramTypeFullFields::Unamed(fields_type_full)
            },
            ToolboxIdlProgramTypeFlatFields::None => {
                ToolboxIdlProgramTypeFullFields::None
            },
        })
    }
}
