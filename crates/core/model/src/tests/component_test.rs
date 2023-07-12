use schemars::schema_for;
use serde_json::json;

use crate::tests::utils::r_string;
use crate::Component;
use crate::ComponentTypeId;
use crate::DataType;
use crate::Extension;
use crate::ExtensionTypeId;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::TypeDefinitionGetter;

#[test]
fn component_test() {
    let namespace = r_string();
    let component_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let mut property_types = Vec::new();
    let property_type = PropertyType::new(&property_name, DataType::String);
    property_types.push(property_type.clone());

    let mut extensions = Vec::new();
    let extension_namespace = r_string();
    let extension_name = r_string();
    let extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_name);
    let extension_value = json!("extension_value");
    let extension = Extension {
        ty: extension_ty.clone(),
        description: r_string(),
        extension: extension_value.clone(),
    };
    extensions.push(extension);
    let other_extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
    let other_extension = Extension::new(&other_extension_ty, r_string(), extension_value.clone());
    extensions.push(other_extension);

    let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let component = Component {
        ty,
        description: description.clone(),
        properties: property_types,
        extensions,
    };

    assert_eq!(namespace, component.namespace());
    assert_eq!(component_name, component.type_name());
    assert_eq!(format!("c__{}__{}", &namespace, &component_name), component.type_definition().to_string());
    assert_eq!(description, component.description);
    assert_eq!(&extension_namespace, &component.extensions.first().unwrap().ty.namespace());
    assert_eq!(&extension_name, &component.extensions.first().unwrap().ty.type_name());
    assert_eq!(extension_value, component.extensions.first().unwrap().extension);
    assert!(component.has_extension(&extension_ty));
    let non_existing_extension = ExtensionTypeId::new_from_type(r_string(), r_string());
    assert!(!component.has_extension(&non_existing_extension));

    let component_2 = component.clone();
    assert_eq!(component_2.type_name(), component.type_name());
}

#[test]
fn create_new_component_test() {
    let namespace = r_string();
    let component_name = r_string();
    let description = r_string();
    let mut property_types = Vec::new();
    let property_name = r_string();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    property_types.push(property_type.clone());

    let mut extensions = Vec::new();
    let extension_namespace = r_string();
    let extension_name = r_string();
    let extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_name);
    let extension_value = json!("extension_value");
    let extension = Extension {
        ty: extension_ty.clone(),
        description: r_string(),
        extension: extension_value.clone(),
    };
    extensions.push(extension);
    let other_extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
    let other_extension = Extension::new(&other_extension_ty, r_string(), extension_value.clone());
    extensions.push(other_extension);

    let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let component = Component::new(ty, description.clone(), property_types.clone(), extensions);
    assert_eq!(namespace, component.namespace());
    assert_eq!(component_name, component.type_name());
    assert_eq!(property_name.clone(), component.properties.first().unwrap().name);
    assert_eq!(property_type.data_type, component.properties.first().unwrap().data_type);
    assert!(!component.properties.iter().filter(|&p| p.name == property_name).collect::<Vec<_>>().is_empty());
    assert!(component.has_property(property_name.clone()));
    assert!(!component.has_property(r_string()));
}

#[test]
fn create_new_component_without_properties_test() {
    let namespace = r_string();
    let component_name = r_string();

    let mut extensions = Vec::new();
    let extension_namespace = r_string();
    let extension_name = r_string();
    let extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_name);
    let extension_value = json!("extension_value");
    let extension = Extension {
        ty: extension_ty.clone(),
        description: r_string(),
        extension: extension_value.clone(),
    };
    extensions.push(extension);
    let other_extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
    let other_extension = Extension::new(&other_extension_ty, r_string(), extension_value.clone());
    extensions.push(other_extension);

    let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let component = Component::new_without_properties(ty, r_string(), extensions.clone());
    assert_eq!(namespace, component.namespace());
    assert_eq!(component_name, component.type_name());
    assert_eq!(&extension_namespace, &component.extensions.first().unwrap().ty.namespace());
    assert_eq!(&extension_name, &component.extensions.first().unwrap().ty.type_name());
    assert_eq!(extension_value, component.extensions.first().unwrap().extension);
    assert!(component.has_extension(&extension_ty));
    let non_existing_extension = ExtensionTypeId::new_from_type(r_string(), r_string());
    assert!(!component.has_extension(&non_existing_extension));
}

#[test]
fn create_component_without_extensions_test() {
    let component_name = r_string();
    let namespace = r_string();

    let property_name = r_string();
    let mut property_types = Vec::new();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    property_types.push(property_type.clone());

    let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let component = Component::new_without_extensions(ty, r_string(), property_types);
    assert_eq!(namespace, component.namespace());
    assert_eq!(component_name, component.type_name());
    assert_eq!(property_name.clone(), component.properties.first().unwrap().name);
    assert_eq!(property_type.data_type, component.properties.first().unwrap().data_type);
    assert!(!component.properties.iter().filter(|&p| p.name == property_name).collect::<Vec<_>>().is_empty());
}

#[test]
fn component_has_property_test() {
    let namespace = r_string();
    let component_name = r_string();
    let mut property_types = Vec::new();
    let property_name = r_string();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    property_types.push(property_type.clone());
    let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let component = Component::new(ty, r_string(), property_types.clone(), Vec::new());
    assert!(component.has_property(property_name));
    assert!(!component.has_property(r_string()));
}

#[test]
fn component_type_ser_test() {
    let ty = ComponentTypeId::new_from_type("cnc", "ctc");
    let c = Component::new(ty, "d", Vec::new(), Vec::new());
    println!("{}", serde_json::to_string_pretty(&c).expect("Failed to serialize component"));
}

#[test]
fn component_ser_test() {
    // TODO: rename "type_name" to "name"
    // https://github.com/serde-rs/serde/pull/2160
    // https://github.com/serde-rs/serde/issues/1504)
    let s = r#"{
  "namespace": "abc",
  "type_name": "def",
  "description": "d",
  "properties": [
    {
      "name": "property_name",
      "data_type": "string",
      "socket_type": "input"
    }
  ],
  "extensions": []
}"#;
    let component: Component = serde_json::from_str(s).unwrap();
    assert_eq!("abc", component.namespace());
    assert_eq!("def", component.type_name());
    assert_eq!("c__abc__def", component.ty.to_string());
    assert_eq!("d", component.description);
    assert_eq!(1, component.properties.len());
    assert_eq!(0, component.extensions.len());
}

#[test]
fn component_json_schema() {
    let schema = schema_for!(Component);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
