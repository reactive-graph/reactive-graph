use std::collections::HashMap;

use indradb::Datastore;
use serde_json::json;
use uuid::Uuid;

use crate::builder::EntityTypeBuilder;
use crate::get_runtime;
use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::TypeDefinitionGetter;
use crate::tests::utils::r_string;

#[test]
fn test_entity_vertex_manager() {
    let runtime = get_runtime();
    let entity_type_manager = runtime.get_entity_type_manager();
    let entity_vertex_manager = runtime.get_entity_vertex_manager();
    let graph_database = runtime.get_graph_database();
    let datastore = graph_database.get_datastore();

    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let component_name = r_string();

    assert_eq!(0, datastore.get_vertex_count().unwrap());

    // Create entity type
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let component_ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let entity_type = EntityTypeBuilder::new(ty.clone())
        .component(component_ty)
        .string_property(property_name.clone())
        .build();
    let result = entity_type_manager.register(entity_type.clone());
    assert!(result.is_ok());

    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());
    let result = entity_vertex_manager.create(&ty, properties);

    assert!(result.is_ok());
    assert_eq!(1, datastore.get_vertex_count().unwrap());

    // Check if entity vertex with the given uuid exists
    let uuid = result.unwrap();
    assert!(entity_vertex_manager.has(uuid));

    let vertex = entity_vertex_manager.get(uuid);
    assert!(vertex.is_some());
    assert_eq!(uuid, vertex.unwrap().id);

    let properties = entity_vertex_manager.get_properties(uuid);
    assert!(properties.is_some());
    let properties = properties.unwrap();
    assert_eq!(uuid, properties.vertex.id);
    assert_eq!(&ty.type_id(), &properties.vertex.t);
    assert_eq!(1, properties.props.len());
    let property = properties.props.get(0);
    assert!(property.is_some());
    let property = property.unwrap();
    assert_eq!(property_name.clone(), property.name.to_string());
    assert_eq!(property_value.clone(), property.value);
    // Delete vertex
    assert!(entity_vertex_manager.delete(uuid));
    // Check if vertex is gone
    assert!(!entity_vertex_manager.has(uuid));
    let vertex = entity_vertex_manager.get(uuid);
    assert!(!vertex.is_some());
    let properties = entity_vertex_manager.get_properties(uuid);
    assert!(!properties.is_some());
}

#[test]
fn test_entity_vertex_manager_with_id() {
    let runtime = get_runtime();
    let entity_type_manager = runtime.get_entity_type_manager();
    let entity_vertex_manager = runtime.get_entity_vertex_manager();

    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let component_name = r_string();

    // Create entity type
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let component_ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let entity_type = EntityTypeBuilder::new(ty.clone())
        .component(component_ty.clone())
        .string_property(property_name.clone())
        .build();
    let result = entity_type_manager.register(entity_type.clone());
    assert!(result.is_ok());

    let vertex_uuid = Uuid::new_v4();
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());
    let result = entity_vertex_manager.create_with_id(&ty, vertex_uuid, properties);
    assert!(result.is_ok());

    // Check if entity vertex with the given uuid exists
    let uuid = result.unwrap();
    assert_eq!(vertex_uuid, uuid);
    assert!(entity_vertex_manager.has(vertex_uuid));

    let vertex = entity_vertex_manager.get(vertex_uuid);
    assert!(vertex.is_some());
    assert_eq!(vertex_uuid, vertex.unwrap().id);

    let properties = entity_vertex_manager.get_properties(vertex_uuid);
    assert!(properties.is_some());
    let properties = properties.unwrap();
    assert_eq!(vertex_uuid, properties.vertex.id);
    assert_eq!(&ty.type_id(), &properties.vertex.t);
    assert_eq!(1, properties.props.len());
    let property = properties.props.get(0);
    assert!(property.is_some());
    let property = property.unwrap();
    assert_eq!(property_name.clone(), property.name.to_string());
    assert_eq!(property_value.clone(), property.value);
    // Delete vertex
    assert!(entity_vertex_manager.delete(vertex_uuid));
    // Check if vertex is gone
    assert!(!entity_vertex_manager.has(vertex_uuid));
    let vertex = entity_vertex_manager.get(vertex_uuid);
    assert!(!vertex.is_some());
    let properties = entity_vertex_manager.get_properties(vertex_uuid);
    assert!(!properties.is_some());
}
