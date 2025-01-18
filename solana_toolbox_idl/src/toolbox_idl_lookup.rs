use crate::toolbox_idl::ToolboxIdl;

impl ToolboxIdl {
    pub fn lookup_error_name(
        &self,
        error_code: u64,
    ) -> Option<&String> {
        for (idl_error_name, idl_error_code) in self.errors_codes.iter() {
            if let Some(code) = idl_error_code.as_u64() {
                if error_code == code {
                    return Some(idl_error_name);
                }
            }
        }
        None
    }
}
