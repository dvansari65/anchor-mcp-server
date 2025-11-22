use std::default;

use rmcp::model::{Implementation, ProtocolVersion, ServerCapabilities, ServerInfo};
use tracing::{info,debug,error};
use tracing_subscriber::{layer::SubscriberExt,util::SubscriberInitExt};

mod tools;
mod types;
mod anchor;

#[tokio::main]
async fn main(){
   setup_loggings();

   info!("Starting mcp server!!");

   let server = ServerInfo{
    protocol_version:ProtocolVersion::V_2024_11_05,
    capabilities:ServerCapabilities::builder()
                    .enable_tools()
                    .build(),
    server_info:Implementation::from_build_env(),
    instructions:Some("anchor mcp server".to_string())
   };

   

}

fn setup_loggings () -> Result<(),Box<dyn std::error::Error>>{
    tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_subscriber::EnvFilter::new("info"))
            .init();
    Ok(())
}
