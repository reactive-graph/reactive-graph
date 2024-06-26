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
        lazy_static::lazy_static! {
            pub static ref $collection_name: std::collections::HashMap<reactive_graph_behaviour_model_api::BehaviourTypeId, $function_type> = vec![
                $((reactive_graph_behaviour_model_api::BehaviourTypeId::new_from_type($namespace, $type_name), $function),)*
            ].into_iter().collect();
        }
    };
}
