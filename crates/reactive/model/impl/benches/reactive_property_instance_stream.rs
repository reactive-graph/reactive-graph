use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use rand::Rng;
use reactive_graph_graph::Mutability::Mutable;
use reactive_graph_reactive_model_impl::ReactiveProperty;
use reactive_graph_test_utils::r_string;
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use uuid::Uuid;

fn reactive_property_instance_stream(criterion: &mut Criterion) {
    criterion.bench_function("reactive_property_instance_stream", move |bencher| {
        let instance1 = ReactiveProperty::new(Uuid::new_v4(), r_string(), Mutable, json!(0));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), r_string(), Mutable, json!(0));

        let v = Arc::new(AtomicU64::new(0));

        {
            let v = v.clone();
            let writer = instance2.stream.write().unwrap();
            let handle_id = Uuid::new_v4().as_u128();
            writer.observe_with_handle(
                move |value| {
                    v.store(value.as_u64().unwrap(), Ordering::Relaxed);
                },
                handle_id,
            );
        }

        {
            let writer = instance1.stream.write().unwrap();
            let handle_id = Uuid::new_v4().as_u128();
            writer.observe_with_handle(
                move |value| {
                    instance2.set(value.clone());
                },
                handle_id,
            );
        }

        let mut rng = rand::rng();
        bencher.iter(move || {
            let number: u64 = rng.random();
            instance1.set(json!(number));
            assert_eq!(number, v.load(Ordering::Relaxed));
        })
    });
}

criterion_group!(benches, reactive_property_instance_stream,);
criterion_main!(benches);
