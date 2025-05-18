use schemars::Schema;
use serde_json::json;

pub(crate) fn add_json_schema_id_property(schema: &mut Schema, id: &str) {
    let root_schema = schema.ensure_object();
    root_schema
        .get_mut("properties")
        .and_then(|properties| properties.as_object_mut())
        .map(|properties| {
            properties.insert(
                "$id".to_owned(),
                json!({
                    "default": id.to_owned(),
                    "description": "The schema identifier",
                    "type": "string"
                }),
            )
        })
        .expect("Inserting $id into json schema failed: Expected field 'properties'");
}
