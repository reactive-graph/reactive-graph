extern crate test;

use crate::builder::EntityTypeBuilder;
use std::process::Termination;
use test::Bencher;

use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[bench]
fn creation_benchmark(bencher: &mut Bencher) -> impl Termination {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let entity_type = EntityTypeBuilder::new(namespace.as_str(), type_name.as_str())
        .string_property(property_name.as_str())
        .build();
    bencher.iter(move || {
        entity_type_manager.register(entity_type.clone());
        entity_type_manager.delete(type_name.as_str());
    })
}
