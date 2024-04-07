#[macro_export]
macro_rules! behaviour_types {
    ($behaviour_types: ident, $namespace: ident $(, $behaviour_type_names:expr)*) => {
        lazy_static::lazy_static! {
            pub static ref $behaviour_types: Vec<reactive_graph_behaviour_model_api::BehaviourTypeId> = vec![
                $(
                reactive_graph_behaviour_model_api::BehaviourTypeId::new_from_type($namespace, $behaviour_type_names),
                )*
            ]
            .into_iter()
            .collect();
        }
    };
}
