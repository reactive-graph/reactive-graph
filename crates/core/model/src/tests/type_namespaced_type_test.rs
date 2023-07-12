use crate::tests::utils::r_string;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use schemars::schema_for;

#[test]
fn namespaced_type_from_str_test() {
    let namespace = r_string();
    let type_name = r_string();
    let nt = NamespacedType::new(&namespace, &type_name);
    assert_eq!(namespace, nt.namespace());
    assert_eq!(type_name, nt.type_name());
    assert_eq!(format!("{namespace}__{type_name}"), format!("{}", nt));
}

#[test]
fn namespaced_type_json_schema() {
    let schema = schema_for!(NamespacedType);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
