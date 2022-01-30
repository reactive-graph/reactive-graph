use crate::tests::utils::r_string;
use crate::{ReactiveEntityInstance, ReactiveRelationInstance};
use indradb::{Edge, EdgeKey, EdgeProperties, Identifier, NamedProperty};
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub fn create_random_relation_instance(
    outbound_entity: Arc<ReactiveEntityInstance>,
    inbound_entity: Arc<ReactiveEntityInstance>,
    property_name: String,
) -> ReactiveRelationInstance {
    let type_name = r_string();
    let t = Identifier::from_str(type_name.as_str()).unwrap();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: Identifier::new(property_name).unwrap(),
        value: property_value_json,
    };
    let properties = vec![property];
    let outbound_id = outbound_entity.id;
    let inbound_id = inbound_entity.id;
    let edge_key = EdgeKey::new(outbound_id, t, inbound_id);
    let edge_properties = EdgeProperties::new(
        Edge::new_with_current_datetime(edge_key),
        properties.clone(),
    );
    let outbound_entity = outbound_entity.clone();
    let inbound_entity = outbound_entity.clone();
    ReactiveRelationInstance::from(outbound_entity, inbound_entity, edge_properties)
}

pub fn create_random_relation_instance_with_properties(
    outbound_entity: Arc<ReactiveEntityInstance>,
    inbound_entity: Arc<ReactiveEntityInstance>,
    property_name: String,
) -> ReactiveRelationInstance {
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), json!(r_string()));
    ReactiveRelationInstance::create_with_properties(
        outbound_entity.clone(),
        r_string(),
        inbound_entity.clone(),
        properties,
    )
}
