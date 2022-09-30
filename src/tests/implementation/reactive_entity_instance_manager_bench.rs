extern crate test;

use std::process::Termination;
use test::Bencher;

use serde_json::json;

use crate::builder::EntityTypeBuilder;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::tests::utils::application::init_application;
use crate::tests::utils::{r_json_string, r_string};

#[bench]
fn creation_benchmark(bencher: &mut Bencher) -> impl Termination {
    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();

    let entity_type = EntityTypeBuilder::new(namespace.as_str(), type_name.as_str())
        .string_property(property_name.clone())
        .build();
    entity_type_manager.register(entity_type);

    bencher.iter(move || {
        reactive_entity_instance_manager.register_reactive_instance(
            ReactiveEntityInstanceBuilder::new(type_name.clone())
                .property(property_name.clone(), property_value.clone())
                .build(),
        );
    })
}

#[bench]
fn get_by_id_benchmark(bencher: &mut Bencher) -> impl Termination {
    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();

    let entity_type = EntityTypeBuilder::new(namespace.as_str(), type_name.as_str())
        .string_property(property_name.clone())
        .build();
    entity_type_manager.register(entity_type);

    let reactive_entity_instance = ReactiveEntityInstanceBuilder::new(type_name).property(property_name, property_value).build();
    let id = reactive_entity_instance.id;
    reactive_entity_instance_manager.register_reactive_instance(reactive_entity_instance);

    bencher.iter(|| reactive_entity_instance_manager.get(id))
}

#[bench]
fn get_by_label_benchmark(bencher: &mut Bencher) -> impl Termination {
    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();
    let label = String::from("/org/inexor/test");

    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();

    let entity_type = EntityTypeBuilder::new(namespace.as_str(), type_name.as_str())
        .string_property(property_name.clone())
        .string_property("label")
        .build();
    entity_type_manager.register(entity_type);

    let reactive_entity_instance = ReactiveEntityInstanceBuilder::new(type_name)
        .property(property_name, property_value)
        .property("label", json!(label.clone()))
        .build();
    reactive_entity_instance_manager.register_reactive_instance(reactive_entity_instance);

    bencher.iter(|| reactive_entity_instance_manager.get_by_label(&label))
}
