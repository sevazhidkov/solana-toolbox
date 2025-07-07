use anyhow::anyhow;
use anyhow::Result;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;

impl ToolboxIdlTypeFull {
    pub fn bytemuck(
        self,
        name: &str,
        repr: &Option<String>,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (typedef_alignment, typedef_size, typedef_content) = match repr {
            None => self.bytemuck_rust(),
            Some(repr) if repr == "c" => self.bytemuck_c(),
            Some(repr) if repr == "rust" => self.bytemuck_rust(),
            Some(repr) if repr == "transparent" => self.bytemuck_rust(),
            Some(repr) => {
                // TODO - enums repr u16 cannot be supported properly ??
                // TODO - REPR unsupported modifiers: packed/align ?
                return Err(anyhow!("Bytemuck: Unsupported Repr: {}", repr));
            },
        }?;
        Ok((
            typedef_alignment,
            typedef_size,
            ToolboxIdlTypeFull::Typedef {
                name: name.to_string(),
                repr: repr.clone(),
                content: Box::new(typedef_content),
            },
        ))
    }
}
