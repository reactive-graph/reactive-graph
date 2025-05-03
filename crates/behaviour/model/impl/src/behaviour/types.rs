#[macro_export]
macro_rules! behaviour_types {
    ($behaviour_types: ident, $namespace: ident $(, $behaviour_type_names:expr)*) => {
        pub static $behaviour_types: std::sync::LazyLock<Vec<reactive_graph_behaviour_model_api::BehaviourTypeId>> = std::sync::LazyLock::new(
            vec![
                $(
                reactive_graph_behaviour_model_api::BehaviourTypeId::new_from_type($namespace, $behaviour_type_names),
                )*
            ]
            .into_iter()
            .collect()
        );
    };
}
