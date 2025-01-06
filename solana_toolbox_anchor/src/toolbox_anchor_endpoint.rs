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
