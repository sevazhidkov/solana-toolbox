use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_path::ToolboxIdlPathPart;
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
                if let ToolboxIdlPathPart::Key(key) = current {
                    return Err(anyhow!("Invalid Vec Index: {}", key));
                }
                next.try_get_type_full(items)
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                if let ToolboxIdlPathPart::Key(key) = current {
                    return Err(anyhow!("Invalid Array Index: {}", key));
                }
                if let ToolboxIdlPathPart::Code(code) = current {
                    if code >= *length {
                        return Err(anyhow!(
                            "Invalid Array index: {} (length: {})",
                            code,
                            length
                        ));
                    }
                }
                next.try_get_type_full(items)
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                self.try_get_type_full_fields(fields)
            },
            ToolboxIdlTypeFull::Enum { variants, .. } => match current {
                ToolboxIdlPathPart::Empty => {
                    Err(anyhow!("Invalid Enum Variant: Empty String"))
                },
                ToolboxIdlPathPart::Key(key) => {
                    for variant in variants {
                        if variant.name == key {
                            return next
                                .try_get_type_full_fields(&variant.fields);
                        }
                    }
                    Err(anyhow!("Could not find enum variant: {}", key))
                },
                ToolboxIdlPathPart::Code(code) => {
                    for variant in variants {
                        if variant.code == code {
                            return next
                                .try_get_type_full_fields(&variant.fields);
                        }
                    }
                    Err(anyhow!("Could not find enum variant: {}", code))
                },
            },
            ToolboxIdlTypeFull::Padded { content, .. } => {
                self.try_get_type_full(content)
            },
            ToolboxIdlTypeFull::Const { .. } => Err(anyhow!(
                "Type literal does not contain path: {}",
                self.value()
            )),
            ToolboxIdlTypeFull::Primitive { .. } => Err(anyhow!(
                "Type primitive does not contain path: {}",
                self.value()
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
                self.value()
            )),
            ToolboxIdlTypeFullFields::Named(fields) => {
                let key = current.value();
                for field in fields {
                    if field.name == key {
                        return next.try_get_type_full(&field.type_full);
                    }
                }
                Err(anyhow!("Could not find named field: {}", key))
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let length = fields.len();
                let index =
                    usize::try_from(current.code().context("Field index")?)?;
                if index >= length {
                    return Err(anyhow!(
                        "Invalid field index: {} (length: {})",
                        index,
                        length
                    ));
                }
                next.try_get_type_full(&fields[index].type_full)
            },
        }
    }
}
