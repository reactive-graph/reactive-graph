use std::collections::HashMap;

use indradb::EdgeKey;
use serde_json::json;

use crate::builder::EntityTypeBuilder;
use crate::builder::RelationTypeBuilder;
use crate::get_runtime;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::RelationInstanceTypeId;
use crate::model::RelationTypeId;
use crate::model::TypeDefinitionGetter;
use crate::tests::utils::r_string;

#[test]
fn test_relation_edge_manager() {
    let runtime = get_runtime();
    let entity_type_manager = runtime.get_entity_type_manager();
    let relation_type_manager = runtime.get_relation_type_manager();
    let entity_vertex_manager = runtime.get_entity_vertex_manager();
    let relation_edge_manager = runtime.get_relation_edge_manager();

    let namespace = r_string();
    let outbound_type_name = r_string();
    let relation_type_name = r_string();
    let inbound_type_name = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());

    // Create entity types
    let entity_type = EntityTypeBuilder::new_from_type(namespace.as_str(), outbound_type_name.as_str()).build();
    let result = entity_type_manager.register(entity_type.clone());
    assert!(result.is_ok());
    let entity_type = EntityTypeBuilder::new_from_type(namespace.as_str(), inbound_type_name.as_str()).build();
    let result = entity_type_manager.register(entity_type.clone());
    assert!(result.is_ok());

    // Create relation type
    let outbound_ty = EntityTypeId::new_from_type(&namespace, &outbound_type_name);
    let relation_ty = RelationTypeId::new_from_type(&namespace, &relation_type_name);
    let inbound_ty = EntityTypeId::new_from_type(&namespace, &inbound_type_name);
    let component_ty = ComponentTypeId::new_from_type(&namespace, &r_string());
    let relation_type = RelationTypeBuilder::new(
        ComponentOrEntityTypeId::EntityType(outbound_ty.clone()),
        &relation_ty,
        ComponentOrEntityTypeId::EntityType(inbound_ty.clone()),
    )
    .component(&component_ty)
    .string_property(property_name.clone())
    .build();
    let result = relation_type_manager.register(relation_type.clone());
    assert!(result.is_ok());

    let outbound_entity = entity_vertex_manager.create(&outbound_ty, HashMap::new());
    let outbound_id = outbound_entity.unwrap();
    let inbound_entity = entity_vertex_manager.create(&inbound_ty, HashMap::new());
    let inbound_id = inbound_entity.unwrap();

    let relation_instance_ty = RelationInstanceTypeId::new_unique_id(relation_ty.clone());
    let edge_key = EdgeKey::new(outbound_id, relation_instance_ty.type_id(), inbound_id);

    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());
    let result = relation_edge_manager.create(&edge_key, properties);
    assert!(result.is_ok());

    // Check if the relation exists by edge key
    let actual_edge_key = result.unwrap();
    assert_eq!(edge_key, actual_edge_key);
    assert!(relation_edge_manager.has(&edge_key));

    let edge = relation_edge_manager.get(&edge_key);
    assert!(edge.is_some());
    assert_eq!(edge_key, edge.unwrap().key);

    let properties = relation_edge_manager.get_properties(&edge_key);
    assert!(properties.is_some());
    let properties = properties.unwrap();
    assert_eq!(edge_key, properties.edge.key);
    assert_eq!(format!("r__{}__{}", &namespace, &relation_type_name), properties.edge.key.t.to_string());
    assert_eq!(1, properties.props.len());
    let property = properties.props.get(0);
    assert!(property.is_some());
    let property = property.unwrap();
    assert_eq!(property_name.clone(), property.name.to_string());
    assert_eq!(property_value.clone(), property.value);

    // Delete edge
    relation_edge_manager.delete(&edge_key);
    // Check if edge is gone
    assert!(!relation_edge_manager.has(&edge_key));
    let edge = relation_edge_manager.get(&edge_key);
    assert!(!edge.is_some());
    let properties = relation_edge_manager.get_properties(&edge_key);
    assert!(!properties.is_some());
}
