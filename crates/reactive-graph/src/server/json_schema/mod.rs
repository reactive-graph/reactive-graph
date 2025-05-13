use crate::server::json_schema::commands::InstancesCommands;
use crate::server::json_schema::commands::JsonSchemaCommands;
use crate::server::json_schema::commands::TypesCommands;
use reactive_graph_graph::Component;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::FlowInstance;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationType;
use schemars::schema_for;
use std::process::exit;

pub mod args;
pub mod commands;

pub async fn print_json_schema_and_exit(commands: &JsonSchemaCommands) {
    let json_schema = match commands {
        JsonSchemaCommands::Types(args) => match args.commands {
            TypesCommands::Components => schema_for!(Component),
            TypesCommands::Entities => schema_for!(EntityType),
            TypesCommands::Relations => schema_for!(RelationType),
            TypesCommands::Flows => schema_for!(FlowType),
        },
        JsonSchemaCommands::Instances(args) => match args.commands {
            InstancesCommands::Entities => schema_for!(EntityInstance),
            InstancesCommands::Relations => schema_for!(RelationInstance),
            InstancesCommands::Flows => schema_for!(FlowInstance),
        },
    };
    match serde_json::to_string_pretty(&json_schema.to_value()) {
        Ok(json_schema) => {
            println!("{}", json_schema);
            exit(0);
        }
        Err(_) => {
            exit(1);
        }
    }
}
