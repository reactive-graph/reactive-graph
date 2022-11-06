extern crate test;

use std::process::Termination;
use test::Bencher;

use crate::builder::EntityTypeBuilder;
use crate::model::EntityTypeId;
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[bench]
fn creation_benchmark(bencher: &mut Bencher) -> impl Termination {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let entity_type = EntityTypeBuilder::new(&ty).string_property(property_name.as_str()).build();
    bencher.iter(move || {
        let _ = entity_type_manager.register(entity_type.clone());
        entity_type_manager.delete(&ty);
    })
}
