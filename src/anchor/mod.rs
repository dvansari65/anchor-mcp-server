
use std::process::Command;
use crate::types::CommandResult;
use tracing::{info,error};


pub fn deploy_program (program_path:&str , network : &str)->CommandResult {
    
    let output = Command::new("anchor")
                        .arg("deploy")
                        .arg("--provider.cluster")
                        .arg(network)
                        .current_dir(program_path)
                        .output();
    match output {
        Ok(out)=> {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            CommandResult {
                success: out.status.success(),
                output:stdout,
                error: if stderr.is_empty() {None} else{ Some(stderr)}
            }
        },
        Err(e)=>{
            error!("Failed to deploy anchor deploy:{}",e);
            CommandResult {
                success:false,
                output:String::new(),
                error:Some(e.to_string())
            }
        }
    }
}

