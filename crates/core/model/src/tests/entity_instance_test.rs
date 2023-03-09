use std::collections::HashMap;
use std::ops::Index;

use indradb::NamedProperty;
use indradb::Vertex;
use indradb::VertexProperties;
use serde_json::json;
use uuid::Uuid;

use crate::property_identifier;
use crate::tests::utils::r_string;
use crate::EntityInstance;
use crate::EntityTypeId;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::MutablePropertyInstanceSetter;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::TypeDefinitionGetter;

#[test]
fn entity_instance_test() {
    let uuid = Uuid::new_v4();
    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());

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
    extensions.push(extension.clone());
    let other_extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
    let other_extension = Extension::new(&other_extension_ty, r_string(), extension_value.clone());
    extensions.push(other_extension);

    let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
    let entity_instance = EntityInstance {
        ty: ty.clone(),
        id: uuid.clone(),
        description: description.to_string(),
        properties: properties.clone(),
        extensions: extensions.clone(),
    };
    assert_eq!(namespace, entity_instance.namespace());
    assert_eq!(type_name, entity_instance.type_name());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(description.clone(), entity_instance.description.clone());
    assert_eq!(properties.clone(), entity_instance.properties.clone());
    assert!(entity_instance.get(property_name.clone()).is_some());
    assert!(entity_instance.get(r_string()).is_none());
    assert_eq!(property_value.clone(), entity_instance.get(property_name.clone()).unwrap());
    assert_eq!(&extension_namespace, &entity_instance.extensions.first().unwrap().ty.namespace());
    assert_eq!(&extension_name, &entity_instance.extensions.first().unwrap().ty.type_name());
    assert!(entity_instance.has_own_extension(&extension_ty));
    let non_existing_extension = ExtensionTypeId::new_from_type(r_string(), r_string());
    assert!(!entity_instance.has_own_extension(&non_existing_extension));
    assert_eq!(extension.extension, entity_instance.get_own_extension(&extension_ty).unwrap().extension);
    assert_eq!(format!("{}__{}", entity_instance.ty, entity_instance.id), format!("{}", entity_instance));
}

#[test]
fn create_entity_instance_test() {
    let uuid = Uuid::new_v4();
    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());
    let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
    let entity_instance = EntityInstance::new(ty, uuid, properties.clone());
    assert_eq!(namespace, entity_instance.namespace());
    assert_eq!(type_name, entity_instance.type_name());
    assert_eq!(uuid, entity_instance.id.clone());
    assert_eq!(properties.clone(), properties.clone());
    assert!(entity_instance.get(property_name.clone()).is_some());
    assert!(entity_instance.get(r_string()).is_none());
    assert_eq!(property_value.clone(), entity_instance.get(property_name.clone()).unwrap());
}

#[test]
fn create_entity_instance_without_properties_test() {
    let uuid = Uuid::new_v4();
    let namespace = r_string();
    let type_name = r_string();
    let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
    let entity_instance = EntityInstance::new_without_properties(ty, uuid);
    assert_eq!(namespace, entity_instance.namespace());
    assert_eq!(type_name, entity_instance.type_name());
    assert_eq!(uuid, entity_instance.id.clone());
    assert!(entity_instance.get(r_string()).is_none());
}

#[test]
fn create_entity_instance_from_vertex_properties() {
    let uuid = Uuid::new_v4();
    let namespace = r_string();
    let type_name = r_string();
    let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
    let property_name = r_string();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: property_identifier(&property_name),
        value: property_value_json,
    };
    let properties = vec![property];
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: ty.type_id() },
        props: properties.clone(),
    };
    let entity_instance = EntityInstance::try_from(vertex_properties).unwrap();
    assert_eq!(namespace, entity_instance.namespace());
    assert_eq!(type_name, entity_instance.type_name());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(property_value.as_str(), entity_instance.properties.get(property_name.as_str()).unwrap().as_str().unwrap());
}

#[test]
fn entity_instance_typed_getter_test() {
    let uuid = Uuid::new_v4();
    let namespace = r_string();
    let type_name = r_string();
    let property_name = r_string();
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), json!(false));
    let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
    let mut i = EntityInstance::new(ty, uuid, properties.clone());
    i.set(property_name.clone(), json!(true));
    assert!(i.as_bool(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(false));
    assert!(!i.as_bool(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(123));
    assert_eq!(123, i.as_u64(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(-123));
    assert_eq!(-123, i.as_i64(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(1.23));
    assert_eq!(1.23, i.as_f64(property_name.clone()).unwrap());
    let s = r_string();
    i.set(property_name.clone(), json!(s.clone()));
    assert_eq!(s, i.as_string(property_name.clone()).unwrap());
    let a = json!([1, 2, 3]);
    i.set(property_name.clone(), a.clone());
    assert_eq!(json!(1), i.as_array(property_name.clone()).unwrap().index(0).clone());
    assert_eq!(json!(2), i.as_array(property_name.clone()).unwrap().index(1).clone());
    assert_eq!(json!(3), i.as_array(property_name.clone()).unwrap().index(2).clone());
    let o = json!({
        "k": "v"
    });
    i.set(property_name.clone(), o.clone());
    assert_eq!(json!("v"), i.as_object(property_name.clone()).unwrap().index("k").clone());
}

#[test]
fn entity_instance_ser_test() {
    let uuid = Uuid::new_v4();
    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());

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

    let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
    let entity_instance = EntityInstance {
        ty: ty.clone(),
        id: uuid.clone(),
        description: description.to_string(),
        properties: properties.clone(),
        extensions: extensions.clone(),
    };
    println!("{}", serde_json::to_string_pretty(&entity_instance).expect("Failed to serialize entity instance"));
}

#[test]
fn entity_instance_de_test() {
    let s = r#"{
  "namespace": "XARPbZkHrU",
  "type_name": "zHMZhLUpeH",
  "id": "590f4446-b080-48d3-bd14-05e09de89e62",
  "description": "gDyZTYONjh",
  "properties": {
    "NaUPOBoqyp": "qEnGqwNeEL"
  },
  "extensions": [
    {
      "namespace": "ext_namespace",
      "type_name": "ext_name",
      "extension": "extension_value"
    },
    {
      "namespace": "other_ext_namespace",
      "type_name": "other_ext_name",
      "extension": "other_extension_value"
    }
  ]
}"#;
    let entity_instance: EntityInstance = serde_json::from_str(s).unwrap();
    assert_eq!("XARPbZkHrU", entity_instance.namespace());
    assert_eq!("zHMZhLUpeH", entity_instance.type_name());
    assert_eq!("e__XARPbZkHrU__zHMZhLUpeH", entity_instance.ty.to_string());
    assert_eq!("gDyZTYONjh", entity_instance.description);
    assert_eq!(1, entity_instance.properties.len());
    let property = entity_instance.properties.get("NaUPOBoqyp").expect("Missing property");
    assert_eq!("qEnGqwNeEL", property.as_str().unwrap());
    assert_eq!(2, entity_instance.extensions.len());
    let extension = entity_instance.extensions.first().unwrap();
    assert_eq!("ext_namespace", extension.ty.namespace());
    assert_eq!("ext_name", extension.ty.type_name());
    assert_eq!(json!("extension_value"), extension.extension);
}
