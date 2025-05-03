#[macro_export]
macro_rules! behaviour_functions {
    (
        $collection_name: ident,
        $function_type: ty,
        $namespace: ident
        $(,
            (
                $type_name: expr,
                $function: ident
            )
        )*
    ) => {
        pub static $collection_name: std::sync::LazyLock<std::collections::HashMap<reactive_graph_behaviour_model_api::BehaviourTypeId, $function_type>> = std::sync::LazyLock(|| vec![
                $((reactive_graph_behaviour_model_api::BehaviourTypeId::new_from_type($namespace, $type_name), $function),)*
            ].into_iter().collect());
    };
}
