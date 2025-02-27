use solana_sdk::signature::Signature;

#[async_trait::async_trait]
pub trait ToolboxEndpointLogger {
    async fn on_signature(
        &self,
        signature: &Signature,
    );
}
