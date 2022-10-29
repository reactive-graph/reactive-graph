use std::collections::HashMap;

use indradb::Edge;
use indradb::EdgeKey;
use indradb::EdgeProperties;
use indradb::NamedProperty;
use serde_json::json;
use uuid::Uuid;

use crate::property_identifier;
use crate::tests::utils::r_string;
use crate::tests::utils::r_string_1000;
use crate::Extension;
use crate::ExtensionContainer;
use crate::MutablePropertyInstanceSetter;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::RelationInstance;
use crate::RelationInstanceTypeId;
use crate::TypeDefinitionGetter;

#[test]
fn relation_instance_test() {
    let namespace = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());
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
    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    let relation_instance = RelationInstance {
        outbound_id,
        ty: ty.clone(),
        inbound_id,
        description: description.to_string(),
        properties: properties.clone(),
        extensions: extensions.clone(),
    };
    assert_eq!(namespace, relation_instance.namespace());
    assert_eq!(outbound_id, relation_instance.outbound_id);
    assert_eq!(type_name.clone(), relation_instance.type_name());
    assert_eq!(inbound_id, relation_instance.inbound_id);
    assert_eq!(description, relation_instance.description);
    assert_eq!(properties.clone(), relation_instance.properties.clone());
    assert!(relation_instance.get(property_name.clone()).is_some());
    assert!(relation_instance.get(r_string()).is_none());
    assert_eq!(property_value.clone(), relation_instance.get(property_name.clone()).unwrap());
    assert_eq!(&extension_name, &relation_instance.extensions.first().unwrap().name);
    assert_eq!(extension_value, relation_instance.extensions.first().unwrap().extension);
    assert!(relation_instance.has_own_extension(extension_name));
    assert!(!relation_instance.has_own_extension(r_string()));
    assert_eq!(extension.extension, relation_instance.get_own_extension(extension_name).unwrap().extension);
}

#[test]
fn edge_key_from_type_unique_id_test() {
    let namespace = r_string();
    let type_name = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    assert_eq!(namespace, ty.namespace());
    assert_eq!(type_name, ty.type_name());
    assert_eq!(format!("r__{}__{}", namespace, type_name), ty.type_definition().to_string());
    let relation_instance = RelationInstance {
        outbound_id,
        ty: ty.clone(),
        inbound_id,
        description: r_string(),
        properties: HashMap::new(),
        extensions: Vec::new(),
    };
    let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);

    assert_eq!(edge_key.t, relation_instance.type_id());
    assert_eq!(edge_key.t, relation_instance.get_key().t);
    assert_eq!(edge_key, relation_instance.get_key());
    let rty = relation_instance.relation_type_id();
    assert_eq!(namespace, rty.namespace());
    assert_eq!(type_name, rty.type_name());
    assert_eq!(format!("r__{namespace}__{type_name}"), relation_instance.get_key().t.as_str());
}

#[test]
fn edge_key_from_type_unique_for_instance_id_test() {
    let namespace = r_string();
    let type_name = r_string();
    let instance_id = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let ty = RelationInstanceTypeId::new_from_type_unique_for_instance_id(&namespace, &type_name, &instance_id);
    assert_eq!(namespace, ty.namespace());
    assert_eq!(type_name, ty.relation_type_id().type_name());
    assert_eq!(format!("{type_name}__{instance_id}"), ty.type_name());
    assert_eq!(format!("r__{namespace}__{type_name}__{instance_id}"), ty.type_definition().to_string());
    let relation_instance = RelationInstance {
        outbound_id,
        ty: ty.clone(),
        inbound_id,
        description: r_string(),
        properties: HashMap::new(),
        extensions: Vec::new(),
    };
    let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);

    assert_eq!(edge_key.t, relation_instance.type_id());
    assert_eq!(edge_key.t, relation_instance.get_key().t);
    assert_eq!(edge_key, relation_instance.get_key());
    assert_eq!(format!("r__{namespace}__{type_name}__{instance_id}"), relation_instance.get_key().t.as_str());
}

#[test]
fn edge_key_from_type_with_random_instance_id_test() {
    let namespace = r_string();
    let type_name = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let ty = RelationInstanceTypeId::new_from_type_with_random_instance_id(&namespace, &type_name);
    assert_eq!(namespace, ty.namespace());
    assert_eq!(type_name, ty.relation_type_id().type_name());
    assert!(ty.type_name().starts_with(&type_name));
    let expected_type_definition_prefix = format!("r__{namespace}__{type_name}__");
    assert!(ty.type_definition().to_string().starts_with(&expected_type_definition_prefix));
    let relation_instance = RelationInstance {
        outbound_id,
        ty: ty.clone(),
        inbound_id,
        description: r_string(),
        properties: HashMap::new(),
        extensions: Vec::new(),
    };
    let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);

    assert_eq!(edge_key.t, relation_instance.type_id());
    assert_eq!(edge_key.t, relation_instance.get_key().t);
    assert_eq!(edge_key, relation_instance.get_key());
    // assert_eq!(format!("r__{namespace}__{type_name}__{instance_id}"), relation_instance.get_key().t.as_str());
}

#[test]
fn edge_key_with_long_namespace_test() {
    let namespace = r_string_1000();
    let type_name = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    let relation_instance = RelationInstance {
        outbound_id,
        ty: ty.clone(),
        inbound_id,
        description: r_string(),
        properties: HashMap::new(),
        extensions: Vec::new(),
    };
    let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);

    assert_eq!(edge_key.t, relation_instance.type_id());
    assert_eq!(edge_key.t, relation_instance.get_key().t);
    assert_eq!(edge_key, relation_instance.get_key());
}

#[test]
fn edge_key_with_long_type_name_test() {
    let namespace = r_string();
    let type_name = r_string_1000();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    let relation_instance = RelationInstance {
        outbound_id,
        ty: ty.clone(),
        inbound_id,
        description: r_string(),
        properties: HashMap::new(),
        extensions: Vec::new(),
    };
    let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);

    assert_eq!(edge_key.t, relation_instance.type_id());
    assert_eq!(edge_key.t, relation_instance.get_key().t);
    assert_eq!(edge_key, relation_instance.get_key());
}

#[test]
fn edge_key_with_long_namespace_and_type_name_test() {
    let namespace = r_string_1000();
    let type_name = r_string_1000();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    let relation_instance = RelationInstance {
        outbound_id,
        ty: ty.clone(),
        inbound_id,
        description: r_string(),
        properties: HashMap::new(),
        extensions: Vec::new(),
    };
    let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);

    assert_eq!(edge_key.t, relation_instance.type_id());
    assert_eq!(edge_key.t, relation_instance.get_key().t);
    assert_eq!(edge_key, relation_instance.get_key());
}

#[test]
fn create_relation_instance_test() {
    let namespace = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());
    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    let relation_instance = RelationInstance::new(outbound_id, ty, inbound_id, properties.clone());
    assert_eq!(namespace, relation_instance.namespace());
    assert_eq!(outbound_id, relation_instance.outbound_id);
    assert_eq!(type_name, relation_instance.type_name());
    assert_eq!(inbound_id, relation_instance.inbound_id);
    assert_eq!(properties.clone(), relation_instance.properties.clone());
    assert!(relation_instance.get(property_name.clone()).is_some());
    assert!(relation_instance.get(r_string()).is_none());
    assert_eq!(property_value.clone(), relation_instance.get(property_name.clone()).unwrap());
}

#[test]
fn create_relation_instance_without_properties_test() {
    let namespace = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    let relation_instance = RelationInstance::new_without_properties(outbound_id, ty.clone(), inbound_id);
    assert_eq!(namespace, relation_instance.namespace());
    assert_eq!(outbound_id, relation_instance.outbound_id);
    assert_eq!(type_name, relation_instance.type_name());
    assert_eq!(inbound_id, relation_instance.inbound_id);
    assert_eq!(0, relation_instance.properties.len());
}

#[test]
fn create_relation_instance_from_edge_properties() {
    let namespace = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    let property_name = r_string();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: property_identifier(&property_name),
        value: property_value_json,
    };
    let properties = vec![property];
    let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);
    let edge_properties = EdgeProperties::new(Edge::new_with_current_datetime(edge_key), properties.clone());
    let relation_instance = RelationInstance::try_from(edge_properties).unwrap();
    assert_eq!(namespace, relation_instance.namespace());
    assert_eq!(outbound_id, relation_instance.outbound_id);
    assert_eq!(type_name, relation_instance.type_name());
    assert_eq!(inbound_id, relation_instance.inbound_id);
    assert_eq!(property_value.as_str(), relation_instance.properties.get(property_name.as_str()).unwrap().as_str().unwrap());
}

#[test]
fn relation_instance_typed_getter_test() {
    let namespace = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let property_name = r_string();
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), json!(false));
    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    let mut i = RelationInstance::new(outbound_id, ty.clone(), inbound_id, properties.clone());
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
    i.set(property_name.clone(), json!([]));
    assert_eq!(0, i.as_array(property_name.clone()).unwrap().len());
    i.set(property_name.clone(), json!({}));
    assert_eq!(0, i.as_object(property_name.clone()).unwrap().len());
}

#[test]
fn relation_instance_get_key_test() {
    let namespace = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
    let properties = HashMap::new();
    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    let relation_instance = RelationInstance {
        outbound_id,
        ty: ty.clone(),
        inbound_id,
        description: description.to_string(),
        properties: properties.clone(),
        extensions: Vec::new(),
    };
    assert_eq!(EdgeKey::new(outbound_id, ty.type_id(), inbound_id), relation_instance.get_key());
}
