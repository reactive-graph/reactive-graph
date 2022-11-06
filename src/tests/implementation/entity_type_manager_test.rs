use std::env;

use crate::model::ComponentTypeId;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyType;
use crate::model::TypeContainer;
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[test]
fn test_register_entity_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();

    let component_ty = ComponentTypeId::new_from_type(&namespace, &r_string());
    let entity_type = EntityType::new_from_type(&namespace, &type_name, &description, vec![component_ty], vec![PropertyType::string("x")], vec![]);
    let result = entity_type_manager.register(entity_type.clone());
    assert!(result.is_ok());
    assert!(entity_type_manager.has_by_type(&namespace, &type_name));
    assert!(entity_type_manager.has(&entity_type.ty));

    assert_eq!(type_name, entity_type_manager.get_by_type(&namespace, &type_name).unwrap().type_name());
    assert_eq!(type_name, entity_type_manager.get(&entity_type.ty).unwrap().type_name());
}

#[test]
fn test_create_and_delete_entity_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();

    let component_ty = ComponentTypeId::new_from_type(&namespace, &r_string());
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let result = entity_type_manager.create(&ty, &description, vec![component_ty], vec![PropertyType::string("x")], vec![]);
    assert!(result.is_ok());
    assert!(entity_type_manager.has_by_type(&namespace, &type_name));
    assert!(entity_type_manager.has(&ty));

    assert_eq!(type_name, entity_type_manager.get_by_type(&namespace, &type_name).unwrap().type_name());

    entity_type_manager.delete(&ty);
    assert!(!entity_type_manager.has(&ty));
    assert!(entity_type_manager.get(&ty).is_none());
}

#[test]
fn test_get_entity_types() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let result = entity_type_manager.create(&ty, description.as_str(), vec![], vec![], vec![]);
    assert!(result.is_ok());
    let entity_types = entity_type_manager.get_all();
    assert_eq!(1, entity_types.len());
    for entity_type in entity_types {
        assert!(entity_type_manager.has(&entity_type.ty));
    }
}

#[test]
fn test_register_entity_type_has_component() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let entity_type_manager = application.get_entity_type_manager();

    let namespace = r_string();
    let component_name = r_string();
    let component_ty = ComponentTypeId::new_from_type(&namespace, &component_name);

    let component = crate::model::Component::new(&component_ty, String::new(), vec![PropertyType::string("x")], Vec::new());
    assert!(component_manager.register(component).is_ok());

    let entity_type_name = r_string();
    let description = r_string();

    let entity_ty = EntityTypeId::new_from_type(&namespace, &entity_type_name);
    let entity_type = EntityType::new(&entity_ty, &description, vec![component_ty.clone()], vec![PropertyType::string("y")], vec![]);
    let result = entity_type_manager.register(entity_type);
    assert!(result.is_ok());
    let entity_type: EntityType = entity_type_manager.get(&entity_ty).unwrap();
    assert!(entity_type.components.contains(&component_ty));
    assert!(entity_type.is_a(&component_ty));
}

#[test]
fn test_register_entity_type_has_property() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let property_name = String::from("x");
    let property_type = PropertyType::string(&property_name);

    let entity_type_name = r_string();
    let namespace = r_string();

    let entity_ty = EntityTypeId::new_from_type(&namespace, &entity_type_name);
    let entity_type = EntityType::new(&entity_ty, String::new(), vec![], vec![property_type], vec![]);
    assert!(entity_type_manager.register(entity_type).is_ok());
    assert!(entity_type_manager.get(&entity_ty).unwrap().has_own_property(property_name.as_str()));
}

#[test]
fn test_export_import_entity_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", type_name));
    let path = path.into_os_string().into_string().unwrap();

    let entity_ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let component_ty = ComponentTypeId::new_from_type(&namespace, &r_string());
    let result = entity_type_manager.create(&entity_ty, &description, vec![component_ty], vec![PropertyType::string("x")], vec![]);
    assert!(result.is_ok());
    entity_type_manager.export(&entity_ty, path.as_str());
    assert!(entity_type_manager.has(&entity_ty));
    entity_type_manager.delete(&entity_ty);
    assert!(!entity_type_manager.has(&entity_ty));
    let result = entity_type_manager.import(path.as_str());
    assert!(result.is_ok());
    assert!(entity_type_manager.has(&entity_ty));
}
