use std::collections::HashMap;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_type::ToolboxIdlProgramType;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
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
                // TODO - what if generics array is not correct length
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
            ToolboxIdlTypeFlat::Vec { items } => {
                ToolboxIdlTypeFull::Vec {
                    items: Box::new(ToolboxIdlTypeFull::try_hydrate(
                        program_types,
                        generics_by_symbol,
                        items,
                        breadcrumbs,
                    )?),
                }
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
                            &length,
                            breadcrumbs,
                        )?
                        .as_const_literal(),
                        "expected a const literal",
                        &breadcrumbs.idl(),
                    )?,
                }
            },
            ToolboxIdlTypeFlat::Struct { fields } => {
                let mut ref_fields = vec![];
                for (field_name, field_def) in fields {
                    ref_fields.push((
                        field_name.to_string(),
                        ToolboxIdlTypeFull::try_hydrate(
                            program_types,
                            generics_by_symbol,
                            field_def,
                            breadcrumbs,
                        )?,
                    ));
                }
                ToolboxIdlTypeFull::Struct { fields: ref_fields }
            },
            ToolboxIdlTypeFlat::Enum { variants } => {
                let mut ref_variants = vec![];
                for (variant_name, variant_defs) in variants {
                    let mut ref_variant_fields = vec![];
                    for (variant_field_name, variant_field_def) in variant_defs
                    {
                        ref_variant_fields.push((
                            variant_field_name.to_string(),
                            ToolboxIdlTypeFull::try_hydrate(
                                program_types,
                                generics_by_symbol,
                                variant_field_def,
                                breadcrumbs,
                            )?,
                        ));
                    }
                    ref_variants
                        .push((variant_name.to_string(), ref_variant_fields));
                }
                ToolboxIdlTypeFull::Enum { variants: ref_variants }
            },
            ToolboxIdlTypeFlat::Generic { symbol } => {
                idl_map_get_key_or_else(
                    generics_by_symbol,
                    &symbol,
                    &breadcrumbs.idl(),
                )?
                .clone()
            },
            ToolboxIdlTypeFlat::Const { literal } => {
                ToolboxIdlTypeFull::Const { literal: *literal }
            },
            ToolboxIdlTypeFlat::Primitive { primitive } => {
                ToolboxIdlTypeFull::Primitive { primitive: primitive.clone() }
            },
        })
    }
}
