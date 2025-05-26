use crate::server::json_schema::commands::InstancesCommands;
use crate::server::json_schema::commands::JsonSchemaCommands;
use crate::server::json_schema::commands::TypesCommands;
use reactive_graph_instance_system_json_schema::schema_entity_instances;
use reactive_graph_instance_system_json_schema::schema_flow_instances;
use reactive_graph_instance_system_json_schema::schema_relation_instances;
use reactive_graph_type_system_json_schema::schema_components;
use reactive_graph_type_system_json_schema::schema_entity_types;
use reactive_graph_type_system_json_schema::schema_flow_types;
use reactive_graph_type_system_json_schema::schema_relation_types;
use std::process::exit;

pub mod args;
pub mod commands;

pub async fn print_json_schema_and_exit(commands: &JsonSchemaCommands) {
    let json_schema = match commands {
        JsonSchemaCommands::Types(args) => match args.commands {
            TypesCommands::Components => schema_components(),
            TypesCommands::Entities => schema_entity_types(),
            TypesCommands::Relations => schema_relation_types(),
            TypesCommands::Flows => schema_flow_types(),
        },
        JsonSchemaCommands::Instances(args) => match args.commands {
            InstancesCommands::Entities => schema_entity_instances(),
            InstancesCommands::Relations => schema_relation_instances(),
            InstancesCommands::Flows => schema_flow_instances(),
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
