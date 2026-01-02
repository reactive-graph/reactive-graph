use anyhow::Result;
use anyhow::anyhow;
use reactive_graph_instance_system_json_schema::schema_entity_instances;
use reactive_graph_instance_system_json_schema::schema_flow_instances;
use reactive_graph_instance_system_json_schema::schema_relation_instances;
use reactive_graph_type_system_json_schema::schema_components;
use reactive_graph_type_system_json_schema::schema_entity_types;
use reactive_graph_type_system_json_schema::schema_flow_types;
use reactive_graph_type_system_json_schema::schema_relation_types;
use serde_json::to_string_pretty;
use std::fs::write;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use workspace_root::get_workspace_root;

#[derive(Debug, Clone)]
pub enum JsonSchemaTypes {
    Component,
    EntityType,
    RelationType,
    FlowType,
    EntityInstance,
    RelationInstance,
    FlowInstance,
}

impl JsonSchemaTypes {
    pub fn schema_path(&self) -> PathBuf {
        match self {
            JsonSchemaTypes::Component => Path::new("schema/json/component.schema.json"),
            JsonSchemaTypes::EntityType => Path::new("schema/json/entity-type.schema.json"),
            JsonSchemaTypes::RelationType => Path::new("schema/json/relation-type.schema.json"),
            JsonSchemaTypes::FlowType => Path::new("schema/json/flow-type.schema.json"),
            JsonSchemaTypes::EntityInstance => Path::new("schema/json/entity-instance.schema.json"),
            JsonSchemaTypes::RelationInstance => Path::new("schema/json/relation-instance.schema.json"),
            JsonSchemaTypes::FlowInstance => Path::new("schema/json/flow-instance.schema.json"),
        }
        .to_owned()
    }
}

pub fn generate_json_schema(schema_type: &JsonSchemaTypes) -> String {
    let json_schema = match schema_type {
        JsonSchemaTypes::Component => schema_components(),
        JsonSchemaTypes::EntityType => schema_entity_types(),
        JsonSchemaTypes::RelationType => schema_relation_types(),
        JsonSchemaTypes::FlowType => schema_flow_types(),
        JsonSchemaTypes::EntityInstance => schema_entity_instances(),
        JsonSchemaTypes::RelationInstance => schema_relation_instances(),
        JsonSchemaTypes::FlowInstance => schema_flow_instances(),
    };
    match to_string_pretty(&json_schema.clone().to_value()) {
        Ok(json_schema) => json_schema,
        Err(_) => json_schema.to_value().to_string(),
    }
}

pub fn write_json_schema(schema_type: JsonSchemaTypes) -> Result<()> {
    let schema_path = get_workspace_root().join(schema_type.schema_path());
    if !schema_path.exists() {
        eprintln!("Schema path doesn't exist: {:?}", schema_path.display());
        exit(1);
    }
    write(schema_path.clone(), generate_json_schema(&schema_type))
        .map_err(|_| anyhow!("Failed to write JSON schema {:?} to {:?}", schema_type, schema_path))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::JsonSchemaTypes;
    use super::generate_json_schema;

    #[test]
    fn test_generate_json_schemas() {
        assert!(generate_json_schema(&JsonSchemaTypes::Component).len() > 0);
        assert!(generate_json_schema(&JsonSchemaTypes::EntityType).len() > 0);
        assert!(generate_json_schema(&JsonSchemaTypes::RelationType).len() > 0);
        assert!(generate_json_schema(&JsonSchemaTypes::FlowType).len() > 0);
        assert!(generate_json_schema(&JsonSchemaTypes::EntityInstance).len() > 0);
        assert!(generate_json_schema(&JsonSchemaTypes::RelationInstance).len() > 0);
        assert!(generate_json_schema(&JsonSchemaTypes::FlowInstance).len() > 0);
    }
}
