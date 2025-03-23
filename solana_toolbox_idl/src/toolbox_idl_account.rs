use std::sync::Arc;

use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_utils::idl_convert_to_type_name;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlAccount {
    pub name: String,
    pub docs: Option<Value>,
    pub space: Option<usize>,
    // TODO - support discrimination by data chunks (for token account for example)
    pub discriminator: Vec<u8>,
    pub content_type_flat: ToolboxIdlTypeFlat,
    pub content_type_full: Arc<ToolboxIdlTypeFull>,
}

impl Default for ToolboxIdlAccount {
    fn default() -> ToolboxIdlAccount {
        ToolboxIdlAccount {
            name: ToolboxIdlAccount::sanitize_name("UnknownAccount"),
            docs: None,
            space: None,
            discriminator: vec![],
            content_type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::None,
            },
            content_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::None,
            }
            .into(),
        }
    }
}

impl ToolboxIdlAccount {
    pub fn sanitize_name(name: &str) -> String {
        idl_convert_to_type_name(name)
    }

    pub fn compile(
        &self,
        account_state: &Value,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let mut account_data = vec![];
        account_data.extend_from_slice(&self.discriminator);
        self.content_type_full.try_serialize(
            account_state,
            &mut account_data,
            true,
            &ToolboxIdlBreadcrumbs::default(),
        )?;
        Ok(account_data)
    }

    pub fn decompile(
        &self,
        account_data: &[u8],
    ) -> Result<Value, ToolboxIdlError> {
        if !account_data.starts_with(&self.discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: self.discriminator.to_vec(),
                found: account_data.to_vec(),
            });
        }
        if let Some(space) = self.space {
            if account_data.len() != space {
                return Err(ToolboxIdlError::InvalidSpace {
                    expected: space,
                    found: account_data.len(),
                });
            }
        }
        let (_, account_value) = self.content_type_full.try_deserialize(
            account_data,
            self.discriminator.len(),
            &ToolboxIdlBreadcrumbs::default(),
        )?;
        Ok(account_value)
    }
}
