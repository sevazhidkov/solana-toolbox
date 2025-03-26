use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_transaction_error::ToolboxIdlTransactionError;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64_or_else;

impl ToolboxIdlTransactionError {
    pub fn try_parse(
        idl_error_name: &str,
        idl_error: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTransactionError, ToolboxIdlError> {
        if let Some(idl_error) = idl_error.as_object() {
            let code = idl_object_get_key_as_u64_or_else(
                idl_error,
                "code",
                &breadcrumbs.idl(),
            )?;
            let msg = idl_object_get_key_as_str(idl_error, "msg")
                .unwrap_or("")
                .to_string();
            return Ok(ToolboxIdlTransactionError {
                name: idl_error_name.to_string(),
                code,
                msg,
            });
        }
        if let Some(code) = idl_error.as_u64() {
            return Ok(ToolboxIdlTransactionError {
                name: idl_error_name.to_string(),
                code,
                msg: "".to_string(),
            });
        }
        // TODO (MEDIUM) - better error handling
        idl_err("Unparsable error", &breadcrumbs.as_idl("@"))
    }
}
