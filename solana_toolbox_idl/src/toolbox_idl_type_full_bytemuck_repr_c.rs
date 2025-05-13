use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use std::cmp::max;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullEnumVariant;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldNamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldUnnamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;

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
            ToolboxIdlTypeFull::Pod {
                alignment,
                size,
                content,
            } => (alignment, size, *content),
            ToolboxIdlTypeFull::Option { prefix, content } => {
                ToolboxIdlTypeFull::bytemuck_repr_c_option(prefix, *content)?
            },
            ToolboxIdlTypeFull::Vec { .. } => {
                return Err(anyhow!("Bytemuck: Repr(C): Vec is not supported"));
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::bytemuck_repr_c_array(*items, length)?
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
                min_size: u64::try_from(size)?,
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
        length: u64,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (items_alignment, items_size, items_repr_c) =
            items.bytemuck_repr_c()?;
        Ok((
            items_alignment,
            items_size * usize::try_from(length)?,
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
            let (variant_alignment, variant_size, variant_repr_c) =
                variant.bytemuck_repr_c()?;
            alignment = max(alignment, variant_alignment);
            size = max(size, variant_size);
            variants_repr_c.push(variant_repr_c);
        }
        size += alignment;
        Ok((
            alignment,
            size,
            ToolboxIdlTypeFull::Padded {
                before: 0,
                min_size: u64::try_from(size)?,
                after: 0,
                content: Box::new(ToolboxIdlTypeFull::Enum {
                    prefix: ToolboxIdlTypePrefix::from_size(alignment)?,
                    variants: variants_repr_c,
                }),
            },
        ))
    }
}

impl ToolboxIdlTypeFullEnumVariant {
    pub fn bytemuck_repr_c(
        self,
    ) -> Result<(usize, usize, ToolboxIdlTypeFullEnumVariant)> {
        let (fields_alignment, fields_size, fields_repr_c) =
            self.fields.bytemuck_repr_c().with_context(|| {
                anyhow!("Bytemuck: Repr(C): Enum Variant: {}", self.name)
            })?;
        Ok((
            fields_alignment,
            fields_size,
            ToolboxIdlTypeFullEnumVariant {
                name: self.name,
                code: self.code,
                fields: fields_repr_c,
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
                    let (
                        field_content_alignment,
                        field_content_size,
                        field_content_repr_c,
                    ) = field.content.bytemuck_repr_c().with_context(|| {
                        anyhow!("Bytemuck: Repr(C): Field: {}", field.name)
                    })?;
                    fields_infos.push((
                        field.name,
                        field_content_alignment,
                        field_content_size,
                        field_content_repr_c,
                    ));
                }
                let (alignment, size, fields_infos_padded) =
                    fields_infos_padded(fields_infos)?;
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
                    let (
                        field_content_alignment,
                        field_content_size,
                        field_content_repr_c,
                    ) = field.content.bytemuck_repr_c().with_context(|| {
                        anyhow!("Bytemuck: Repr(C): Field: {}", index)
                    })?;
                    fields_infos.push((
                        index,
                        field_content_alignment,
                        field_content_size,
                        field_content_repr_c,
                    ));
                }
                let (alignment, size, fields_infos_padded) =
                    fields_infos_padded(fields_infos)?;
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

fn fields_infos_padded<T>(
    fields_infos: Vec<(T, usize, usize, ToolboxIdlTypeFull)>,
) -> Result<(usize, usize, Vec<(T, ToolboxIdlTypeFull)>)> {
    let mut alignment = 1;
    let mut size = 0;
    let mut last_field_info = None;
    let mut fields_infos_padded = vec![];
    for field_info in fields_infos {
        let (
            field_meta,
            field_content_alignment,
            field_content_size,
            field_content_repr_c,
        ) = field_info;
        alignment = max(alignment, field_content_alignment);
        if let Some((
            last_field_meta,
            last_field_content_size,
            last_field_content_repr_c,
        )) = last_field_info
        {
            let (last_field_content_size, last_field_content_padded) =
                field_content_padded(
                    size,
                    field_content_alignment,
                    last_field_content_size,
                    last_field_content_repr_c,
                )?;
            size += last_field_content_size;
            fields_infos_padded
                .push((last_field_meta, last_field_content_padded));
        }
        last_field_info =
            Some((field_meta, field_content_size, field_content_repr_c));
    }
    if let Some((
        last_field_meta,
        last_field_content_size,
        last_field_content_repr_c,
    )) = last_field_info
    {
        let (last_field_content_size, last_field_content_padded) =
            field_content_padded(
                size,
                alignment,
                last_field_content_size,
                last_field_content_repr_c,
            )?;
        size += last_field_content_size;
        fields_infos_padded.push((last_field_meta, last_field_content_padded));
    }
    Ok((alignment, size, fields_infos_padded))
}

fn field_content_padded(
    offset_before: usize,
    desired_alignment: usize,
    field_content_size: usize,
    field_content_repr_c: ToolboxIdlTypeFull,
) -> Result<(usize, ToolboxIdlTypeFull)> {
    let offset_after = offset_before + field_content_size;
    let missalignment = offset_after % desired_alignment;
    if missalignment == 0 {
        return Ok((field_content_size, field_content_repr_c));
    }
    let padding = desired_alignment - missalignment;
    let size = field_content_size + padding;
    Ok((
        size,
        ToolboxIdlTypeFull::Padded {
            before: 0,
            min_size: u64::try_from(size)?,
            after: 0,
            content: Box::new(field_content_repr_c),
        },
    ))
}
