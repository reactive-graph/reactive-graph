use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use default_test::DefaultTest;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyType;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveSystem;
use reactive_graph_reactive_service_impl::ReactiveSystemImpl;
use serde_json::json;

// Do not remove! This import is necessary to make the dependency injection work
#[allow(unused_imports)]
use reactive_graph_behaviour_service_impl::BehaviourSystemImpl;
// Do not remove! This import is necessary to make the dependency injection work
#[allow(unused_imports)]
use reactive_graph_type_system_impl::TypeSystemImpl;

fn create_reactive_entity(criterion: &mut Criterion) {
    criterion.bench_function("create_reactive_entity", move |bencher| {
        let reactive_system = reactive_graph_di::get_container::<ReactiveSystemImpl>();
        let type_system = reactive_system.type_system();
        let entity_type_manager = type_system.get_entity_type_manager();
        let reactive_entity_manager = reactive_system.get_reactive_entity_manager();

        let entity_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register entity type");

        bencher.iter(move || {
            let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
            reactive_entity_manager
                .register_reactive_instance(reactive_entity)
                .expect("Failed to register reactive instance");
        })
    });
}

fn get_reactive_entity_by_id(criterion: &mut Criterion) {
    criterion.bench_function("get_reactive_entity_by_id", move |bencher| {
        let reactive_system = reactive_graph_di::get_container::<ReactiveSystemImpl>();
        let type_system = reactive_system.type_system();
        let entity_type_manager = type_system.get_entity_type_manager();
        let reactive_entity_manager = reactive_system.get_reactive_entity_manager();

        let entity_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register entity type");

        let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
        let id = reactive_entity.id;

        reactive_entity_manager
            .register_reactive_instance(reactive_entity)
            .expect("Failed to register reactive instance");

        bencher.iter(|| reactive_entity_manager.get(id))
    });
}

fn get_reactive_entity_by_label(criterion: &mut Criterion) {
    criterion.bench_function("get_reactive_entity_by_label", move |bencher| {
        let reactive_system = reactive_graph_di::get_container::<ReactiveSystemImpl>();
        let type_system = reactive_system.type_system();
        let entity_type_manager = type_system.get_entity_type_manager();
        let reactive_entity_manager = reactive_system.get_reactive_entity_manager();

        let entity_type = EntityType::default_test();
        entity_type.properties.push(PropertyType::string("label"));

        let entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type");

        let label = String::from("/io/reactive-graph/test");

        let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
        reactive_entity.set("label", json!(label.clone()));
        reactive_entity_manager
            .register_reactive_instance(reactive_entity)
            .expect("Failed to register reactive entity");

        bencher.iter(|| reactive_entity_manager.get_by_label(&label))
    });
}

criterion_group!(benches, create_reactive_entity, get_reactive_entity_by_id, get_reactive_entity_by_label);
criterion_main!(benches);
