#[macro_export]
macro_rules! behaviour_types {
    ($behaviour_types: ident, $namespace: expr $(, $behaviour_type_names:expr)*) => {
        lazy_static::lazy_static! {
            pub static ref $behaviour_types: Vec<inexor_rgf_core_model::BehaviourTypeId> = vec![
                $(
                inexor_rgf_core_model::BehaviourTypeId::new_from_type($namespace, $behaviour_type_names),
                )*
            ]
            .into_iter()
            .collect();
        }
    };
}
