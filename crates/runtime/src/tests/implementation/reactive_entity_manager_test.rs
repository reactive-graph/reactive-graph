// use default_test::DefaultTest;
//
// use crate::get_runtime;
// use crate::model::EntityType;
// use crate::model::EntityTypeId;
// use crate::model::PropertyTypes;
// use crate::reactive::ReactiveEntity;
// use crate::test_utils::r_string;
//
// #[test]
// fn test_register_reactive_entity_instance() {
//     let runtime = get_runtime();
//     let entity_type_manager = runtime.get_entity_type_manager();
//     let reactive_entity_manager = runtime.get_reactive_entity_manager();
//
//     let entity_type = EntityType::default_test();
//     let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
//
//     // Check that we cannot register an reactive entity with an entity type which doesn't exist
//     assert!(reactive_entity_manager.register_reactive_instance(reactive_entity.clone()).is_err());
//     // assert_eq!(ReactiveEntityRegistrationError::UnknownEntityType(entity_type.ty.clone()), result.unwrap_err(), "It shouldn't be allowed to register a reactive entity for a non-existent entity type!");
//     assert!(!reactive_entity_manager.has(reactive_entity.id), "There shouldn't be a reactive entity with the id");
//
//     // Register entity type
//     let entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type");
//     // Register the reactive entity
//     let reactive_entity = reactive_entity_manager
//         .register_reactive_instance(reactive_entity)
//         .expect("Failed to register the reactive entity");
//     // Register the reactive entity
//     assert!(reactive_entity_manager.has(reactive_entity.id), "The reactive entity with the id should be known by the reactive_entity_manager!");
//     // Get the reactive entity by id
//     let id = reactive_entity.id;
//     let reactive_entity = reactive_entity_manager.get(reactive_entity.id).expect("Failed to get the reactive entity by id!");
//     assert_eq!(id, reactive_entity.id, "The id of the reactive entity doesn't match!");
//     assert_eq!(entity_type.ty, reactive_entity.ty, "The entity type id doesn't match!");
// }
//
// #[test]
// fn test_unregister_reactive_entity_instance() {
//     let runtime = get_runtime();
//     let entity_type_manager = runtime.get_entity_type_manager();
//     let reactive_entity_manager = runtime.get_reactive_entity_manager();
//
//     let entity_type = EntityType::builder()
//         .ty(EntityTypeId::default_test())
//         .properties(PropertyTypes::new_with_string_property(r_string()))
//         .build();
//     // Register entity type
//     let entity_type = entity_type_manager
//         .register(entity_type)
//         .expect("Failed to register entity type");
//
//     let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
//     let id = reactive_entity.id;
//
//     // Register the reactive entity
//     let _reactive_entity = reactive_entity_manager
//         .register_reactive_instance(reactive_entity)
//         .expect("Failed to register the reactive entity");
//
//     assert!(reactive_entity_manager.has(id), "The reactive entity should be registered!");
//     assert!(reactive_entity_manager.unregister_reactive_instance(id), "The reactive entity should have been unregistered!");
//     assert!(!reactive_entity_manager.has(id), "The reactive entity shouldn't be registered anymore!");
// }
//
// #[test]
// fn test_not_register_twice_reactive_entity_instance() {
//     let runtime = get_runtime();
//     let entity_type_manager = runtime.get_entity_type_manager();
//     let reactive_entity_manager = runtime.get_reactive_entity_manager();
//
//     let entity_type = EntityType::builder()
//         .ty(EntityTypeId::default_test())
//         .properties(PropertyTypes::new_with_string_property(r_string()))
//         .build();
//
//     let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
//     let id = reactive_entity.id;
//
//     // Check that we cannot create an entity instance with a type which doesn't exist
//     assert!(reactive_entity_manager.register_reactive_instance(reactive_entity.clone()).is_err(), "The reactive entity shouldn't have been registered because the entity type was not registered!");
//
//     assert!(!reactive_entity_manager.has(id), "There shouldn't be a reactive entity with id!");
//     assert_eq!(reactive_entity_manager.count(), 0);
//
//     // Register entity type
//     let _entity_type = entity_type_manager
//         .register(entity_type)
//         .expect("Failed to register entity type");
//
//     let reactive_entity = reactive_entity_manager
//         .register_reactive_instance(reactive_entity)
//         .expect("Failed to register the reactive entity!");
//
//     assert!(reactive_entity_manager.has(id), "The reactive entity with id should be registered!");
//     assert_eq!(reactive_entity_manager.count(), 1);
//
//     assert!(reactive_entity_manager.register_reactive_instance(reactive_entity).is_err(), "The reactive entity was registered twice!");
//
//     assert!(reactive_entity_manager.has(id), "The reactive entity with id should be registered!");
//     assert_eq!(reactive_entity_manager.count(), 1);
// }
