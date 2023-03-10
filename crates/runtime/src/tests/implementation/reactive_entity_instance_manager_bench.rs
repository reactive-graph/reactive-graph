extern crate test;

use std::process::Termination;
use test::Bencher;

use serde_json::json;

use crate::builder::EntityTypeBuilder;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::get_runtime;
use crate::model::EntityTypeId;
use crate::tests::utils::r_json_string;
use crate::tests::utils::r_string;

#[bench]
fn creation_benchmark(bencher: &mut Bencher) -> impl Termination {
    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let runtime = get_runtime();
    let entity_type_manager = runtime.get_entity_type_manager();
    let reactive_entity_instance_manager = runtime.get_reactive_entity_instance_manager();

    let entity_type = EntityTypeBuilder::new_from_type(&namespace, &type_name)
        .string_property(property_name.clone())
        .build();
    entity_type_manager
        .register(entity_type)
        .expect("Failed to register entity type {namespace} {type_name}");

    let ty = EntityTypeId::new_from_type(&namespace, &type_name);

    bencher.iter(move || {
        reactive_entity_instance_manager
            .register_reactive_instance(ReactiveEntityInstanceBuilder::new(&ty).property(&property_name, property_value.clone()).build())
            .expect("Failed to register reactive instance");
    })
}

#[bench]
fn get_by_id_benchmark(bencher: &mut Bencher) -> impl Termination {
    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let runtime = get_runtime();
    let entity_type_manager = runtime.get_entity_type_manager();
    let reactive_entity_instance_manager = runtime.get_reactive_entity_instance_manager();

    let entity_type = EntityTypeBuilder::new_from_type(&namespace, &type_name).string_property(&property_name).build();
    entity_type_manager
        .register(entity_type)
        .expect("Failed to register entity type {namespace} {type_name}");

    let ty = EntityTypeId::new_from_type(&namespace, &type_name);

    let reactive_entity_instance = ReactiveEntityInstanceBuilder::new(&ty).property(&property_name, property_value).build();
    let id = reactive_entity_instance.id;
    reactive_entity_instance_manager
        .register_reactive_instance(reactive_entity_instance)
        .expect("Failed to register reactive instance");

    bencher.iter(|| reactive_entity_instance_manager.get(id))
}

#[bench]
fn get_by_label_benchmark(bencher: &mut Bencher) -> impl Termination {
    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();
    let label = String::from("/org/inexor/test");

    let runtime = get_runtime();
    let entity_type_manager = runtime.get_entity_type_manager();
    let reactive_entity_instance_manager = runtime.get_reactive_entity_instance_manager();

    let entity_type = EntityTypeBuilder::new_from_type(&namespace, &type_name)
        .string_property(&property_name)
        .string_property("label")
        .build();
    entity_type_manager
        .register(entity_type)
        .expect("Failed to register entity type {namespace} {type_name}");

    let ty = EntityTypeId::new_from_type(&namespace, &type_name);

    let reactive_entity_instance = ReactiveEntityInstanceBuilder::new(&ty)
        .property(&property_name, property_value)
        .property("label", json!(label.clone()))
        .build();
    reactive_entity_instance_manager
        .register_reactive_instance(reactive_entity_instance)
        .expect("Failed to register reactive instance");

    bencher.iter(|| reactive_entity_instance_manager.get_by_label(&label))
}
