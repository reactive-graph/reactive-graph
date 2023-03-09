use serde_json::json;

use crate::model::ComponentTypeId;
use crate::model::DataType;
use crate::model::EntityTypeId;
use crate::model::ExtensionContainer;
use crate::model::ExtensionTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyType;
use crate::model::SocketType;
use crate::model::TypeContainer;
use crate::model::TypeDefinitionGetter;
use crate::tests::utils::r_string;
use crate::EntityTypeBuilder;

#[test]
fn entity_type_builder_test() {
    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();
    let component_1_namespace = r_string();
    let component_1_type_name = r_string();
    let component_1_ty = ComponentTypeId::new_from_type(&component_1_namespace, &component_1_type_name);
    let component_2_namespace = r_string();
    let component_2_type_name = r_string();
    let component_2_ty = ComponentTypeId::new_from_type(&component_2_namespace, &component_2_type_name);
    let extension_namespace = r_string();
    let extension_1_name = r_string();
    let extension_2_name = r_string();
    let property_1_name = r_string();
    let property_2_name = r_string();
    let property_3_name = r_string();
    let property_4_name = r_string();
    let property_5_name = r_string();
    let property_6_name = r_string();
    let property_7_name = r_string();
    let property_8_name = r_string();
    let property_9_name = r_string();
    let entity_type = EntityTypeBuilder::new_from_type(&namespace, &type_name)
        .description(description.clone())
        .property(property_1_name.clone(), DataType::String)
        .property_from(PropertyType::new(property_2_name.clone(), DataType::Bool))
        .string_property(property_3_name.clone())
        .bool_property(property_4_name.clone())
        .number_property(property_5_name.clone())
        .array_property(property_6_name.clone())
        .object_property(property_7_name.clone())
        .input_property(property_8_name.clone(), DataType::Bool)
        .output_property(property_9_name.clone(), DataType::Bool)
        .component(component_1_ty.clone())
        .component_from_type(&component_2_namespace, &component_2_type_name)
        .extension(&extension_namespace, &extension_1_name, json!(true))
        .extension(&extension_namespace, &extension_2_name, json!(true))
        .build();
    assert_eq!(namespace, entity_type.namespace());
    assert_eq!(type_name, entity_type.type_name());
    assert_eq!(format!("e__{namespace}__{type_name}"), entity_type.type_definition().to_string());
    assert_eq!(description, entity_type.description);
    assert!(entity_type.is_a(&component_1_ty));
    assert!(entity_type.is_a(&component_2_ty));
    let component_ty_non_existent = ComponentTypeId::new_from_type(&r_string(), &r_string());
    assert!(!entity_type.is_a(&component_ty_non_existent));
    let extension_1_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_1_name);
    assert!(entity_type.has_own_extension(&extension_1_ty));
    let extension_2_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_2_name);
    assert!(entity_type.has_own_extension(&extension_2_ty));
    let non_existing_extension = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
    assert!(!entity_type.has_own_extension(&non_existing_extension));
    assert!(entity_type.has_own_property(property_1_name.clone()));
    assert!(entity_type.has_own_property(property_2_name.clone()));
    assert!(entity_type.has_own_property(property_3_name.clone()));
    assert!(entity_type.has_own_property(property_4_name.clone()));
    assert!(entity_type.has_own_property(property_5_name.clone()));
    assert!(entity_type.has_own_property(property_6_name.clone()));
    assert!(entity_type.has_own_property(property_7_name.clone()));
    assert!(entity_type.has_own_property(property_8_name.clone()));
    assert!(entity_type.has_own_property(property_9_name.clone()));
    assert!(!entity_type.has_own_property(r_string()));
    assert_eq!(
        SocketType::Input,
        entity_type.properties.iter().find(|p| p.name == property_8_name.clone()).unwrap().socket_type
    );
    assert_eq!(
        SocketType::Output,
        entity_type.properties.iter().find(|p| p.name == property_9_name.clone()).unwrap().socket_type
    );
}

#[test]
fn entity_type_builder_new_test() {
    let namespace = r_string();
    let type_name = r_string();
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let entity_type = EntityTypeBuilder::new(&ty).build();
    assert_eq!(ty, entity_type.ty);
}
