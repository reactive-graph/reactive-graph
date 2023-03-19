use indradb::Identifier;

use crate::tests::utils::r_string;
use crate::EntityTypeId;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

#[test]
fn entity_type_id_test() {
    let namespace = r_string();
    let type_name = r_string();

    let nt = NamespacedType::new(&namespace, &type_name);
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    assert_eq!(namespace, ty.namespace());
    assert_eq!(type_name, ty.type_name());
    assert_eq!(nt.namespace, ty.namespace());
    assert_eq!(nt.type_name, ty.type_name());
    assert_eq!(format!("e__{namespace}__{type_name}"), format!("{}", ty));
    let type_definition = ty.type_definition();
    assert_eq!(TypeIdType::EntityType, type_definition.type_id_type);
    assert_eq!(namespace, type_definition.namespace());
    assert_eq!(type_name, type_definition.type_name());

    let type_definition_3 = TypeDefinition::from(&ty);
    assert_eq!(TypeIdType::EntityType, type_definition_3.type_id_type);
    assert_eq!(namespace, type_definition_3.namespace());
    assert_eq!(type_name, type_definition_3.type_name());
}

#[test]
fn entity_type_id_new_from_namespaced_type_test() {
    let namespace = r_string();
    let type_name = r_string();

    let nt = NamespacedType::new(&namespace, &type_name);
    let ty2 = EntityTypeId::new(nt.clone());
    assert_eq!(namespace, ty2.namespace());
    assert_eq!(type_name, ty2.type_name());

    let nt2 = NamespacedType::from(&ty2);
    assert_eq!(nt, nt2);
}

#[test]
fn entity_type_id_from_namespaced_type_test() {
    let namespace = r_string();
    let type_name = r_string();

    let nt = NamespacedType::new(&namespace, &type_name);
    let ty1 = EntityTypeId::from(nt);
    assert_eq!(namespace, ty1.namespace());
    assert_eq!(type_name, ty1.type_name());
}

#[test]
fn entity_type_id_from_identifier_test() {
    let t1 = Identifier::new("e__ns__ty").unwrap();
    let ty1 = EntityTypeId::try_from(&t1).unwrap();
    assert_eq!("ns", ty1.namespace());
    assert_eq!("ty", ty1.type_name());

    let t2 = Identifier::new("r__ns__ty").unwrap();
    let ty2 = EntityTypeId::try_from(&t2);
    assert!(ty2.is_err());

    let t3 = Identifier::new("e__").unwrap();
    let ty3 = EntityTypeId::try_from(&t3);
    assert!(ty3.is_err());

    let t4 = Identifier::new("e__ns").unwrap();
    let ty4 = EntityTypeId::try_from(&t4);
    assert!(ty4.is_err());

    let t5 = Identifier::new("e__ns__").unwrap();
    let ty5 = EntityTypeId::try_from(&t5);
    assert!(ty5.is_err());

    let t6 = Identifier::new("e__ns__ty__").unwrap();
    let ty6 = EntityTypeId::try_from(&t6);
    assert!(ty6.is_err());

    let t7 = Identifier::new("e__ns__ty__xx").unwrap();
    let ty7 = EntityTypeId::try_from(&t7);
    assert!(ty7.is_err());
}

#[test]
fn entity_type_id_from_string_test() {
    let s1 = String::from("e__ns__ty");
    let ty1 = EntityTypeId::try_from(&s1).unwrap();
    assert_eq!("ns", ty1.namespace());
    assert_eq!("ty", ty1.type_name());

    let s2 = String::from("r__ns__ty");
    let ty2 = EntityTypeId::try_from(&s2);
    assert!(ty2.is_err());

    let s3 = String::from("e__");
    let ty3 = EntityTypeId::try_from(&s3);
    assert!(ty3.is_err());

    let s4 = String::from("e__ns");
    let ty4 = EntityTypeId::try_from(&s4);
    assert!(ty4.is_err());

    let s5 = String::from("e__ns__");
    let ty5 = EntityTypeId::try_from(&s5);
    assert!(ty5.is_err());

    let s6 = String::from("e__ns__ty__");
    let ty6 = EntityTypeId::try_from(&s6);
    assert!(ty6.is_err());

    let s7 = String::from("e__ns__ty__xx");
    let ty7 = EntityTypeId::try_from(&s7);
    assert!(ty7.is_err());
}