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
            pub static ref $collection_name: std::collections::HashMap<inexor_rgf_core_model::BehaviourTypeId, $function_type> = vec![
                $((inexor_rgf_core_model::BehaviourTypeId::new_from_type($namespace, $type_name), $function),)*
            ].into_iter().collect();
        }
    };
}
