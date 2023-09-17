// use std::env;
//
// use uuid::Uuid;
// use inexor_rgf_graph::RelationInstanceId;
//
// use crate::get_runtime;
// use crate::model::ComponentOrEntityTypeId;
// use crate::model::EntityTypeId;
// use crate::model::NamespacedTypeGetter;
// use crate::model::RelationInstanceTypeId;
// use crate::model::RelationTypeId;
// use crate::test_utils::r_json_string;
// use crate::test_utils::r_string;
//
// #[test]
// fn test_relation_instance_manager() {
//     let runtime = get_runtime();
//     let entity_type_manager = runtime.get_entity_type_manager();
//     let entity_instance_manager = runtime.get_entity_instance_manager();
//     let relation_type_manager = runtime.get_relation_type_manager();
//     let relation_instance_manager = runtime.get_relation_instance_manager();
//
//     let namespace = r_string();
//     let outbound_type_name = r_string();
//     let outbound_id = Uuid::new_v4();
//     let type_name = r_string();
//     let inbound_type_name = r_string();
//     let inbound_id = Uuid::new_v4();
//     let property_name = r_string();
//     let property_value = r_json_string();
//
//     let relation_ty = RelationTypeId::new_from_type(&namespace, &type_name);
//     let ty = RelationInstanceTypeId::new_unique_id(&relation_ty);
//     // let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);
//
//     let outbound_type = EntityTypeBuilder::new_from_type(&namespace, &outbound_type_name)
//         .string_property(&property_name)
//         .build();
//     let result = entity_type_manager.register(outbound_type);
//     assert!(result.is_ok());
//     let inbound_type = EntityTypeBuilder::new_from_type(&namespace, &inbound_type_name)
//         .string_property(&property_name)
//         .build();
//     let result = entity_type_manager.register(inbound_type);
//     assert!(result.is_ok());
//
//     let outbound_instance = EntityInstanceBuilder::new_from_type(&namespace, &outbound_type_name)
//         .id(outbound_id)
//         .property(&property_name, property_value.clone())
//         .build();
//     let result = entity_instance_manager.create_from_instance(outbound_instance);
//     assert!(result.is_ok());
//
//     let inbound_instance = EntityInstanceBuilder::new_from_type(&namespace, &inbound_type_name)
//         .id(inbound_id)
//         .property(&property_name, property_value.clone())
//         .build();
//     let result = entity_instance_manager.create_from_instance(inbound_instance);
//     assert!(result.is_ok());
//
//     // Check that we cannot create an relation instance with a type which is not registered
//     let relation_instance = RelationInstanceBuilder::new(outbound_id, &ty, inbound_id)
//         .property(&property_name, property_value.clone())
//         .build();
//     let result = relation_instance_manager.create_from_instance(relation_instance);
//     assert!(result.is_err());
//
//     // Now: create the relation type
//     let outbound_ty = ComponentOrEntityTypeId::EntityType(EntityTypeId::new_from_type(&namespace, &outbound_type_name));
//     let inbound_ty = ComponentOrEntityTypeId::EntityType(EntityTypeId::new_from_type(&namespace, &inbound_type_name));
//     let relation_type = RelationTypeBuilder::new(&outbound_ty, &relation_ty, &inbound_ty)
//         .string_property(&property_name)
//         .build();
//     let result = relation_type_manager.register(relation_type);
//     assert!(result.is_ok());
//     let relation_type_2 = result.unwrap();
//     assert_eq!(&namespace, &relation_type_2.namespace());
//     assert_eq!(&type_name, &relation_type_2.type_name());
//     assert_eq!(&outbound_type_name, &relation_type_2.outbound_type.type_name());
//     assert_eq!(&inbound_type_name, &relation_type_2.inbound_type.type_name());
//
//     // Check that we cannot create a relation instance with a non-existent outbound
//     let relation_instance = RelationInstanceBuilder::new(Uuid::new_v4(), &ty, inbound_id)
//         .property(&property_name, property_value.clone())
//         .build();
//     let result = relation_instance_manager.create_from_instance(relation_instance);
//     assert!(result.is_err());
//
//     // Check that we cannot create a relation instance with a non-existent inbound
//     let relation_instance = RelationInstanceBuilder::new(outbound_id, &ty, Uuid::new_v4())
//         .property(&property_name, property_value.clone())
//         .build();
//     let result = relation_instance_manager.create_from_instance(relation_instance);
//     assert!(result.is_err());
//
//     // Check that we can create a relation instance with existent inbound and outbound
//     let relation_instance = RelationInstanceBuilder::new(outbound_id, &ty, inbound_id)
//         .property(&property_name, property_value.clone())
//         .build();
//     assert_eq!(ty, relation_instance.id());
//     let result = relation_instance_manager.create_from_instance(relation_instance);
//     if result.is_err() {
//         println!("{:?}", result.as_ref().err());
//         assert!(false);
//     }
//     let actual_edge_key = result.unwrap();
//     assert_eq!(outbound_id, actual_edge_key.outbound_id);
//     assert_eq!(format!("r__{}__{}", &namespace, &type_name), actual_edge_key.t.to_string());
//     assert_eq!(inbound_id, actual_edge_key.inbound_id);
//
//     // Check if has returns false for a non-existent uuid
//     // let wrong_outbound_id_edge_key = EdgeKey::new(Uuid::new_v4(), ty.type_id(), inbound_id);
//     let wrong_outbound_id = RelationInstanceId::new(Uuid::new_v4(), relation_ty.clone(), inbound_id);
//     // let wrong_inbound_id_edge_key = EdgeKey::new(outbound_id, ty.type_id(), Uuid::new_v4());
//     let wrong_inbound_id = RelationInstanceId::new(outbound_id, relation_ty.clone(), Uuid::new_v4());
//
//     let correct_id = RelationInstanceId::new(outbound_id, relation_ty.clone(), inbound_id);
//
//     assert!(!relation_instance_manager.has(&wrong_outbound_id));
//     assert!(!relation_instance_manager.has(&wrong_inbound_id));
//
//     // Check if has returns true for the created relation
//     assert!(relation_instance_manager.has(&correct_id));
//
//     // Check if get returns none for a non-existent uuid
//     assert!(relation_instance_manager.get(&wrong_outbound_id).is_none());
//     assert!(relation_instance_manager.get(&wrong_inbound_id).is_none());
//
//     // Check if get returns the created relation
//     let relation_instance = relation_instance_manager.get(&ty);
//     assert!(relation_instance.is_some());
//     let relation_instance = relation_instance.unwrap();
//     assert_eq!(outbound_id, relation_instance.outbound_id);
//     assert_eq!(inbound_id, relation_instance.inbound_id);
//     assert_eq!(type_name.clone(), relation_instance.type_name());
//
//     // Check that we cannot create the same relation instance twice
//     let relation_instance = RelationInstanceBuilder::new(outbound_id, &ty, inbound_id)
//         .property(&property_name, property_value.clone())
//         .build();
//     let result = relation_instance_manager.create_from_instance(relation_instance.clone());
//     assert!(result.is_err());
//     // TODO: Replace with TypedBuilder
//     let relation_instance = RelationInstanceBuilder::try_from(&edge_key)
//         .unwrap()
//         .property(&property_name, property_value.clone())
//         .build();
//     let result = relation_instance_manager.create_from_instance(relation_instance.clone());
//     assert!(result.is_err());
//
//     relation_instance_manager.delete(&edge_key);
//     assert!(!relation_instance_manager.has(&edge_key));
//     assert!(relation_instance_manager.get(&edge_key).is_none());
// }
//
// // #[test]
// // fn test_relation_instance_manager_import_export() {
// //     let runtime = get_runtime();
// //     let entity_type_manager = runtime.get_entity_type_manager();
// //     let entity_instance_manager = runtime.get_entity_instance_manager();
// //     let relation_type_manager = runtime.get_relation_type_manager();
// //     let relation_instance_manager = runtime.get_relation_instance_manager();
// //
// //     let namespace = r_string();
// //     let outbound_type_name = r_string();
// //     let outbound_id = Uuid::new_v4();
// //     let type_name = r_string();
// //     let inbound_type_name = r_string();
// //     let inbound_id = Uuid::new_v4();
// //     let property_name = r_string();
// //     let property_value = r_json_string();
// //
// //     let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
// //     let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);
// //
// //     let outbound_type = EntityTypeBuilder::new_from_type(&namespace, &outbound_type_name)
// //         .string_property(&property_name)
// //         .build();
// //     let result = entity_type_manager.register(outbound_type.clone());
// //     assert!(result.is_ok());
// //
// //     let inbound_type = EntityTypeBuilder::new_from_type(&namespace, &inbound_type_name)
// //         .string_property(&property_name)
// //         .build();
// //     let result = entity_type_manager.register(inbound_type.clone());
// //     assert!(result.is_ok());
// //
// //     let entity_instance = EntityInstanceBuilder::new_from_type(&namespace, &outbound_type_name)
// //         .id(outbound_id)
// //         .property(&property_name, property_value.clone())
// //         .build();
// //     let result = entity_instance_manager.create_from_instance(entity_instance);
// //     assert!(result.is_ok());
// //
// //     let entity_instance = EntityInstanceBuilder::new_from_type(&namespace, &inbound_type_name)
// //         .id(inbound_id)
// //         .property(&property_name, property_value.clone())
// //         .build();
// //     let result = entity_instance_manager.create_from_instance(entity_instance);
// //     assert!(result.is_ok());
// //
// //     let mut path = env::temp_dir();
// //     path.push(format!("{}-{}-{}.json", outbound_id.to_string().as_str(), type_name.clone(), inbound_id.to_string().as_str()));
// //     let path = path.into_os_string().into_string().unwrap();
// //
// //     let outbound_ty: ComponentOrEntityTypeId = outbound_type.ty.into();
// //     let relation_ty = ty.relation_type_id();
// //     let inbound_ty: ComponentOrEntityTypeId = inbound_type.ty.into();
// //
// //     let relation_type = RelationTypeBuilder::new(&outbound_ty, &relation_ty, &inbound_ty)
// //         .string_property(&property_name)
// //         .build();
// //     let result = relation_type_manager.register(relation_type);
// //     assert!(result.is_ok());
// //
// //     let relation_instance = RelationInstanceBuilder::new(outbound_id, &ty, inbound_id)
// //         .property(&property_name, property_value.clone())
// //         .build();
// //     let result = relation_instance_manager.create_from_instance(relation_instance);
// //
// //     let actual_edge_key = result.unwrap();
// //     assert_eq!(outbound_id, actual_edge_key.outbound_id);
// //     assert_eq!(format!("r__{}__{}", &namespace, &type_name), actual_edge_key.t.to_string());
// //     assert_eq!(inbound_id, actual_edge_key.inbound_id);
// //
// //     relation_instance_manager.export(&edge_key, &path);
// //     assert!(relation_instance_manager.has(&edge_key));
// //     relation_instance_manager.delete(&edge_key);
// //     assert!(!relation_instance_manager.has(&edge_key));
// //     let result = relation_instance_manager.import(&path);
// //     assert!(result.is_ok());
// //     let relation_instance = result.unwrap();
// //     assert_eq!(outbound_id, relation_instance.outbound_id);
// //     assert_eq!(&type_name, &relation_instance.type_name());
// //     assert_eq!(inbound_id, relation_instance.inbound_id);
// //     assert!(relation_instance_manager.has(&edge_key));
// // }
