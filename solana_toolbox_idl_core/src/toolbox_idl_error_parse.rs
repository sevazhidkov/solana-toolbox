use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::Value;

use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64_or_else;

impl ToolboxIdlError {
    pub fn try_parse(
        idl_error_name: &str,
        idl_error: &Value,
    ) -> Result<ToolboxIdlError> {
        if let Some(idl_error) = idl_error.as_object() {
            let docs = idl_error.get("docs").cloned();
            let code = idl_object_get_key_as_u64_or_else(idl_error, "code")
                .context("Parse Code")?;
            let msg = idl_object_get_key_as_str(idl_error, "msg")
                .map(|s| s.to_string());
            return Ok(ToolboxIdlError {
                name: idl_error_name.to_string(),
                docs,
                code,
                msg,
            });
        }
        if let Some(code) = idl_error.as_u64() {
            return Ok(ToolboxIdlError {
                name: idl_error_name.to_string(),
                docs: None,
                code,
                msg: None,
            });
        }
        Err(anyhow!("Unparsable error (expected an object or number)"))
    }
}
