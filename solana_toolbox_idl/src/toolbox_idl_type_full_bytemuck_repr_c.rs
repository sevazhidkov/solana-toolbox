use std::cmp::max;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullEnumVariant;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldNamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldUnnamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;
use crate::toolbox_idl_utils::idl_fields_infos_aligned;

impl ToolboxIdlTypeFull {
    pub fn bytemuck_repr_c(self) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        Ok(match self {
            ToolboxIdlTypeFull::Typedef {
                name,
                repr,
                content,
            } => content.bytemuck_typedef(&name, &repr).with_context(|| {
                anyhow!("Bytemuck: Repr(C): Typedef: {}", name)
            })?,
            ToolboxIdlTypeFull::Option { prefix, content } => {
                ToolboxIdlTypeFull::bytemuck_repr_c_option(prefix, *content)?
            },
            ToolboxIdlTypeFull::Vec { .. } => {
                return Err(anyhow!("Bytemuck: Repr(C): Vec is not supported"));
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::bytemuck_repr_c_array(*items, length)?
            },
            ToolboxIdlTypeFull::String { .. } => {
                return Err(anyhow!(
                    "Bytemuck: Repr(C): String is not supported"
                ));
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                ToolboxIdlTypeFull::bytemuck_repr_c_struct(fields)?
            },
            ToolboxIdlTypeFull::Enum {
                prefix, variants, ..
            } => ToolboxIdlTypeFull::bytemuck_repr_c_enum(prefix, variants)?,
            ToolboxIdlTypeFull::Padded { .. } => {
                return Err(anyhow!(
                    "Bytemuck: Repr(C): Padded is not supported"
                ));
            },
            ToolboxIdlTypeFull::Const { .. } => {
                return Err(anyhow!(
                    "Bytemuck: Repr(C): Const is not supported"
                ));
            },
            ToolboxIdlTypeFull::Primitive { primitive } => (
                primitive.alignment(),
                primitive.size(),
                ToolboxIdlTypeFull::Primitive { primitive },
            ),
        })
    }

    fn bytemuck_repr_c_option(
        option_prefix: ToolboxIdlTypePrefix,
        option_content: ToolboxIdlTypeFull,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (content_alignment, content_size, content_repr_c) =
            option_content.bytemuck_repr_c()?;
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
                    content: Box::new(content_repr_c),
                }),
            },
        ))
    }

    fn bytemuck_repr_c_array(
        items: ToolboxIdlTypeFull,
        length: usize,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (items_alignment, items_size, items_repr_c) =
            items.bytemuck_repr_c()?;
        Ok((
            items_alignment,
            items_size * length,
            ToolboxIdlTypeFull::Array {
                items: Box::new(items_repr_c),
                length,
            },
        ))
    }

    fn bytemuck_repr_c_struct(
        fields: ToolboxIdlTypeFullFields,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (fields_alignment, fields_size, fields_repr_c) =
            fields.bytemuck_repr_c()?;
        Ok((
            fields_alignment,
            fields_size,
            ToolboxIdlTypeFull::Struct {
                fields: fields_repr_c,
            },
        ))
    }

    fn bytemuck_repr_c_enum(
        prefix: ToolboxIdlTypePrefix,
        variants: Vec<ToolboxIdlTypeFullEnumVariant>,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let mut alignment = max(4, prefix.to_size());
        let mut size = 0;
        let mut variants_repr_c = vec![];
        for variant in variants {
            let (
                variant_fields_alignment,
                variant_fields_size,
                variant_fields_repr_c,
            ) = variant.fields.bytemuck_repr_c().with_context(|| {
                anyhow!("Bytemuck: Repr(C): Enum Variant: {}", variant.name)
            })?;
            alignment = max(alignment, variant_fields_alignment);
            size = max(size, variant_fields_size);
            variants_repr_c.push(ToolboxIdlTypeFullEnumVariant {
                name: variant.name,
                code: variant.code,
                fields: variant_fields_repr_c,
            });
        }
        size += alignment;
        Ok((
            alignment,
            size,
            ToolboxIdlTypeFull::Padded {
                before: 0,
                min_size: size,
                after: 0,
                content: Box::new(ToolboxIdlTypeFull::Enum {
                    prefix: ToolboxIdlTypePrefix::from_size(alignment)?,
                    variants: variants_repr_c,
                }),
            },
        ))
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn bytemuck_repr_c(
        self,
    ) -> Result<(usize, usize, ToolboxIdlTypeFullFields)> {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut fields_infos = vec![];
                for field in fields {
                    let (field_alignment, field_size, field_repr_c) = field
                        .content
                        .bytemuck_repr_rust()
                        .with_context(|| {
                            anyhow!("Bytemuck: Repr(C): Field: {}", field.name)
                        })?;
                    fields_infos.push((
                        field_alignment,
                        field_size,
                        field.name,
                        field_repr_c,
                    ));
                }
                let (alignment, size, fields_infos) =
                    idl_fields_infos_aligned(0, fields_infos)?;
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
                for (index, field) in fields.into_iter().enumerate() {
                    let (field_alignment, field_size, field_repr_c) =
                        field.content.bytemuck_repr_rust().with_context(
                            || anyhow!("Bytemuck: Repr(C): Field: {}", index),
                        )?;
                    fields_infos.push((
                        field_alignment,
                        field_size,
                        index,
                        field_repr_c,
                    ));
                }
                let (alignment, size, fields_infos) =
                    idl_fields_infos_aligned(0, fields_infos)?;
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
