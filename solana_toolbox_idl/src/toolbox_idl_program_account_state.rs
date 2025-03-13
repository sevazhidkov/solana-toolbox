use serde_json::Value;

use crate::{ToolboxIdlBreadcrumbs, ToolboxIdlError, ToolboxIdlProgramAccount};

impl ToolboxIdlProgramAccount {
    pub fn compile_state(
        &self,
        account_state: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let mut account_data = vec![];
        account_data.extend_from_slice(&self.discriminator);
        self.data_type_full.try_serialize(
            account_state,
            &mut account_data,
            true,
            breadcrumbs,
        )?;
        Ok(account_data)
    }

    pub fn decompile_state(
        &self,
        account_data: &[u8],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Value, ToolboxIdlError> {
        if !account_data.starts_with(&self.discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: self.discriminator.to_vec(),
                found: account_data.to_vec(),
            });
        }
        let (_, account_state) = self.data_type_full.try_deserialize(
            account_data,
            self.discriminator.len(),
            breadcrumbs,
        )?;
        Ok(account_state)
    }
}
