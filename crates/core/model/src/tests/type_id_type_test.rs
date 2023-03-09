use crate::TypeIdType;
use crate::TYPE_ID_TYPE_NAMESPACE_BEHAVIOUR;
use crate::TYPE_ID_TYPE_NAMESPACE_COMPONENT;
use crate::TYPE_ID_TYPE_NAMESPACE_ENTITY_TYPE;
use crate::TYPE_ID_TYPE_NAMESPACE_EXTENSION;
use crate::TYPE_ID_TYPE_NAMESPACE_FLOW_TYPE;
use crate::TYPE_ID_TYPE_NAMESPACE_RELATION_TYPE;

#[test]
fn type_id_type_to_string_test() {
    let tidt_b = TypeIdType::Behaviour;
    assert_eq!("b", tidt_b.to_string());

    let tidt_c = TypeIdType::Component;
    assert_eq!("c", tidt_c.to_string());

    let tidt_e = TypeIdType::EntityType;
    assert_eq!("e", tidt_e.to_string());

    let tidt_x = TypeIdType::Extension;
    assert_eq!("x", tidt_x.to_string());

    let tidt_r = TypeIdType::RelationType;
    assert_eq!("r", tidt_r.to_string());

    let tidt_f = TypeIdType::FlowType;
    assert_eq!("f", tidt_f.to_string());
}

#[test]
fn type_id_type_uuid_test() {
    let tidt_b_uuid = TypeIdType::Behaviour.into();
    assert_eq!(TYPE_ID_TYPE_NAMESPACE_BEHAVIOUR, tidt_b_uuid);

    let tidt_c_uuid = TypeIdType::Component.into();
    assert_eq!(TYPE_ID_TYPE_NAMESPACE_COMPONENT, tidt_c_uuid);

    let tidt_e_uuid = TypeIdType::EntityType.into();
    assert_eq!(TYPE_ID_TYPE_NAMESPACE_ENTITY_TYPE, tidt_e_uuid);

    let tidt_x_uuid = TypeIdType::Extension.into();
    assert_eq!(TYPE_ID_TYPE_NAMESPACE_EXTENSION, tidt_x_uuid);

    let tidt_r_uuid = TypeIdType::RelationType.into();
    assert_eq!(TYPE_ID_TYPE_NAMESPACE_RELATION_TYPE, tidt_r_uuid);

    let tidt_f_uuid = TypeIdType::FlowType.into();
    assert_eq!(TYPE_ID_TYPE_NAMESPACE_FLOW_TYPE, tidt_f_uuid);
}

#[test]
fn type_id_type_from_str_test() {
    assert_eq!(TypeIdType::Behaviour, TypeIdType::try_from("b").unwrap());
    assert_eq!(TypeIdType::Component, TypeIdType::try_from("c").unwrap());
    assert_eq!(TypeIdType::EntityType, TypeIdType::try_from("e").unwrap());
    assert_eq!(TypeIdType::Extension, TypeIdType::try_from("x").unwrap());
    assert_eq!(TypeIdType::RelationType, TypeIdType::try_from("r").unwrap());
    assert_eq!(TypeIdType::FlowType, TypeIdType::try_from("f").unwrap());
    assert!(TypeIdType::try_from("a").is_err());
    assert!(TypeIdType::try_from("abc").is_err());
}
