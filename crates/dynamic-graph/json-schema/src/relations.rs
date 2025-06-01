use crate::data_type::JsonSchemaDataType;
use crate::extensions::EXTENSION_JSON_SCHEMA_FORMAT;
use crate::id::dynamic_schema_id;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
use schemars::Schema;
use schemars::consts::meta_schemas::DRAFT2020_12;
use schemars::json_schema;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;

pub fn dynamic_json_schema_for_relation_type(relation_type: RelationType) -> Schema {
    let schema_id = dynamic_schema_id(&relation_type.ty);
    let mut properties = Map::new();
    properties.insert(
        "$id".to_string(),
        json!({
            "default": schema_id,
            "description": "The schema identifier",
            "type": "string"
        }),
    );
    properties.insert(
        "outbound_id".to_string(),
        json!({
            "type": "string",
            "format": "uuid"
        }),
    );
    properties.insert(
        "instance_id".to_string(),
        json!({
            "type": "string",
        }),
    );
    properties.insert(
        "inbound_id".to_string(),
        json!({
            "type": "string",
            "format": "uuid"
        }),
    );
    for entry in relation_type.properties.iter() {
        let property = entry.value();
        let property_name = entry.key().clone();
        if property_name == "$id" || property_name == "outbound_uuid" || property_name == "instance_id" || property_name == "inbound_uuid" {
            continue;
        }
        let mut json_schema_property = Map::new();
        json_schema_property.insert("type".to_string(), JsonSchemaDataType::new(property.data_type).to_value());
        if let Some(extension_json_schema_format) = property.get_extension(&EXTENSION_JSON_SCHEMA_FORMAT).map(|extension| extension.extension) {
            json_schema_property.insert("format".to_string(), extension_json_schema_format);
        }
        if !property.description.is_empty() {
            json_schema_property.insert("description".to_string(), json!(&property.description));
        }
        properties.insert(property_name, Value::Object(json_schema_property));
    }
    let mut required = relation_type.properties.names();
    required.push("outbound_uuid".to_string());
    required.push("instance_id".to_string());
    required.push("inbound_uuid".to_string());
    required.sort();
    json_schema!({
        "$schema": DRAFT2020_12,
        "$id": schema_id,
        "type": "object",
        "title": relation_type.type_name(),
        "description": relation_type.description,
        "properties": properties,
        "required": required,
    })
}

#[cfg(test)]
mod tests {
    use super::dynamic_json_schema_for_relation_type;
    use default_test::DefaultTest;
    use reactive_graph_graph::RelationType;

    #[test]
    fn test_generate_schema_for_entity_type() {
        let relation_type = RelationType::default_test();
        let schema = dynamic_json_schema_for_relation_type(relation_type.clone());
        println!("{}", serde_json::to_string_pretty(schema.as_value()).unwrap());
    }
}
