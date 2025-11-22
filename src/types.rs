use serde::{Serialize,Deserialize};

#[derive(Debug,Deserialize)]
pub struct DeployRequest {
    pub program_path : String,
    pub network : String
}

#[derive(Debug,Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}
