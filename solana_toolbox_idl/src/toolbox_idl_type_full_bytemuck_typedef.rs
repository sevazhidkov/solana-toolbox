use anyhow::anyhow;
use anyhow::Result;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;

impl ToolboxIdlTypeFull {
    pub fn bytemuck_typedef(
        self,
        name: &str,
        repr: &Option<String>,
    ) -> Result<(usize, usize, ToolboxIdlTypeFull)> {
        let (typedef_alignment, typedef_size, typedef_content) = match repr {
            Some(repr) if repr == "c" => self.bytemuck_repr_c(),
            Some(repr) if repr == "transparent" => self.bytemuck_repr_rust(),
            Some(repr) if repr == "rust" => self.bytemuck_repr_rust(),
            Some(repr) if repr == "packed" => {
                return Err(anyhow!("Bytemuck: Repr(packed) is not supported"))
            },
            _ => self.bytemuck_repr_rust(),
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
