use std::cmp::max;
use std::fmt::Debug;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullEnumVariant;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldNamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldUnnamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;
use crate::toolbox_idl_utils::idl_alignment_padding_needed;
use crate::toolbox_idl_utils::idl_fields_infos_aligned;

impl ToolboxIdlTypeFull {
    pub fn bytemuck_rust(self) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        Ok(match self {
            ToolboxIdlTypeFull::Typedef {
                name,
                repr,
                content,
            } => content.bytemuck(&name, &repr)?,
            ToolboxIdlTypeFull::Option { prefix, content } => {
                ToolboxIdlTypeFull::bytemuck_rust_option(prefix, *content)?
            },
            ToolboxIdlTypeFull::Vec { .. } => {
                return Err(anyhow!(
                    "Bytemuck: Repr(Rust): Vec is not supported"
                ));
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::bytemuck_rust_array(*items, length)?
            },
            ToolboxIdlTypeFull::String { .. } => {
                return Err(anyhow!(
                    "Bytemuck: Repr(Rust): String is not supported"
                ));
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                ToolboxIdlTypeFull::bytemuck_rust_struct(fields)?
            },
            ToolboxIdlTypeFull::Enum {
                prefix, variants, ..
            } => ToolboxIdlTypeFull::bytemuck_rust_enum(prefix, variants)?,
            ToolboxIdlTypeFull::Padded { .. } => {
                return Err(anyhow!(
                    "Bytemuck: Repr(Rust): Padded is not supported"
                ));
            },
            ToolboxIdlTypeFull::Const { .. } => {
                return Err(anyhow!(
                    "Bytemuck: Repr(Rust): Const is not supported"
                ));
            },
            ToolboxIdlTypeFull::Primitive { primitive } => (
                primitive.alignment(),
                primitive.size(),
                ToolboxIdlTypeFull::Primitive { primitive },
            ),
        })
    }

    fn bytemuck_rust_option(
        option_prefix: ToolboxIdlTypePrefix,
        option_content: ToolboxIdlTypeFull,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (content_alignment, content_size, content_rust) =
            option_content.bytemuck_rust()?;
        let alignment = max(option_prefix.to_size(), content_alignment);
        let size = alignment + content_size;
        Ok((
            alignment,
            size,
            ToolboxIdlTypeFull::Padded {
                before: 0,
                min_size: size,
                after: 0,
                content: Box::new(ToolboxIdlTypeFull::Option {
                    prefix: ToolboxIdlTypePrefix::from_size(alignment)?,
                    content: Box::new(content_rust),
                }),
            },
        ))
    }

    fn bytemuck_rust_array(
        items: ToolboxIdlTypeFull,
        length: usize,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (items_alignment, items_size, items_rust) =
            items.bytemuck_rust()?;
        Ok((
            items_alignment,
            items_size * length,
            ToolboxIdlTypeFull::Array {
                items: Box::new(items_rust),
                length,
            },
        ))
    }

    fn bytemuck_rust_struct(
        fields: ToolboxIdlTypeFullFields,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (fields_alignment, fields_size, fields_rust) =
            fields.bytemuck_rust(0)?;
        Ok((
            fields_alignment,
            fields_size,
            ToolboxIdlTypeFull::Struct {
                fields: fields_rust,
            },
        ))
    }

    fn bytemuck_rust_enum(
        prefix: ToolboxIdlTypePrefix,
        variants: Vec<ToolboxIdlTypeFullEnumVariant>,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let mut alignment = prefix.to_size();
        let mut size = prefix.to_size();
        let mut variants_rust = vec![];
        for variant in variants {
            let (
                variant_fields_alignment,
                variant_fields_size,
                variant_fields_rust,
            ) = variant
                .fields
                .bytemuck_rust(prefix.to_size())
                .with_context(|| {
                    anyhow!(
                        "Bytemuck: Repr(Rust): Enum Variant: {}",
                        variant.name
                    )
                })?;
            alignment = max(alignment, variant_fields_alignment);
            size = max(size, variant_fields_size);
            variants_rust.push(ToolboxIdlTypeFullEnumVariant {
                name: variant.name,
                code: variant.code,
                fields: variant_fields_rust,
            });
        }
        size += idl_alignment_padding_needed(size, alignment);
        Ok((
            alignment,
            size,
            ToolboxIdlTypeFull::Padded {
                before: 0,
                min_size: size,
                after: 0,
                content: Box::new(ToolboxIdlTypeFull::Enum {
                    prefix,
                    variants: variants_rust,
                }),
            },
        ))
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn bytemuck_rust(
        self,
        prefix_size: usize,
    ) -> Result<(usize, usize, ToolboxIdlTypeFullFields)> {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut fields_infos = vec![];
                for field in fields {
                    let (field_alignment, field_size, field_rust) =
                        field.content.bytemuck_rust().with_context(|| {
                            anyhow!(
                                "Bytemuck: Repr(Rust): Field: {}",
                                field.name
                            )
                        })?;
                    fields_infos.push((
                        field_alignment,
                        field_size,
                        field.name,
                        field_rust,
                    ));
                }
                verify_unstable_fields_infos(prefix_size, &fields_infos)?;
                let (alignment, size, fields_infos) =
                    idl_fields_infos_aligned(prefix_size, fields_infos)?;
                Ok((
                    alignment,
                    size,
                    ToolboxIdlTypeFullFields::Named(
                        fields_infos
                            .into_iter()
                            .map(|field_info_padded| {
                                ToolboxIdlTypeFullFieldNamed {
                                    name: field_info_padded.0,
                                    content: field_info_padded.1,
                                }
                            })
                            .collect(),
                    ),
                ))
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let mut fields_infos = vec![];
                for field in fields {
                    let (field_alignment, field_size, field_rust) =
                        field.content.bytemuck_rust().with_context(|| {
                            anyhow!(
                                "Bytemuck: Repr(Rust): Field: {}",
                                field.position
                            )
                        })?;
                    fields_infos.push((
                        field_alignment,
                        field_size,
                        field.position,
                        field_rust,
                    ));
                }
                verify_unstable_fields_infos(prefix_size, &fields_infos)?;
                let (alignment, size, fields_infos) =
                    idl_fields_infos_aligned(prefix_size, fields_infos)?;
                Ok((
                    alignment,
                    size,
                    ToolboxIdlTypeFullFields::Unnamed(
                        fields_infos
                            .into_iter()
                            .map(|field_info_padded| {
                                ToolboxIdlTypeFullFieldUnnamed {
                                    position: field_info_padded.0,
                                    content: field_info_padded.1,
                                }
                            })
                            .collect(),
                    ),
                ))
            },
        }
    }
}

fn verify_unstable_fields_infos<T: Debug>(
    prefix_size: usize,
    fields_infos: &[(usize, usize, T, ToolboxIdlTypeFull)],
) -> Result<()> {
    if prefix_size == 0 && fields_infos.len() <= 2 {
        return Ok(());
    }
    if fields_infos.len() <= 1 {
        return Ok(());
    }
    Err(anyhow!("Bytemuck: Repr(Rust): Structs/Enums/Tuples fields ordering is compiler-dependent. Use Repr(C) instead."))
}
