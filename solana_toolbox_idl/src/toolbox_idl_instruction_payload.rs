use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;

impl ToolboxIdlInstruction {
    pub fn encode_payload(
        &self,
        instruction_payload: &Value,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let mut instruction_data = vec![];
        instruction_data.extend_from_slice(&self.discriminator);
        self.args_type_full_fields.try_serialize(
            instruction_payload,
            &mut instruction_data,
            true,
            &ToolboxIdlBreadcrumbs::default(),
        )?;
        Ok(instruction_data)
    }

    pub fn decode_payload(
        &self,
        instruction_data: &[u8],
    ) -> Result<Value, ToolboxIdlError> {
        if !instruction_data.starts_with(&self.discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: self.discriminator.to_vec(),
                found: instruction_data.to_vec(),
            });
        }
        let (_, instruction_payload) =
            self.args_type_full_fields.try_deserialize(
                instruction_data,
                self.discriminator.len(),
                &ToolboxIdlBreadcrumbs::default(),
            )?;
        Ok(instruction_payload)
    }
}
