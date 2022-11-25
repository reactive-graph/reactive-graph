#[macro_export]
macro_rules! behaviour_types {
    ($behaviour_types: ident, $namespace: expr $(, $behaviour_type_names:expr)*) => {
        lazy_static! {
            pub static ref $behaviour_types: Vec<BehaviourTypeId> = vec![
                $(
                BehaviourTypeId::new_from_type($namespace, $behaviour_type_names),
                )*
            ]
            .into_iter()
            .collect();
        }
    };
}
