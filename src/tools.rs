use std::sync::Arc;

use rmcp::{Json, model::{Tool,ToolAnnotations,Icon}};
use serde_json::json; 

pub fn deploy_tool () -> Vec<Tool> {
    let schema_value = json!({
        "type":"object",
        "properties" : {
            "program_path" : {
                "type" : "string",
                "description": "Path to the Anchor program directory"
            },
            "network":{
                "type" : "string",
                "enum" : ["devnet" , "testnet" , "mainnet-beta","localnet"]
            }
        },
        "required" : ["program_path","network"]
    });
    let schema_object = schema_value
                .as_object()
                .expect("Schema must be json format!")
                .clone();

    let output_schema_value = json!({
        "type":"object",
        "properties":{
            "program_id" : {
                "type" : "string"
            },
            "tx_signature" : {
                "type" : "signature"
            },
            "network" : {
                "type" : "string"
            },
            "success" :{
                "type" : "boolean"
            },
            "message" :{
                "type" : "string"
            }
        },
        "required" : ["program_id","tx_signature","network" ,"success" ]
    });

    let output_schema_object = output_schema_value
            .as_object()
            .expect("output schema must be json format!")
            .clone();

    vec![
        Tool {
            name:"deploy_program".to_string().into(),
            description:Some("This is the tool for deploying the program!".to_string().into()),
            input_schema:Arc::new(schema_object),
            title:Some("Deploy anchor program".to_string()),
            output_schema:Some(Arc::new(output_schema_object)),
            annotations: Some(ToolAnnotations {
                title:Some("Deploy anchor program".to_string()),
                read_only_hint:Some(false),
                destructive_hint:Some(false),
                idempotent_hint:Some(false),
                open_world_hint:Some(true)

            }),
            // ✅ icons example
            icons: Some(vec![
                Icon {
                    src:"https://example.com/rocket-icon.svg".to_string() , mime_type:None,sizes:None
                }
            ]),
        }
    ]
}