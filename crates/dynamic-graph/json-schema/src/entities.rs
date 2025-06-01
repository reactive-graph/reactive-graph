use crate::data_type::JsonSchemaDataType;
use crate::extensions::EXTENSION_JSON_SCHEMA_FORMAT;
use crate::id::dynamic_schema_id;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::NamespacedTypeGetter;
use schemars::Schema;
use schemars::json_schema;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;

pub fn dynamic_json_schema_for_entity_type(entity_type: EntityType) -> Schema {
    let type_name = entity_type.type_name();
    let schema_id = dynamic_schema_id(&entity_type.ty);
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
        "id".to_string(),
        json!({
            "description": format!("The unique identifier of an entity instance of entity type {}", type_name),
            "type": "string",
            "format": "uuid"
        }),
    );
    for entry in entity_type.properties.iter() {
        // TODO: sort
        let property = entry.value();
        let property_name = entry.key().clone();
        if property_name == "$id" || property_name == "id" {
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
    // TODO: Extract required from property extension
    let mut required = entity_type.properties.names();
    required.push("id".to_string());
    required.sort();
    json_schema!({
        "$id": schema_id,
        "type": "object",
        "title": entity_type.type_name(),
        "description": entity_type.description,
        "properties": properties,
        "required": required,
    })
}

#[cfg(test)]
mod tests {
    use super::dynamic_json_schema_for_entity_type;
    use default_test::DefaultTest;
    use reactive_graph_graph::EntityType;

    #[test]
    fn test_generate_schema_for_entity_type() {
        let entity_type = EntityType::default_test();
        let schema = dynamic_json_schema_for_entity_type(entity_type);
        println!("{}", serde_json::to_string_pretty(schema.as_value()).unwrap());
    }
}
