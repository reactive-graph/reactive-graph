use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use default_test::DefaultTest;
use reactive_graph_graph::Component;
use reactive_graph_type_system_api::TypeSystem;
use reactive_graph_type_system_impl::TypeSystemImpl;

fn create_components(criterion: &mut Criterion) {
    reactive_graph_test_utils::init_logger();
    criterion.bench_function("create_components", move |bencher| {
        let type_system = reactive_graph_di::get_container::<TypeSystemImpl>();
        let component_manager = type_system.get_component_manager();
        let component = Component::default_test();
        let ty = component.ty.clone();
        bencher.iter(move || {
            let _ = component_manager.register(component.clone());
            component_manager.delete(&ty);
        })
    });
}

criterion_group!(benches, create_components,);
criterion_main!(benches);
