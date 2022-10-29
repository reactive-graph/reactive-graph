use indradb::Identifier;
use serde_json::json;

use crate::tests::utils::r_string;
use crate::tests::utils::r_string_1000;
use crate::ComponentTypeId;
use crate::DataType;
use crate::EntityType;
use crate::EntityTypeId;
use crate::Extension;
use crate::ExtensionContainer;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::SocketType;
use crate::TypeContainer;
use crate::TypeDefinitionGetter;

#[test]
fn create_entity_type_test() {
    let entity_type_name = r_string();

    let namespace = r_string();
    let description = r_string();

    let component_name = r_string();
    let mut component_names = Vec::new();
    let component_ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    component_names.push(component_ty.clone());

    let mut property_types = Vec::new();
    let property_name = "property_name";
    let property_type = PropertyType::new(property_name, DataType::String);
    property_types.push(property_type.clone());

    let mut extensions = Vec::new();
    let extension_name = "extension_name";
    let extension_value = json!("extension_value");
    let extension = Extension {
        name: extension_name.to_string(),
        extension: extension_value.clone(),
    };
    extensions.push(extension);
    let extension = Extension::new("other_extension", extension_value.clone());
    extensions.push(extension.clone());

    let ty = EntityTypeId::new_from_type(&namespace, &entity_type_name);
    let entity_type = EntityType::new(ty, &description, component_names, property_types, extensions);

    assert_eq!(namespace, entity_type.namespace());

    assert_eq!(entity_type_name, entity_type.type_name());

    assert_eq!(format!("e__{}__{}", &namespace, &entity_type_name), entity_type.type_definition().to_string());
    assert_eq!(
        Identifier::new(entity_type.type_definition().to_string().as_str()).unwrap(),
        (&entity_type.type_definition()).into()
    );

    assert_eq!(description, entity_type.description);

    assert_eq!(component_ty, *entity_type.components.first().unwrap());

    assert!(entity_type.is_a(&component_ty));

    assert_eq!(property_name, entity_type.properties.first().unwrap().name);

    assert!(entity_type.has_own_property(property_name));
    assert!(!entity_type.has_own_property(r_string()));
    assert_eq!(property_type.data_type, entity_type.get_own_property(property_name).unwrap().data_type);

    assert_eq!(&extension_name, &entity_type.extensions.first().unwrap().name);

    assert_eq!(extension_value, entity_type.extensions.first().unwrap().extension);
    assert!(entity_type.has_own_extension(extension_name));
    assert!(!entity_type.has_own_extension(r_string()));
    assert_eq!(extension.extension, entity_type.get_own_extension(extension_name).unwrap().extension);
}

#[test]
fn long_entity_type_test() {
    let namespace = r_string_1000();
    let entity_type_name = r_string_1000();
    let description = r_string();
    let ty = EntityTypeId::new_from_type(&namespace, &entity_type_name);
    let et = EntityType::new(ty, description, Vec::new(), Vec::new(), Vec::new());
    let identifier: Identifier = et.type_id();
    assert!(identifier.as_str().len() < 255);
}

#[test]
fn entity_type_serde_test() {
    let s = r#"{
  "namespace": "abc",
  "type_name": "def",
  "description": "d",
  "components": [
    {
      "namespace": "mno",
      "type_name": "pqr"
    }
  ],
  "properties": [
    {
      "name": "property_name",
      "data_type": "string",
      "socket_type": "input"
    }
  ],
  "extensions": [
    {
      "name": "ext_name",
      "extension": "ext_value"
    }
  ]
}"#;
    let entity_type: EntityType = serde_json::from_str(s).unwrap();
    assert_eq!("abc", entity_type.namespace());
    assert_eq!("def", entity_type.type_name());
    assert_eq!("e__abc__def", entity_type.ty.to_string());
    assert_eq!("d", entity_type.description);
    assert_eq!(1, entity_type.components.len());
    let component = entity_type.components.first().unwrap();
    assert_eq!("mno", component.namespace());
    assert_eq!("pqr", component.type_name());
    assert_eq!(1, entity_type.properties.len());
    let property = entity_type.properties.first().unwrap();
    assert_eq!("property_name", property.name);
    assert_eq!(DataType::String, property.data_type);
    assert_eq!(SocketType::Input, property.socket_type);
    assert_eq!(1, entity_type.extensions.len());
    let extension = entity_type.extensions.first().unwrap();
    assert_eq!("ext_name", extension.name);
    assert_eq!(json!("ext_value"), extension.extension);
}
