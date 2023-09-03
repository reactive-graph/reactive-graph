// extern crate test;
//
// use std::process::Termination;
// use test::Bencher;
//
// use default_test::DefaultTest;
// use serde_json::json;
//
// use inexor_rgf_reactive::ReactiveEntity;
//
// use crate::get_runtime;
// use crate::model::PropertyType;
// use crate::model::PropertyInstanceSetter;
// use crate::model::EntityType;
//
// #[bench]
// fn creation_benchmark(bencher: &mut Bencher) -> impl Termination {
//     let runtime = get_runtime();
//     let entity_type_manager = runtime.get_entity_type_manager();
//     let reactive_entity_manager = runtime.get_reactive_entity_manager();
//
//     let entity_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register entity type");
//
//     bencher.iter(move || {
//         let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
//         reactive_entity_manager
//             .register_reactive_instance(reactive_entity)
//             .expect("Failed to register reactive instance");
//     })
// }
//
// #[bench]
// fn get_by_id_benchmark(bencher: &mut Bencher) -> impl Termination {
//     let runtime = get_runtime();
//     let entity_type_manager = runtime.get_entity_type_manager();
//     let reactive_entity_manager = runtime.get_reactive_entity_manager();
//
//     let entity_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register entity type");
//
//     let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
//     let id = reactive_entity.id;
//
//     reactive_entity_manager.register_reactive_instance(reactive_entity).expect("Failed to register reactive instance");
//
//     bencher.iter(|| reactive_entity_manager.get(id))
// }
//
// #[bench]
// fn get_by_label_benchmark(bencher: &mut Bencher) -> impl Termination {
//     let runtime = get_runtime();
//     let entity_type_manager = runtime.get_entity_type_manager();
//     let reactive_entity_manager = runtime.get_reactive_entity_manager();
//
//     let entity_type = EntityType::default_test();
//     entity_type.properties.push(PropertyType::string("label"));
//
//     let entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type");
//
//     let label = String::from("/org/inexor/test");
//
//     let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
//     reactive_entity.set("label", json!(label.clone()));
//     reactive_entity_manager.register_reactive_instance(reactive_entity).expect("Failed to register reactive entity");
//
//     bencher.iter(|| reactive_entity_manager.get_by_label(&label))
// }
