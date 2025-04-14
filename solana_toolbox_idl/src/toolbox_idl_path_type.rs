use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

impl ToolboxIdlPath {
    pub fn try_get_type_full(
        &self,
        type_full: &ToolboxIdlTypeFull,
    ) -> Result<ToolboxIdlTypeFull> {
        let Some((current, next)) = self.split_first() else {
            return Ok(type_full.clone());
        };
        match type_full {
            ToolboxIdlTypeFull::Option { content, .. } => {
                self.try_get_type_full(content)
            },
            ToolboxIdlTypeFull::Vec { items, .. } => {
                let _index = current.index().context("Vec index")?;
                next.try_get_type_full(items)
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                let index = current.index().context("Array index")?;
                if u64::try_from(index)? >= *length {
                    return Err(anyhow!(
                        "Invalid array index: {} (length: {})",
                        index,
                        length
                    ));
                }
                next.try_get_type_full(items)
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                self.try_get_type_full_fields(fields)
            },
            ToolboxIdlTypeFull::Enum { variants, .. } => {
                let key = current.key().context("Enum variant")?;
                for (variant_name, variant_fields) in variants {
                    if variant_name == key {
                        return next.try_get_type_full_fields(variant_fields);
                    }
                }
                Err(anyhow!("Could not find enum variant: {}", key))
            },
            ToolboxIdlTypeFull::Padded { content, .. } => {
                self.try_get_type_full(content)
            },
            ToolboxIdlTypeFull::Const { .. } => Err(anyhow!(
                "Type literal does not contain path: {}",
                self.export()
            )),
            ToolboxIdlTypeFull::Primitive { .. } => Err(anyhow!(
                "Type primitive does not contain path: {}",
                self.export()
            )),
        }
    }

    pub fn try_get_type_full_fields(
        &self,
        type_full_fields: &ToolboxIdlTypeFullFields,
    ) -> Result<ToolboxIdlTypeFull> {
        let Some((current, next)) = self.split_first() else {
            return Ok(ToolboxIdlTypeFull::Struct {
                fields: type_full_fields.clone(),
            });
        };
        match type_full_fields {
            ToolboxIdlTypeFullFields::None => Err(anyhow!(
                "Empty fields does not contain path: {}",
                self.export()
            )),
            ToolboxIdlTypeFullFields::Named(fields) => {
                let key = current.key().context("Field name")?;
                for (field_name, field_type) in fields {
                    if field_name == key {
                        return next.try_get_type_full(field_type);
                    }
                }
                Err(anyhow!("Could not find named field: {}", current.export()))
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let length = fields.len();
                let index = current.index().context("Field index")?;
                if index >= length {
                    return Err(anyhow!(
                        "Invalid field index: {} (length: {})",
                        index,
                        length
                    ));
                }
                next.try_get_type_full(&fields[index])
            },
        }
    }
}
