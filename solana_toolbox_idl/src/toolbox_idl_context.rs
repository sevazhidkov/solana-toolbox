#[derive(Debug, Clone)]
pub struct ToolboxIdlContext {
    idl: String,
    val: String,
}

impl ToolboxIdlContext {
    pub fn new(
        idl: &str,
        val: &str,
    ) -> ToolboxIdlContext {
        ToolboxIdlContext { idl: idl.to_string(), val: val.to_string() }
    }

    pub fn describe(&self) -> String {
        format!("idl({}) val({})", self.idl, self.val)
    }
}
