use indradb::Identifier;
use schemars::schema_for;

use crate::tests::utils::r_string;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeDefinition;
use crate::TypeIdType;

#[test]
fn type_definition_component_test() {
    let namespace = r_string();
    let type_name = r_string();
    let nt = NamespacedType::new(&namespace, &type_name);
    let td = TypeDefinition::new(TypeIdType::Component, nt.clone());
    assert_eq!(TypeIdType::Component, td.type_id_type);
    assert_eq!(namespace, td.namespace());
    assert_eq!(type_name, td.type_name());
    assert_eq!(format!("c__{namespace}__{type_name}"), td.to_string());

    let tid: TypeIdType = TypeIdType::from(&td);
    assert_eq!(TypeIdType::Component, tid);

    let nt2: NamespacedType = NamespacedType::from(&td);
    assert_eq!(nt, nt2);

    let t = Identifier::from(&td);
    assert_eq!(format!("c__{namespace}__{type_name}"), t.as_str());

    let td2 = TypeDefinition::try_from(&t).unwrap();
    assert_eq!(TypeIdType::Component, td2.type_id_type);
    assert_eq!(namespace, td2.namespace());
    assert_eq!(type_name, td2.type_name());
    assert_eq!(format!("c__{namespace}__{type_name}"), td2.to_string());
    assert_eq!(td, td2);
}

#[test]
fn type_definition_component_2_test() {
    let namespace = r_string();
    let type_name = r_string();
    let td = TypeDefinition::new_from_type(TypeIdType::Component, &namespace, &type_name);
    assert_eq!(TypeIdType::Component, td.type_id_type);
    assert_eq!(namespace, td.namespace());
    assert_eq!(type_name, td.type_name());
    assert_eq!(format!("c__{namespace}__{type_name}"), td.to_string());
}

#[test]
fn type_definition_component_3_test() {
    let namespace = r_string();
    let type_name = r_string();
    let td = TypeDefinition::component(&namespace, &type_name);
    assert_eq!(TypeIdType::Component, td.type_id_type);
    assert_eq!(namespace, td.namespace());
    assert_eq!(type_name, td.type_name());
    assert_eq!(format!("c__{namespace}__{type_name}"), td.to_string());
}

#[test]
fn type_definition_entity_type_test() {
    let namespace = r_string();
    let type_name = r_string();
    let td = TypeDefinition::entity_type(&namespace, &type_name);
    assert_eq!(TypeIdType::EntityType, td.type_id_type);
    assert_eq!(namespace, td.namespace());
    assert_eq!(type_name, td.type_name());
    assert_eq!(format!("e__{namespace}__{type_name}"), td.to_string());

    let tid: TypeIdType = TypeIdType::from(&td);
    assert_eq!(TypeIdType::EntityType, tid);
}

#[test]
fn type_definition_relation_type_test() {
    let namespace = r_string();
    let type_name = r_string();
    let td = TypeDefinition::relation_type(&namespace, &type_name);
    assert_eq!(TypeIdType::RelationType, td.type_id_type);
    assert_eq!(namespace, td.namespace());
    assert_eq!(type_name, td.type_name());
    assert_eq!(format!("r__{namespace}__{type_name}"), td.to_string());

    let tid: TypeIdType = TypeIdType::from(&td);
    assert_eq!(TypeIdType::RelationType, tid);
}

#[test]
fn type_definition_flow_type_test() {
    let namespace = r_string();
    let type_name = r_string();
    let td = TypeDefinition::flow_type(&namespace, &type_name);
    assert_eq!(TypeIdType::FlowType, td.type_id_type);
    assert_eq!(namespace, td.namespace());
    assert_eq!(type_name, td.type_name());
    assert_eq!(format!("f__{namespace}__{type_name}"), td.to_string());

    let tid: TypeIdType = TypeIdType::from(&td);
    assert_eq!(TypeIdType::FlowType, tid);
}

#[test]
fn type_definition_component_from_identifier_test() {
    let namespace = r_string();
    let type_name = r_string();
    let t = Identifier::new(format!("c__{namespace}__{type_name}")).unwrap();
    let td = TypeDefinition::try_from(&t).unwrap();
    assert_eq!(TypeIdType::Component, td.type_id_type);
    assert_eq!(namespace, td.namespace());
    assert_eq!(type_name, td.type_name());
    assert_eq!(format!("c__{namespace}__{type_name}"), td.to_string());
}

#[test]
fn type_definition_json_schema() {
    let schema = schema_for!(TypeDefinition);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
