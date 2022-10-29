use std::collections::HashMap;
use std::sync::Arc;

use indradb::Edge;
use indradb::EdgeKey;
use indradb::EdgeProperties;
use indradb::NamedProperty;
use serde_json::json;

use crate::property_identifier;
use crate::tests::utils::r_string;
use crate::ReactiveEntityInstance;
use crate::ReactiveRelationInstance;
use crate::RelationTypeId;
use crate::TypeDefinitionGetter;

pub fn create_random_relation_instance(
    outbound_entity: Arc<ReactiveEntityInstance>,
    inbound_entity: Arc<ReactiveEntityInstance>,
    property_name: String,
) -> ReactiveRelationInstance {
    let namespace = r_string();
    let type_name = r_string();
    let ty = RelationTypeId::new_from_type(&namespace, &type_name);
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: property_identifier(&property_name),
        value: property_value_json,
    };
    let properties = vec![property];
    let outbound_id = outbound_entity.id;
    let inbound_id = inbound_entity.id;
    let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);
    let edge_properties = EdgeProperties::new(Edge::new_with_current_datetime(edge_key), properties.clone());
    let outbound_entity = outbound_entity.clone();
    let inbound_entity = outbound_entity.clone();
    ReactiveRelationInstance::new_from_properties(outbound_entity, inbound_entity, edge_properties).unwrap()
}

pub fn create_random_relation_instance_with_properties(
    outbound_entity: Arc<ReactiveEntityInstance>,
    inbound_entity: Arc<ReactiveEntityInstance>,
    property_name: String,
) -> ReactiveRelationInstance {
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), json!(r_string()));
    ReactiveRelationInstance::new_from_type_with_properties(r_string(), outbound_entity.clone(), r_string(), inbound_entity.clone(), properties)
}
