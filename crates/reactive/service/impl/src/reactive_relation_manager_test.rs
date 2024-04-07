// // TODO: fix these unit test
//
// use indradb::{EdgeKey, Type};
//
// use crate::builder::{EntityTypeBuilder, ReactiveEntityBuilder, ReactiveRelationBuilder, RelationTypeBuilder};
// use reactive_graph_graph::PropertyInstanceSetter;
// use crate::tests::r_json_string;
// use crate::tests::utils::{init_application, r_string};
// use std::env;
// use uuid::Uuid;
//
// #[test]
// fn test_reactive_relation_manager() {
//     let application = init_application();
//     let entity_type_manager = application.get_entity_type_manager();
//     let reactive_entity_manager = application.get_reactive_entity_manager();
//     let relation_type_manager = application.get_relation_type_manager();
//     let relation_instance_manager = application.get_relation_instance_manager();
//     let reactive_relation_manager = application.get_reactive_relation_manager();
//
//     let outbound_type = r_string();
//     let outbound_id = Uuid::new_v4();
//     let type_name = r_string();
//     let inbound_type = r_string();
//     let inbound_id = Uuid::new_v4();
//     let property_name = r_string();
//     let property_value = r_json_string();
//
//     let edge_key = EdgeKey::new(outbound_id, Type::new(type_name.clone()).unwrap(), inbound_id);
//
//     EntityTypeBuilder::new(outbound_type.clone())
//         .string_property(property_name.clone())
//         .register(entity_type_manager.clone());
//     EntityTypeBuilder::new(inbound_type.clone())
//         .string_property(property_name.clone())
//         .register(entity_type_manager.clone());
//     RelationTypeBuilder::new(outbound_type.clone(), type_name.clone(), inbound_type.clone())
//         .string_property(property_name.clone())
//         .register(relation_type_manager.clone());
//
//     let result = ReactiveEntityBuilder::new(outbound_type.clone())
//         .id(outbound_id)
//         .property(property_name.clone(), property_value.clone())
//         .create(reactive_entity_manager.clone());
//     assert!(result.is_ok());
//     assert!(reactive_entity_manager.has(outbound_id));
//
//     let result = ReactiveEntityBuilder::new(inbound_type.clone())
//         .id(inbound_id)
//         .property(property_name.clone(), property_value.clone())
//         .create(reactive_entity_manager.clone());
//     assert!(result.is_ok());
//     assert!(reactive_entity_manager.has(inbound_id));
//
//     assert!(!reactive_relation_manager.has(edge_key.clone()));
//     assert!(reactive_relation_manager.get(edge_key.clone()).is_none());
//     let result = ReactiveRelationBuilder::new(outbound_id, type_name.clone(), inbound_id)
//         .property(property_name.clone(), property_value.clone())
//         .create(reactive_relation_manager.clone());
//     assert!(result.is_ok());
//     assert!(reactive_relation_manager.has(edge_key.clone()));
//     assert!(reactive_relation_manager.get(edge_key.clone()).is_some());
//
//     let reactive_relation_instance = result.unwrap();
//     let actual_edge_key = reactive_relation_instance.get_key();
//     assert!(actual_edge_key.is_some());
//     let actual_edge_key = actual_edge_key.unwrap();
//     assert_eq!(edge_key, actual_edge_key);
//     assert_eq!(type_name.clone(), reactive_relation_instance.type_name.clone());
//     assert_eq!(outbound_id, reactive_relation_instance.outbound.id);
//     assert_eq!(outbound_type.clone(), reactive_relation_instance.outbound.type_name.clone());
//     assert_eq!(inbound_id, reactive_relation_instance.inbound.id);
//     assert_eq!(inbound_type.clone(), reactive_relation_instance.inbound.type_name.clone());
//
//     let property_value_new = r_json_string();
//     reactive_relation_instance.set(property_name.clone(), property_value_new.clone());
//     reactive_relation_manager.commit(edge_key.clone());
//
//     let relation_instance = relation_instance_manager.get(edge_key.clone());
//     assert!(relation_instance.is_some());
//     let relation_instance = relation_instance.unwrap();
//     let actual_property_value = relation_instance.properties.get(property_name.clone().as_str());
//     assert!(actual_property_value.is_some());
//     let actual_property_value = actual_property_value.unwrap();
//     assert_eq!(property_value_new, *actual_property_value);
//
//     reactive_relation_manager.delete(edge_key.clone());
//     assert!(!reactive_relation_manager.has(edge_key.clone()));
//     assert!(reactive_relation_manager.get(edge_key.clone()).is_none());
//     assert!(!relation_instance_manager.has(edge_key.clone()));
//     assert!(relation_instance_manager.get(edge_key.clone()).is_none());
// }
//
// #[test]
// fn test_reactive_relation_manager_import_export() {
//     let application = init_application();
//     let entity_type_manager = application.get_entity_type_manager();
//     let reactive_entity_manager = application.get_reactive_entity_manager();
//     let relation_type_manager = application.get_relation_type_manager();
//     let relation_instance_manager = application.get_relation_instance_manager();
//     let reactive_relation_manager = application.get_reactive_relation_manager();
//
//     let outbound_type = r_string();
//     let outbound_id = Uuid::new_v4();
//     let type_name = r_string();
//     let inbound_type = r_string();
//     let inbound_id = Uuid::new_v4();
//     let property_name = r_string();
//     let property_value = r_json_string();
//
//     let edge_key = EdgeKey::new(outbound_id, Type::new(type_name.clone()).unwrap(), inbound_id);
//
//     EntityTypeBuilder::new(outbound_type.clone())
//         .string_property(property_name.clone())
//         .register(entity_type_manager.clone());
//     EntityTypeBuilder::new(inbound_type.clone())
//         .string_property(property_name.clone())
//         .register(entity_type_manager.clone());
//     RelationTypeBuilder::new(outbound_type.clone(), type_name.clone(), inbound_type.clone())
//         .string_property(property_name.clone())
//         .register(relation_type_manager.clone());
//
//     let result = ReactiveEntityBuilder::new(outbound_type.clone())
//         .id(outbound_id)
//         .property(property_name.clone(), property_value.clone())
//         .create(reactive_entity_manager.clone());
//     assert!(result.is_ok());
//
//     let result = ReactiveEntityBuilder::new(inbound_type.clone())
//         .id(inbound_id)
//         .property(property_name.clone(), property_value.clone())
//         .create(reactive_entity_manager.clone());
//     assert!(result.is_ok());
//
//     let result = ReactiveRelationBuilder::new(outbound_id, type_name.clone(), inbound_id)
//         .property(property_name.clone(), property_value.clone())
//         .create(reactive_relation_manager.clone());
//     assert!(result.is_ok());
//
//     let reactive_relation_instance = result.unwrap();
//     assert_eq!(edge_key, reactive_relation_instance.get_key().unwrap());
//
//     let mut path = env::temp_dir();
//     path.push(format!("{}.json", type_name));
//     let path = path.into_os_string().into_string().unwrap();
//
//     reactive_relation_manager.export(edge_key.clone(), path.clone());
//     assert!(reactive_relation_manager.has(edge_key.clone()));
//     assert!(relation_instance_manager.has(edge_key.clone()));
//
//     relation_instance_manager.delete(edge_key.clone());
//     assert!(!reactive_relation_manager.has(edge_key.clone()));
//     assert!(!relation_instance_manager.has(edge_key.clone()));
//
//     let result = reactive_relation_manager.import(path.clone());
//     assert!(result.is_ok());
//     let reactive_relation_instance = result.unwrap();
//     assert_eq!(outbound_id, reactive_relation_instance.outbound.id);
//     assert_eq!(type_name.clone(), reactive_relation_instance.type_name.clone());
//     assert_eq!(inbound_id, reactive_relation_instance.inbound.id);
//     assert!(reactive_relation_manager.has(edge_key.clone()));
//     assert!(relation_instance_manager.has(edge_key.clone()));
// }
