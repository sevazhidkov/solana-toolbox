use std::collections::HashMap;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_type::ToolboxIdlProgramType;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdlTypeFull {
    pub fn try_hydrate(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFull>,
        type_flat: &ToolboxIdlTypeFlat,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFull, ToolboxIdlError> {
        Ok(match type_flat {
            ToolboxIdlTypeFlat::Defined { name, generics } => {
                let mut generics_full = vec![];
                for generic_flat in generics {
                    generics_full.push(ToolboxIdlTypeFull::try_hydrate(
                        program_types,
                        generics_by_symbol,
                        generic_flat,
                        breadcrumbs,
                    )?);
                }
                let program_type = idl_map_get_key_or_else(
                    program_types,
                    name,
                    &breadcrumbs.idl(),
                )?;
                if generics_full.len() != program_type.generics.len() {
                    return idl_err(
                        "Wrong number of generic parameter",
                        &breadcrumbs.val(),
                    );
                }
                let mut generics_by_symbol = HashMap::new();
                for (program_type_generic_name, generic_full) in
                    program_type.generics.iter().zip(generics_full)
                {
                    generics_by_symbol.insert(
                        program_type_generic_name.to_string(),
                        generic_full,
                    );
                }
                ToolboxIdlTypeFull::try_hydrate(
                    program_types,
                    &generics_by_symbol,
                    &program_type.type_flat,
                    breadcrumbs,
                )?
            },
            ToolboxIdlTypeFlat::Option { content } => {
                ToolboxIdlTypeFull::Option {
                    content: Box::new(ToolboxIdlTypeFull::try_hydrate(
                        program_types,
                        generics_by_symbol,
                        content,
                        breadcrumbs,
                    )?),
                }
            },
            ToolboxIdlTypeFlat::Vec { items } => ToolboxIdlTypeFull::Vec {
                items: Box::new(ToolboxIdlTypeFull::try_hydrate(
                    program_types,
                    generics_by_symbol,
                    items,
                    breadcrumbs,
                )?),
            },
            ToolboxIdlTypeFlat::Array { items, length } => {
                ToolboxIdlTypeFull::Array {
                    items: Box::new(ToolboxIdlTypeFull::try_hydrate(
                        program_types,
                        generics_by_symbol,
                        items,
                        breadcrumbs,
                    )?),
                    length: *idl_ok_or_else(
                        ToolboxIdlTypeFull::try_hydrate(
                            program_types,
                            generics_by_symbol,
                            length,
                            breadcrumbs,
                        )?
                        .as_const_literal(),
                        "expected a const literal",
                        &breadcrumbs.idl(),
                    )?,
                }
            },
            ToolboxIdlTypeFlat::Struct { fields } => {
                ToolboxIdlTypeFull::Struct {
                    fields: ToolboxIdlTypeFullFields::try_hydrate(
                        program_types,
                        generics_by_symbol,
                        fields,
                        breadcrumbs,
                    )?,
                }
            },
            ToolboxIdlTypeFlat::Enum { variants } => {
                let mut variants_full = vec![];
                for (variant_name, variant_fields) in variants {
                    variants_full.push((
                        variant_name.to_string(),
                        ToolboxIdlTypeFullFields::try_hydrate(
                            program_types,
                            generics_by_symbol,
                            variant_fields,
                            breadcrumbs,
                        )?,
                    ));
                }
                ToolboxIdlTypeFull::Enum { variants: variants_full }
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
                ToolboxIdlTypeFull::Primitive { primitive: primitive.clone() }
            },
        })
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn try_hydrate(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFull>,
        type_flat_fields: &ToolboxIdlTypeFlatFields,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFullFields, ToolboxIdlError> {
        Ok(match type_flat_fields {
            ToolboxIdlTypeFlatFields::Named(fields) => {
                let mut fields_full = vec![];
                for (field_name, type_flat) in fields {
                    fields_full.push((
                        field_name.to_string(),
                        ToolboxIdlTypeFull::try_hydrate(
                            program_types,
                            generics_by_symbol,
                            type_flat,
                            breadcrumbs,
                        )?,
                    ));
                }
                ToolboxIdlTypeFullFields::Named(fields_full)
            },
            ToolboxIdlTypeFlatFields::Unamed(fields) => {
                let mut fields_type_full = vec![];
                for field_type_flat in fields {
                    fields_type_full.push(ToolboxIdlTypeFull::try_hydrate(
                        program_types,
                        generics_by_symbol,
                        field_type_flat,
                        breadcrumbs,
                    )?);
                }
                ToolboxIdlTypeFullFields::Unamed(fields_type_full)
            },
            ToolboxIdlTypeFlatFields::None => ToolboxIdlTypeFullFields::None,
        })
    }
}
