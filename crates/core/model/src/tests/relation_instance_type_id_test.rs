use crate::tests::utils::r_string;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::RelationInstanceTypeId;
use crate::RelationTypeId;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use indradb::Identifier;
use schemars::schema_for;

#[test]
fn relation_instance_type_id_unique_id_test() {
    let namespace = r_string();
    let type_name = r_string();

    let nt = NamespacedType::new(&namespace, &type_name);
    let rty = RelationTypeId::new_from_type(&namespace, &type_name);
    let ty = RelationInstanceTypeId::new_unique_id(rty.clone());
    assert_eq!(namespace, ty.namespace());
    assert_eq!(nt.namespace, ty.namespace());
    assert_eq!(nt.type_name, ty.type_name());
    assert_eq!(format!("r__{namespace}__{type_name}"), format!("{}", ty));
    let type_definition = ty.type_definition();
    assert_eq!(TypeIdType::RelationType, type_definition.type_id_type);
    assert_eq!(namespace, type_definition.namespace());
    assert_eq!(type_name, type_definition.type_name());

    let type_definition_3 = TypeDefinition::from(&ty);
    assert_eq!(TypeIdType::RelationType, type_definition_3.type_id_type);
    assert_eq!(namespace, type_definition_3.namespace());
    assert_eq!(type_name, type_definition_3.type_name());

    let ty2 = RelationInstanceTypeId::new_unique_id(rty.clone());
    assert_eq!(ty, ty2);
    assert_eq!(ty.namespace(), ty2.namespace());
    assert_eq!(ty.type_name(), ty2.type_name());
    assert_eq!(ty.instance_id(), ty2.instance_id());
    assert_eq!(ty.to_string(), ty2.to_string());
}

#[test]
fn relation_instance_type_id_unique_for_instance_id_test() {
    let namespace = r_string();
    let type_name = r_string();
    let instance_id = r_string();

    let nt = NamespacedType::new(&namespace, &type_name);
    let rty = RelationTypeId::new_from_type(&namespace, &type_name);
    let ty = RelationInstanceTypeId::new_unique_for_instance_id(rty.clone(), &instance_id);
    assert_eq!(namespace, ty.namespace());
    assert_eq!(nt.namespace, ty.namespace());
    assert_eq!(format!("{}__{}", type_name, instance_id), ty.type_name());
    assert_eq!(instance_id, ty.instance_id());
    assert_eq!(rty, ty.relation_type_id());
    assert_eq!(format!("r__{namespace}__{type_name}__{instance_id}"), format!("{}", ty));
    let type_definition = ty.type_definition();
    assert_eq!(TypeIdType::RelationType, type_definition.type_id_type);
    assert_eq!(namespace, type_definition.namespace());
    assert_eq!(format!("{}__{}", type_name, instance_id), type_definition.type_name());

    let type_definition_3 = TypeDefinition::from(&ty);
    assert_eq!(TypeIdType::RelationType, type_definition_3.type_id_type);
    assert_eq!(namespace, type_definition_3.namespace());
    assert_eq!(format!("{}__{}", type_name, instance_id), type_definition_3.type_name());

    let instance_id_2 = r_string();
    let ty2 = RelationInstanceTypeId::new_unique_for_instance_id(rty.clone(), &instance_id_2);
    assert_eq!(namespace, ty2.namespace());
    assert_eq!(nt.namespace, ty2.namespace());
    assert_eq!(format!("{}__{}", type_name, instance_id_2), ty2.type_name());
    assert_eq!(instance_id_2, ty2.instance_id());
    assert_eq!(rty, ty2.relation_type_id());
    assert_eq!(format!("r__{namespace}__{type_name}__{instance_id_2}"), format!("{}", ty2));
    assert_ne!(ty, ty2);
    assert_eq!(ty.namespace(), ty2.namespace());
    assert_ne!(ty.type_name(), ty2.type_name());
    assert_eq!(ty.relation_type_id(), ty2.relation_type_id());
    assert_ne!(ty.instance_id(), ty2.instance_id());
    assert_ne!(ty.to_string(), ty2.to_string());
}

#[test]
fn relation_instance_type_id_with_random_instance_id_test() {
    let namespace = r_string();
    let type_name = r_string();

    let nt = NamespacedType::new(&namespace, &type_name);
    let rty = RelationTypeId::new_from_type(&namespace, &type_name);
    let ty = RelationInstanceTypeId::new_with_random_instance_id(rty.clone());
    assert_eq!(namespace, ty.namespace());
    assert_eq!(nt.namespace, ty.namespace());
    assert_eq!(format!("{}__{}", type_name, ty.instance_id()), ty.type_name());
    assert_eq!(rty, ty.relation_type_id());
    assert_eq!(format!("r__{namespace}__{type_name}__{}", ty.instance_id()), format!("{}", ty));
    let type_definition = ty.type_definition();
    assert_eq!(TypeIdType::RelationType, type_definition.type_id_type);
    assert_eq!(namespace, type_definition.namespace());
    assert_eq!(format!("{}__{}", type_name, ty.instance_id()), type_definition.type_name());

    let type_definition_3 = TypeDefinition::from(&ty);
    assert_eq!(TypeIdType::RelationType, type_definition_3.type_id_type);
    assert_eq!(namespace, type_definition_3.namespace());
    assert_eq!(format!("{}__{}", type_name, ty.instance_id()), type_definition_3.type_name());

    let ty2 = RelationInstanceTypeId::new_with_random_instance_id(rty.clone());
    assert_eq!(namespace, ty2.namespace());
    assert_eq!(nt.namespace, ty2.namespace());
    assert_eq!(format!("{}__{}", type_name, ty2.instance_id()), ty2.type_name());
    assert_ne!(ty.instance_id(), ty2.instance_id());
    assert_eq!(rty, ty2.relation_type_id());
    assert_eq!(format!("r__{namespace}__{type_name}__{}", ty2.instance_id()), format!("{}", ty2));
    assert_ne!(ty, ty2);
    assert_eq!(ty.namespace(), ty2.namespace());
    assert_ne!(ty.type_name(), ty2.type_name());
    assert_eq!(ty.relation_type_id(), ty2.relation_type_id());
    assert_ne!(ty.instance_id(), ty2.instance_id());
    assert_ne!(ty.to_string(), ty2.to_string());
}

// #[test]
// fn relation_instance_type_id_new_from_namespaced_type_test() {
//     let namespace = r_string();
//     let type_name = r_string();
//
//     let nt = NamespacedType::new(&namespace, &type_name);
//     let ty2 = RelationTypeId::new(nt.clone());
//     assert_eq!(namespace, ty2.namespace());
//     assert_eq!(type_name, ty2.type_name());
//
//     let nt2 = NamespacedType::from(&ty2);
//     assert_eq!(nt, nt2);
// }
//
// #[test]
// fn relation_instance_type_id_from_namespaced_type_test() {
//     let namespace = r_string();
//     let type_name = r_string();
//
//     let nt = NamespacedType::new(&namespace, &type_name);
//     let ty1 = RelationTypeId::from(nt);
//     assert_eq!(namespace, ty1.namespace());
//     assert_eq!(type_name, ty1.type_name());
// }
//
#[test]
fn relation_instance_type_id_from_identifier_test() {
    let t1 = Identifier::new("r__ns__ty").unwrap();
    let ty1 = RelationInstanceTypeId::try_from(&t1).unwrap();
    assert_eq!("ns", ty1.namespace());
    assert_eq!("ty", ty1.relation_type_id().type_name());
    assert_eq!("ty", ty1.type_name());
    assert!(ty1.instance_id().is_empty());

    let t2 = Identifier::new("r__ns__ty__instance").unwrap();
    let ty2 = RelationInstanceTypeId::try_from(&t2).unwrap();
    assert_eq!("ns", ty2.namespace());
    assert_eq!("ty", ty2.relation_type_id().type_name());
    assert_eq!("ty__instance", ty2.type_name());
    assert_eq!("instance", ty2.instance_id());

    let t3 = Identifier::new("r__ns__ty__outbound__inbound").unwrap();
    let ty3 = RelationInstanceTypeId::try_from(&t3).unwrap();
    assert_eq!("ns", ty3.namespace());
    assert_eq!("ty", ty3.relation_type_id().type_name());
    assert_eq!("ty__outbound__inbound", ty3.type_name());
    assert_eq!("outbound__inbound", ty3.instance_id());

    let t4 = Identifier::new("e__ns__ty").unwrap();
    let ty4 = RelationInstanceTypeId::try_from(&t4);
    assert!(ty4.is_err());

    let t5 = Identifier::new("r__").unwrap();
    let ty5 = RelationInstanceTypeId::try_from(&t5);
    assert!(ty5.is_err());

    let t6 = Identifier::new("r__ns").unwrap();
    let ty6 = RelationInstanceTypeId::try_from(&t6);
    assert!(ty6.is_err());

    let t7 = Identifier::new("r__ns__").unwrap();
    let ty7 = RelationInstanceTypeId::try_from(&t7);
    assert!(ty7.is_err());
}

#[test]
fn relation_instance_type_id_from_string_test() {
    let t1 = String::from("r__ns__ty");
    let ty1 = RelationInstanceTypeId::try_from(&t1).unwrap();
    assert_eq!("ns", ty1.namespace());
    assert_eq!("ty", ty1.relation_type_id().type_name());
    assert_eq!("ty", ty1.type_name());
    assert!(ty1.instance_id().is_empty());

    let t2 = String::from("r__ns__ty__instance");
    let ty2 = RelationInstanceTypeId::try_from(&t2).unwrap();
    assert_eq!("ns", ty2.namespace());
    assert_eq!("ty", ty2.relation_type_id().type_name());
    assert_eq!("ty__instance", ty2.type_name());
    assert_eq!("instance", ty2.instance_id());

    let t3 = String::from("r__ns__ty__outbound__inbound");
    let ty3 = RelationInstanceTypeId::try_from(&t3).unwrap();
    assert_eq!("ns", ty3.namespace());
    assert_eq!("ty", ty3.relation_type_id().type_name());
    assert_eq!("ty__outbound__inbound", ty3.type_name());
    assert_eq!("outbound__inbound", ty3.instance_id());

    let t4 = String::from("e__ns__ty");
    let ty4 = RelationInstanceTypeId::try_from(&t4);
    assert!(ty4.is_err());

    let t5 = String::from("r__");
    let ty5 = RelationInstanceTypeId::try_from(&t5);
    assert!(ty5.is_err());

    let t6 = String::from("r__ns");
    let ty6 = RelationInstanceTypeId::try_from(&t6);
    assert!(ty6.is_err());

    let t7 = String::from("r__ns__");
    let ty7 = RelationInstanceTypeId::try_from(&t7);
    assert!(ty7.is_err());
}

#[test]
fn relation_instance_type_id_json_schema() {
    let schema = schema_for!(RelationInstanceTypeId);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
