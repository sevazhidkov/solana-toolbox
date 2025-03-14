use serde_json::Value;

use crate::ToolboxIdlBreadcrumbs;
use crate::ToolboxIdlError;
use crate::ToolboxIdlInstruction;

impl ToolboxIdlInstruction {
    pub fn compile_args(
        &self,
        instruction_args: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let mut instruction_data = vec![];
        instruction_data.extend_from_slice(&self.discriminator);
        self.args_type_full_fields.try_serialize(
            instruction_args,
            &mut instruction_data,
            true,
            breadcrumbs,
        )?;
        Ok(instruction_data)
    }

    pub fn decompile_args(
        &self,
        instruction_data: &[u8],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Value, ToolboxIdlError> {
        if !instruction_data.starts_with(&self.discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: self.discriminator.to_vec(),
                found: instruction_data.to_vec(),
            });
        }
        let (_, instruction_args) =
            self.args_type_full_fields.try_deserialize(
                instruction_data,
                self.discriminator.len(),
                breadcrumbs,
            )?;
        Ok(instruction_args)
    }
}
