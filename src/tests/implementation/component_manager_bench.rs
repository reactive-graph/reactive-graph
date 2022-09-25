extern crate test;

use std::process::Termination;
use test::Bencher;

use crate::builder::ComponentBuilder;
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[bench]
fn creation_benchmark(bencher: &mut Bencher) -> impl Termination {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let component_name = r_string();
    let property_name = r_string();
    let component = ComponentBuilder::new(component_name.as_str()).string_property(property_name.as_str()).build();
    bencher.iter(move || {
        component_manager.register(component.clone());
        component_manager.delete(component_name.as_str());
    })
}
