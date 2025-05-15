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
use crate::toolbox_idl_utils::idl_padding_entries;
use crate::toolbox_idl_utils::idl_padding_needed;

impl ToolboxIdlTypeFull {
    pub fn bytemuck_repr_rust(
        self,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        Ok(match self {
            ToolboxIdlTypeFull::Typedef {
                name,
                repr,
                content,
            } => content.bytemuck_typedef(&name, &repr)?,
            ToolboxIdlTypeFull::Pod {
                alignment,
                size,
                content,
            } => (alignment, size, *content),
            ToolboxIdlTypeFull::Option { prefix, content } => {
                ToolboxIdlTypeFull::bytemuck_repr_rust_option(prefix, *content)?
            },
            ToolboxIdlTypeFull::Vec { .. } => {
                return Err(anyhow!(
                    "Bytemuck: Repr(Rust): Vec is not supported"
                ));
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::bytemuck_repr_rust_array(*items, length)?
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                ToolboxIdlTypeFull::bytemuck_repr_rust_struct(fields)?
            },
            ToolboxIdlTypeFull::Enum {
                prefix, variants, ..
            } => ToolboxIdlTypeFull::bytemuck_repr_rust_enum(prefix, variants)?,
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

    fn bytemuck_repr_rust_option(
        option_prefix: ToolboxIdlTypePrefix,
        option_content: ToolboxIdlTypeFull,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (content_alignment, content_size, content_repr_rust) =
            option_content.bytemuck_repr_rust()?;
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
                    content: Box::new(content_repr_rust),
                }),
            },
        ))
    }

    fn bytemuck_repr_rust_array(
        items: ToolboxIdlTypeFull,
        length: u64,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (items_alignment, items_size, items_repr_rust) =
            items.bytemuck_repr_rust()?;
        Ok((
            items_alignment,
            items_size * usize::try_from(length)?,
            ToolboxIdlTypeFull::Array {
                items: Box::new(items_repr_rust),
                length,
            },
        ))
    }

    fn bytemuck_repr_rust_struct(
        fields: ToolboxIdlTypeFullFields,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (fields_alignment, fields_size, fields_repr_rust) =
            fields.bytemuck_repr_rust(0)?;
        Ok((
            fields_alignment,
            fields_size,
            ToolboxIdlTypeFull::Struct {
                fields: fields_repr_rust,
            },
        ))
    }

    fn bytemuck_repr_rust_enum(
        prefix: ToolboxIdlTypePrefix,
        variants: Vec<ToolboxIdlTypeFullEnumVariant>,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let mut alignment = prefix.to_size();
        let mut size = prefix.to_size();
        let mut variants_repr_rust = vec![];
        for variant in variants {
            let (variant_alignment, variant_size, variant_repr_rust) =
                variant.bytemuck_repr_rust(prefix.to_size())?;
            alignment = max(alignment, variant_alignment);
            size = max(size, variant_size);
            variants_repr_rust.push(variant_repr_rust);
        }
        size += idl_padding_needed(size, alignment);
        Ok((
            alignment,
            size,
            ToolboxIdlTypeFull::Padded {
                before: 0,
                min_size: size,
                after: 0,
                content: Box::new(ToolboxIdlTypeFull::Enum {
                    prefix,
                    variants: variants_repr_rust,
                }),
            },
        ))
    }
}

impl ToolboxIdlTypeFullEnumVariant {
    pub fn bytemuck_repr_rust(
        self,
        prefix_size: usize,
    ) -> Result<(usize, usize, ToolboxIdlTypeFullEnumVariant)> {
        let (fields_alignment, fields_size, fields_repr_rust) = self
            .fields
            .bytemuck_repr_rust(prefix_size)
            .with_context(|| {
                anyhow!("Bytemuck: Repr(Rust): Enum Variant: {}", self.name)
            })?;
        Ok((
            fields_alignment,
            fields_size,
            ToolboxIdlTypeFullEnumVariant {
                name: self.name,
                code: self.code,
                fields: fields_repr_rust,
            },
        ))
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn bytemuck_repr_rust(
        self,
        prefix_size: usize,
    ) -> Result<(usize, usize, ToolboxIdlTypeFullFields)> {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut fields_infos = vec![];
                for field in fields {
                    let (field_alignment, field_size, field_repr_rust) = field
                        .content
                        .bytemuck_repr_rust()
                        .with_context(|| {
                            anyhow!(
                                "Bytemuck: Repr(Rust): Field: {}",
                                field.name
                            )
                        })?;
                    fields_infos.push((
                        field.name,
                        field_alignment,
                        field_size,
                        field_repr_rust,
                    ));
                }
                fields_infos.sort_by(|a, b| b.2.cmp(&a.2));
                let (alignment, size, fields_infos_padded) =
                    idl_padding_entries(
                        prefix_size,
                        prefix_size,
                        fields_infos,
                    )?;
                Ok((
                    alignment,
                    size,
                    ToolboxIdlTypeFullFields::Named(
                        fields_infos_padded
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
                    let (field_alignment, field_size, field_repr_rust) = field
                        .content
                        .bytemuck_repr_rust()
                        .with_context(|| {
                            anyhow!("Bytemuck: Repr(Rust): Field: {}", index)
                        })?;
                    fields_infos.push((
                        index,
                        field_alignment,
                        field_size,
                        field_repr_rust,
                    ));
                }
                let (alignment, size, fields_infos_padded) =
                    idl_padding_entries(
                        prefix_size,
                        prefix_size,
                        fields_infos,
                    )?;
                Ok((
                    alignment,
                    size,
                    ToolboxIdlTypeFullFields::Unnamed(
                        fields_infos_padded
                            .into_iter()
                            .map(|field_info_padded| {
                                ToolboxIdlTypeFullFieldUnnamed {
                                    content: field_info_padded.1,
                                }
                            })
                            .collect(),
                    ),
                ))
            },
            ToolboxIdlTypeFullFields::None => {
                Ok((1, 0, ToolboxIdlTypeFullFields::None))
            },
        }
    }
}
