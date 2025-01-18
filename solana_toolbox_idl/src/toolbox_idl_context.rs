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
        eprintln!("context: idl({}) val({})", idl, val);
        ToolboxIdlContext { idl: idl.to_string(), val: val.to_string() }
    }
}
