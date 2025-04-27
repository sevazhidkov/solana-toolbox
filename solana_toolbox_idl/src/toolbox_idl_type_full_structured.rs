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
    // TODO - should the repr be a struct/enum?
    pub fn structured_repr_c(
        self,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        Ok(match self {
            ToolboxIdlTypeFull::Option { prefix, content } => {
                ToolboxIdlTypeFull::structured_repr_c_option(prefix, *content)?
            },
            ToolboxIdlTypeFull::Vec { .. } => {
                return Err(anyhow!(
                    "Vec is not supported in structured_repr_c"
                ));
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::structured_repr_c_array(*items, length)
                    .context("Structuring Array")?
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                ToolboxIdlTypeFull::structured_repr_c_struct(fields)?
            },
            ToolboxIdlTypeFull::Enum {
                prefix, variants, ..
            } => ToolboxIdlTypeFull::structured_repr_c_enum(prefix, variants)?,
            ToolboxIdlTypeFull::Padded {
                before,
                size,
                after,
                content,
            } => ToolboxIdlTypeFull::structured_repr_c_padded(
                before, size, after, *content,
            )?,
            ToolboxIdlTypeFull::Const { .. } => {
                return Err(anyhow!(
                    "Const is not supported in structured_repr_c"
                ));
            },
            ToolboxIdlTypeFull::Primitive { primitive } => (
                primitive.alignment(),
                primitive.size(),
                ToolboxIdlTypeFull::Primitive { primitive },
            ),
        })
    }

    fn structured_repr_c_option(
        option_prefix: ToolboxIdlTypePrefix,
        option_content: ToolboxIdlTypeFull,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (content_alignment, content_size, content_structured) =
            option_content
                .structured_repr_c()
                .context("Structuring Option Content")?;
        let alignment = max(option_prefix.size(), content_alignment);
        let size = alignment + content_size;
        Ok((
            alignment,
            size,
            ToolboxIdlTypeFull::Padded {
                before: None,
                size: Some(u64::try_from(size)?),
                after: None,
                content: Box::new(ToolboxIdlTypeFull::Option {
                    prefix: prefix_from_alignment(alignment)?,
                    content: Box::new(content_structured),
                }),
            },
        ))
    }

    fn structured_repr_c_array(
        items: ToolboxIdlTypeFull,
        length: u64,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (items_alignment, items_size, items_structured) = items
            .structured_repr_c()
            .context("Structuring Array Items")?;
        Ok((
            items_alignment,
            items_size * usize::try_from(length)?,
            ToolboxIdlTypeFull::Array {
                items: Box::new(items_structured),
                length,
            },
        ))
    }

    fn structured_repr_c_struct(
        fields: ToolboxIdlTypeFullFields,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (fields_alignment, fields_size, fields_structured) =
            fields.structured_repr_c()?;
        Ok((
            fields_alignment,
            fields_size,
            ToolboxIdlTypeFull::Struct {
                fields: fields_structured,
            },
        ))
    }

    fn structured_repr_c_enum(
        prefix: ToolboxIdlTypePrefix,
        variants: Vec<ToolboxIdlTypeFullEnumVariant>,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let mut alignment = prefix.size();
        let mut size = 0;
        let mut variants_structured = vec![];
        for variant in variants {
            let (variant_alignment, variant_size, variant_structured) =
                variant.structured_repr_c()?;
            alignment = max(alignment, variant_alignment);
            size = max(size, variant_size);
            variants_structured.push(variant_structured);
        }
        size += alignment;
        Ok((
            alignment,
            size,
            ToolboxIdlTypeFull::Padded {
                before: None,
                size: Some(u64::try_from(size)?),
                after: None,
                content: Box::new(ToolboxIdlTypeFull::Enum {
                    prefix: prefix_from_alignment(alignment)?,
                    variants: variants_structured,
                }),
            },
        ))
    }

    fn structured_repr_c_padded(
        before: Option<u64>,
        size: Option<u64>,
        after: Option<u64>,
        content: ToolboxIdlTypeFull,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (content_alignment, content_size, content_structured) = content
            .structured_repr_c()
            .context("Structuring Padded Content")?;
        if let Some(before) = before {
            if usize::try_from(before)? % content_alignment != 0 {
                return Err(anyhow!(
                    "Padded before {} is not aligned to content alignment of {} in structured_repr_c",
                    before,
                    content_alignment
                ));
            }
        }
        if let Some(size) = size {
            if usize::try_from(size)? % content_alignment != 0 {
                return Err(anyhow!(
                    "Padded size {} is not aligned to content alignment of {} in structured_repr_c",
                    size,
                    content_alignment
                ));
            }
        }
        if let Some(after) = after {
            if usize::try_from(after)? % content_alignment != 0 {
                return Err(anyhow!(
                    "Padded after {} is not aligned to content alignment of {} in structured_repr_c",
                    after,
                    content_alignment
                ));
            }
        }
        Ok((
            content_alignment,
            content_size,
            ToolboxIdlTypeFull::Padded {
                before,
                size,
                after,
                content: Box::new(content_structured),
            },
        ))
    }
}

impl ToolboxIdlTypeFullEnumVariant {
    pub fn structured_repr_c(
        self,
    ) -> Result<(usize, usize, ToolboxIdlTypeFullEnumVariant)> {
        let (fields_alignment, fields_size, fields_structured) =
            self.fields.structured_repr_c().with_context(|| {
                anyhow!("Structuring Enum Variant Fields: {}", self.name)
            })?;
        Ok((
            fields_alignment,
            fields_size,
            ToolboxIdlTypeFullEnumVariant {
                name: self.name,
                code: self.code,
                fields: fields_structured,
            },
        ))
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn structured_repr_c(
        self,
    ) -> Result<(usize, usize, ToolboxIdlTypeFullFields)> {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut alignment = 1;
                let mut size = 0;
                let mut last_field_info: Option<(
                    String,
                    usize,
                    ToolboxIdlTypeFull,
                )> = None;
                let mut fields_structured = vec![];
                for field in fields {
                    let (
                        field_content_alignment,
                        field_content_size,
                        field_content_structured,
                    ) = field.content.structured_repr_c().with_context(
                        || anyhow!("Structuring field: {}", field.name),
                    )?;
                    alignment = max(alignment, field_content_alignment);
                    if let Some((
                        last_field_name,
                        last_field_content_size,
                        last_field_content_structured,
                    )) = last_field_info
                    {
                        let (
                            last_field_content_size,
                            last_field_content_padded,
                        ) = field_content_padded(
                            size,
                            field_content_alignment,
                            last_field_content_size,
                            last_field_content_structured,
                        )?;
                        size += last_field_content_size;
                        fields_structured.push(ToolboxIdlTypeFullFieldNamed {
                            name: last_field_name,
                            content: last_field_content_padded,
                        });
                    }
                    last_field_info = Some((
                        field.name,
                        field_content_size,
                        field_content_structured,
                    ));
                }
                if let Some((
                    last_field_name,
                    last_field_content_size,
                    last_field_content_structured,
                )) = last_field_info
                {
                    let (last_field_content_size, last_field_content_padded) =
                        field_content_padded(
                            size,
                            alignment,
                            last_field_content_size,
                            last_field_content_structured,
                        )?;
                    size += last_field_content_size;
                    fields_structured.push(ToolboxIdlTypeFullFieldNamed {
                        name: last_field_name,
                        content: last_field_content_padded,
                    });
                }
                Ok((
                    alignment,
                    size,
                    ToolboxIdlTypeFullFields::Named(fields_structured),
                ))
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let mut alignment = 1;
                let mut size = 0;
                let mut last_field_info: Option<(usize, ToolboxIdlTypeFull)> =
                    None;
                let mut fields_structured = vec![];
                for (index, field) in fields.into_iter().enumerate() {
                    let (
                        field_content_alignment,
                        field_content_size,
                        field_content_structured,
                    ) = field.content.structured_repr_c().with_context(
                        || anyhow!("Structuring Field: {}", index),
                    )?;
                    alignment = max(alignment, field_content_alignment);
                    if let Some((
                        last_field_content_size,
                        last_field_content_structured,
                    )) = last_field_info
                    {
                        let (
                            last_field_content_size,
                            last_field_content_padded,
                        ) = field_content_padded(
                            size,
                            field_content_alignment,
                            last_field_content_size,
                            last_field_content_structured,
                        )?;
                        size += last_field_content_size;
                        fields_structured.push(
                            ToolboxIdlTypeFullFieldUnnamed {
                                content: last_field_content_padded,
                            },
                        );
                    }
                    last_field_info =
                        Some((field_content_size, field_content_structured));
                }
                if let Some((
                    last_field_content_size,
                    last_field_content_structured,
                )) = last_field_info
                {
                    let (last_field_content_size, last_field_content_padded) =
                        field_content_padded(
                            size,
                            alignment,
                            last_field_content_size,
                            last_field_content_structured,
                        )?;
                    size += last_field_content_size;
                    fields_structured.push(ToolboxIdlTypeFullFieldUnnamed {
                        content: last_field_content_padded,
                    });
                }
                Ok((
                    alignment,
                    size,
                    ToolboxIdlTypeFullFields::Unnamed(fields_structured),
                ))
            },
            ToolboxIdlTypeFullFields::None => {
                Ok((1, 0, ToolboxIdlTypeFullFields::None))
            },
        }
    }
}

fn prefix_from_alignment(alignment: usize) -> Result<ToolboxIdlTypePrefix> {
    Ok(match alignment {
        1 => ToolboxIdlTypePrefix::U8,
        2 => ToolboxIdlTypePrefix::U16,
        4 => ToolboxIdlTypePrefix::U32,
        8 => ToolboxIdlTypePrefix::U64,
        _ => {
            return Err(anyhow!(
                "Prefix alignment {} is not supported",
                alignment
            ))
        },
    })
}

fn field_content_padded(
    offset_before: usize,
    desired_alignment: usize,
    field_content_size: usize,
    field_content_structured: ToolboxIdlTypeFull,
) -> Result<(usize, ToolboxIdlTypeFull)> {
    let offset_after = offset_before + field_content_size;
    let missalignment = offset_after % desired_alignment;
    if missalignment == 0 {
        return Ok((field_content_size, field_content_structured));
    }
    let padding = desired_alignment - missalignment;
    let size = field_content_size + padding;
    Ok((
        size,
        ToolboxIdlTypeFull::Padded {
            before: None,
            size: Some(u64::try_from(size)?),
            after: None,
            content: Box::new(field_content_structured),
        },
    ))
}
