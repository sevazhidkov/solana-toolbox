use solana_sdk::signature::Signature;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;

#[derive(Default)]
pub struct ToolboxEndpointLoggerPrinter {} // TODO - probably should depreacte this now

#[async_trait::async_trait]
impl ToolboxEndpointLogger for ToolboxEndpointLoggerPrinter {
    async fn on_processed(
        &self,
        processed: &(Signature, ToolboxEndpointExecution),
    ) {
        println!("---------------------------- TRANSACTION PROCESSED -----------------------------");
        println!("----");
        println!("signature: {:?}", processed.0);
        println!("----");
        ToolboxEndpoint::print_execution(&processed.1);
        println!("----");
        ToolboxEndpoint::print_backtrace("from");
        println!();
    }
}
