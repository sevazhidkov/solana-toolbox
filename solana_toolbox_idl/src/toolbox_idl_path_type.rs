use anyhow::anyhow;
use anyhow::Result;

use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

impl ToolboxIdlPath {
    pub fn try_extract_type_full(
        &self,
        type_full: &ToolboxIdlTypeFull,
    ) -> Result<ToolboxIdlTypeFull> {
        let Some((current, next)) = self.split_first() else {
            return Ok(type_full.clone());
        };
        match type_full {
            ToolboxIdlTypeFull::Option { content, .. } => {
                self.try_extract_type_full(content)
            },
            ToolboxIdlTypeFull::Vec { items } => {
                let _index = current.parse::<u64>()?;
                next.try_extract_type_full(items)
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                let index = current.parse::<u64>()?;
                if index >= *length {
                    return Err(anyhow!(
                        "Invalid array index: {} (length: {})",
                        index,
                        length
                    ));
                }
                next.try_extract_type_full(items)
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                self.try_extract_type_full_fields(fields)
            },
            ToolboxIdlTypeFull::Enum { variants } => {
                for (variant_name, variant_fields) in variants {
                    if variant_name == &current {
                        return next
                            .try_extract_type_full_fields(variant_fields);
                    }
                }
                Err(anyhow!("Could not find enum variant: {}", current))
            },
            ToolboxIdlTypeFull::Padded { content, .. } => {
                self.try_extract_type_full(content)
            },
            ToolboxIdlTypeFull::Const { .. } => {
                Err(anyhow!("Type literal does not contain: {}", current))
            },
            ToolboxIdlTypeFull::Primitive { .. } => {
                Err(anyhow!("Type primitive does not contain: {}", current))
            },
        }
    }

    pub fn try_extract_type_full_fields(
        &self,
        type_full_fields: &ToolboxIdlTypeFullFields,
    ) -> Result<ToolboxIdlTypeFull> {
        let Some((current, next)) = self.split_first() else {
            return Ok(ToolboxIdlTypeFull::Struct {
                fields: type_full_fields.clone(),
            });
        };
        match type_full_fields {
            ToolboxIdlTypeFullFields::None => {
                Err(anyhow!("Fields does not contain: {}", current))
            },
            ToolboxIdlTypeFullFields::Named(fields) => {
                for (field_name, field_type) in fields {
                    if field_name == &current {
                        return next.try_extract_type_full(field_type);
                    }
                }
                Err(anyhow!("Could not find named field: {}", current))
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let length = fields.len();
                let index = current.parse::<usize>()?;
                if index >= length {
                    return Err(anyhow!(
                        "Invalid field index: {} (length: {})",
                        index,
                        length
                    ));
                }
                next.try_extract_type_full(&fields[index])
            },
        }
    }
}
