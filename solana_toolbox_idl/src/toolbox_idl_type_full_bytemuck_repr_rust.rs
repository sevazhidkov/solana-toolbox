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
            } => {
                // TODO - this is not correct
                (
                    prefix.to_size(),
                    prefix.to_size(),
                    ToolboxIdlTypeFull::Enum { prefix, variants },
                )
            },
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
                min_size: u64::try_from(size)?,
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
            fields.bytemuck_repr_rust()?;
        Ok((
            fields_alignment,
            fields_size,
            ToolboxIdlTypeFull::Struct {
                fields: fields_repr_rust,
            },
        ))
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn bytemuck_repr_rust(
        self,
    ) -> Result<(usize, usize, ToolboxIdlTypeFullFields)> {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut fields_infos = vec![];
                for field in fields {
                    let (
                        field_content_alignment,
                        field_content_size,
                        field_content_repr_rust,
                    ) = field.content.bytemuck_repr_rust().with_context(
                        || anyhow!("Bytemuck: Repr(C): Field: {}", field.name),
                    )?;
                    fields_infos.push((
                        field.name,
                        field_content_alignment,
                        field_content_size,
                        field_content_repr_rust,
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
                        field_content_repr_rust,
                    ) = field.content.bytemuck_repr_rust().with_context(
                        || anyhow!("Bytemuck: Repr(C): Field: {}", index),
                    )?;
                    fields_infos.push((
                        index,
                        field_content_alignment,
                        field_content_size,
                        field_content_repr_rust,
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

// TODO - put this in a common place
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
            field_content_repr_rust,
        ) = field_info;
        alignment = max(alignment, field_content_alignment);
        if let Some((
            last_field_meta,
            last_field_content_size,
            last_field_content_repr_rust,
        )) = last_field_info
        {
            let (last_field_content_size, last_field_content_padded) =
                field_content_padded(
                    size,
                    field_content_alignment,
                    last_field_content_size,
                    last_field_content_repr_rust,
                )?;
            size += last_field_content_size;
            fields_infos_padded
                .push((last_field_meta, last_field_content_padded));
        }
        last_field_info =
            Some((field_meta, field_content_size, field_content_repr_rust));
    }
    if let Some((
        last_field_meta,
        last_field_content_size,
        last_field_content_repr_rust,
    )) = last_field_info
    {
        let (last_field_content_size, last_field_content_padded) =
            field_content_padded(
                size,
                alignment,
                last_field_content_size,
                last_field_content_repr_rust,
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
    field_content_repr_rust: ToolboxIdlTypeFull,
) -> Result<(usize, ToolboxIdlTypeFull)> {
    let offset_after = offset_before + field_content_size;
    let missalignment = offset_after % desired_alignment;
    if missalignment == 0 {
        return Ok((field_content_size, field_content_repr_rust));
    }
    let padding = desired_alignment - missalignment;
    let size = field_content_size + padding;
    Ok((
        size,
        ToolboxIdlTypeFull::Padded {
            before: 0,
            min_size: u64::try_from(size)?,
            after: 0,
            content: Box::new(field_content_repr_rust),
        },
    ))
}
