use anyhow::Result;
use anyhow::anyhow;
use reactive_graph_graph::Component;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::FlowInstance;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationType;
use schemars::_private::serde_json;
use schemars::schema_for;
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
        JsonSchemaTypes::Component => schema_for!(Component),
        JsonSchemaTypes::EntityType => schema_for!(EntityType),
        JsonSchemaTypes::RelationType => schema_for!(RelationType),
        JsonSchemaTypes::FlowType => schema_for!(FlowType),
        JsonSchemaTypes::EntityInstance => schema_for!(EntityInstance),
        JsonSchemaTypes::RelationInstance => schema_for!(RelationInstance),
        JsonSchemaTypes::FlowInstance => schema_for!(FlowInstance),
    };
    match serde_json::to_string_pretty(&json_schema.clone().to_value()) {
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
