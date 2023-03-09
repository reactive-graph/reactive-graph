use std::env;

use crate::builder::EntityTypeBuilder;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyType;
use crate::model::RelationType;
use crate::model::RelationTypeId;
use crate::model::TypeContainer;
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[test]
fn test_register_relation_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let namespace = r_string();
    let type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let outbound_type = EntityTypeBuilder::new_from_type(namespace.as_str(), outbound_type_name.as_str()).build();
    let result = entity_type_manager.register(outbound_type.clone());
    assert!(result.is_ok());
    let inbound_type = EntityTypeBuilder::new_from_type(namespace.as_str(), inbound_type_name.as_str()).build();
    let result = entity_type_manager.register(inbound_type.clone());
    assert!(result.is_ok());

    let outbound_type: ComponentOrEntityTypeId = outbound_type.ty.into();
    let relation_ty = RelationTypeId::new_from_type(&namespace, &type_name);
    let inbound_type: ComponentOrEntityTypeId = inbound_type.ty.into();
    let component_ty = ComponentTypeId::new_from_type(&namespace, &r_string());
    let relation_type = RelationType::new(outbound_type, &relation_ty, inbound_type, "", vec![component_ty], vec![PropertyType::string("x")], Vec::new());
    let result = relation_type_manager.register(relation_type);
    assert!(result.is_ok());
    assert!(relation_type_manager.has(&relation_ty));

    let relation_type: Option<RelationType> = relation_type_manager.get(&relation_ty);
    assert_eq!(type_name, relation_type.unwrap().type_name());
}

#[test]
fn test_create_and_delete_relation_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let namespace = r_string();
    let type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();
    let description = r_string();

    let outbound_type = EntityTypeBuilder::new_from_type(namespace.as_str(), outbound_type_name.as_str()).build();
    let result = entity_type_manager.register(outbound_type.clone());
    assert!(result.is_ok());
    let inbound_type = EntityTypeBuilder::new_from_type(namespace.as_str(), inbound_type_name.as_str()).build();
    let result = entity_type_manager.register(inbound_type.clone());
    assert!(result.is_ok());

    let outbound_ty: ComponentOrEntityTypeId = outbound_type.ty.into();
    let relation_ty = RelationTypeId::new_from_type(&namespace, &type_name);
    let inbound_ty: ComponentOrEntityTypeId = inbound_type.ty.into();
    let component_ty = ComponentTypeId::new_from_type(&namespace, &r_string());

    let result = relation_type_manager.create(
        &outbound_ty,
        &relation_ty,
        &inbound_ty,
        &description,
        vec![component_ty],
        vec![PropertyType::string("x")],
        Vec::new(),
    );
    assert!(result.is_ok());
    assert!(relation_type_manager.has(&relation_ty));
    assert!(relation_type_manager.has_by_type(&namespace, &type_name));

    assert_eq!(type_name, relation_type_manager.get(&relation_ty).unwrap().type_name());
    relation_type_manager.delete(&relation_ty);
    assert!(!relation_type_manager.has(&relation_ty));
    assert!(relation_type_manager.get(&relation_ty).is_none());
}

#[test]
fn test_get_relation_types() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let namespace = r_string();
    let outbound_type_name = r_string();
    let type_name = r_string();
    let inbound_type_name = r_string();
    let description = r_string();

    let outbound_type = EntityTypeBuilder::new_from_type(namespace.as_str(), outbound_type_name.as_str()).build();
    let result = entity_type_manager.register(outbound_type.clone());
    assert!(result.is_ok());
    let inbound_type = EntityTypeBuilder::new_from_type(namespace.as_str(), inbound_type_name.as_str()).build();
    let result = entity_type_manager.register(inbound_type.clone());
    assert!(result.is_ok());

    let outbound_ty: ComponentOrEntityTypeId = outbound_type.ty.into();
    let relation_ty = RelationTypeId::new_from_type(&namespace, &type_name);
    let inbound_ty: ComponentOrEntityTypeId = inbound_type.ty.into();

    let result = relation_type_manager.create(&outbound_ty, &relation_ty, &inbound_ty, &description, vec![], vec![], vec![]);
    assert!(result.is_ok());
    let relation_types = relation_type_manager.get_all();
    assert_eq!(1, relation_types.len());
    for relation_type in relation_types {
        assert!(relation_type_manager.has(&relation_type.ty));
    }
}

#[test]
fn test_register_relation_type_has_component() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let namespace = r_string();
    let component_name = r_string();

    let component =
        crate::model::Component::new_from_type(namespace.clone(), component_name.clone(), String::new(), vec![PropertyType::string("x")], Vec::new());
    let result = component_manager.register(component.clone());
    assert!(result.is_ok());
    let component_ty = component.ty.clone();

    let relation_type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let outbound_type = EntityTypeBuilder::new_from_type(&namespace, &outbound_type_name).build();
    let result = entity_type_manager.register(outbound_type.clone());
    assert!(result.is_ok());
    let inbound_type = EntityTypeBuilder::new_from_type(&namespace, &inbound_type_name).build();
    let result = entity_type_manager.register(inbound_type.clone());
    assert!(result.is_ok());

    let outbound_ty: ComponentOrEntityTypeId = outbound_type.ty.into();
    let relation_ty = RelationTypeId::new_from_type(&namespace, &relation_type_name);
    let inbound_ty: ComponentOrEntityTypeId = inbound_type.ty.into();

    let relation_type = RelationType::new(
        &outbound_ty,
        &relation_ty,
        &inbound_ty,
        String::new(),
        vec![component_ty.clone()],
        vec![PropertyType::string("y")],
        Vec::new(),
    );
    let result = relation_type_manager.register(relation_type);
    assert!(result.is_ok());
    let relation_type: RelationType = relation_type_manager.get(&relation_ty).unwrap();
    assert!(relation_type.components.contains(&component_ty));
    assert!(relation_type.is_a(&component_ty));
}

#[test]
fn test_register_relation_type_has_property() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let property_name = String::from("x");
    let property_type = PropertyType::string(&property_name);

    let namespace = r_string();
    let relation_type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let outbound_type = EntityTypeBuilder::new_from_type(&namespace, &outbound_type_name).build();
    let result = entity_type_manager.register(outbound_type.clone());
    assert!(result.is_ok());
    let inbound_type = EntityTypeBuilder::new_from_type(&namespace, &inbound_type_name).build();
    let result = entity_type_manager.register(inbound_type.clone());
    assert!(result.is_ok());

    let outbound_ty: ComponentOrEntityTypeId = outbound_type.ty.into();
    let relation_ty = RelationTypeId::new_from_type(&namespace, &relation_type_name);
    let inbound_ty: ComponentOrEntityTypeId = inbound_type.ty.into();

    let relation_type = RelationType::new(&outbound_ty, &relation_ty, &inbound_ty, String::new(), Vec::new(), vec![property_type], Vec::new());
    let result = relation_type_manager.register(relation_type);
    assert!(result.is_ok());
    let relation_type: Option<RelationType> = relation_type_manager.get(&relation_ty);
    assert!(relation_type.unwrap().has_own_property(property_name.clone()));
}

#[test]
fn test_export_import_relation_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let namespace = r_string();
    let type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();
    let description = r_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", type_name));
    let path = path.into_os_string().into_string().unwrap();

    let outbound_type = EntityTypeBuilder::new_from_type(&namespace, &outbound_type_name).build();
    let result = entity_type_manager.register(outbound_type.clone());
    assert!(result.is_ok());
    let inbound_type = EntityTypeBuilder::new_from_type(&namespace, &inbound_type_name).build();
    let result = entity_type_manager.register(inbound_type.clone());
    assert!(result.is_ok());

    let outbound_ty: ComponentOrEntityTypeId = outbound_type.ty.into();
    let relation_ty = RelationTypeId::new_from_type(&namespace, &type_name);
    let inbound_ty: ComponentOrEntityTypeId = inbound_type.ty.into();
    let component_ty = ComponentTypeId::new_from_type(&namespace, &r_string());

    let result = relation_type_manager.create(
        &outbound_ty,
        &relation_ty,
        &inbound_ty,
        &description,
        vec![component_ty],
        vec![PropertyType::string("x")],
        Vec::new(),
    );
    assert!(result.is_ok());
    relation_type_manager.export(&relation_ty, path.as_str());
    assert!(relation_type_manager.has(&relation_ty));
    relation_type_manager.delete(&relation_ty);
    assert!(!relation_type_manager.has(&relation_ty));
    let result = relation_type_manager.import(path.as_str());
    assert!(relation_type_manager.has(&relation_ty));
    assert!(result.is_ok());
}
