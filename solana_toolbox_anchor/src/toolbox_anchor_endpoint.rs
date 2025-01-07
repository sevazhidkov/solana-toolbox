use std::ops::Deref;
use std::ops::DerefMut;

use solana_toolbox_endpoint::ToolboxEndpoint;

pub struct ToolboxAnchorEndpoint {
    inner: ToolboxEndpoint,
}

impl From<ToolboxEndpoint> for ToolboxAnchorEndpoint {
    fn from(inner: ToolboxEndpoint) -> Self {
        Self { inner }
    }
}

impl Deref for ToolboxAnchorEndpoint {
    type Target = ToolboxEndpoint;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ToolboxAnchorEndpoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[macro_export]
macro_rules! toolbox_endpoint_program_test_builtin_program_anchor {
    ($program_id:expr, $program_entry:expr) => {
        $crate::ToolboxEndpointProgramTestBuiltinProgram {
            id: $program_id,
            name: "",
            processor: $crate::solana_program_test_processor!(
                |program_id, accounts, data| {
                    let accounts = Box::leak(Box::new(accounts.to_vec()));
                    $program_entry(program_id, accounts, data)
                }
            ),
        }
    };
}
