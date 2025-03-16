use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use default_test::DefaultTest;
use reactive_graph_graph::EntityType;
use reactive_graph_type_system_api::TypeSystem;
use reactive_graph_type_system_impl::TypeSystemImpl;

fn create_entity_type(criterion: &mut Criterion) {
    reactive_graph_test_utils::init_logger();
    criterion.bench_function("create_reactive_entity", move |bencher| {
        let type_system = reactive_graph_di::get_container::<TypeSystemImpl>();
        let entity_type_manager = type_system.get_entity_type_manager();
        let entity_type = EntityType::default_test();
        let ty = entity_type.ty.clone();
        bencher.iter(move || {
            let _ = entity_type_manager.register(entity_type.clone());
            let _ = entity_type_manager.delete(&ty);
        })
    });
}

criterion_group!(benches, create_entity_type);
criterion_main!(benches);
