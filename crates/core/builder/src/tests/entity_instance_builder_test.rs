use serde_json::json;
use uuid::Uuid;

use crate::model::DataType;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::tests::utils::r_string;
use crate::EntityInstanceBuilder;
use crate::EntityTypeBuilder;

#[test]
fn entity_instance_builder_test() {
    let namespace = r_string();
    let type_name = r_string();
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let id = Uuid::new_v4();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let entity_instance = EntityInstanceBuilder::new_from_type(&namespace, &type_name)
        .id(id)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .build();
    assert_eq!(namespace, entity_instance.namespace());
    assert_eq!(type_name, entity_instance.type_name());
    assert_eq!(ty, entity_instance.ty);
    assert_eq!(id, entity_instance.id);
    assert_eq!(property_1_value.clone().as_str(), entity_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());

    let entity_instance = EntityInstanceBuilder::new(ty.clone())
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .build();
    assert_eq!(namespace, entity_instance.namespace());
    assert_eq!(type_name, entity_instance.type_name());
    assert_eq!(ty, entity_instance.ty);
    assert_ne!(id, entity_instance.id);
    assert_eq!(property_1_value.clone().as_str(), entity_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
}

#[test]
fn entity_instance_from_type_test() {
    let namespace = r_string();
    let type_name = r_string();
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let id = Uuid::new_v4();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let property_2_name = r_string();
    let entity_type = EntityTypeBuilder::new(ty.clone())
        .property(property_1_name.clone(), DataType::String)
        .property(property_2_name.clone(), DataType::String)
        .build();
    let entity_instance = EntityInstanceBuilder::from(entity_type.clone())
        .id(id)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .build();
    assert_eq!(namespace, entity_instance.namespace());
    assert_eq!(type_name, entity_instance.type_name());
    assert_eq!(ty, entity_instance.ty);
    assert_eq!(id, entity_instance.id);
    assert_eq!(property_1_value.clone().as_str(), entity_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
    // Should return the default value of the data type
    assert_eq!(
        DataType::String.default_value().as_str().unwrap(),
        entity_instance.get(property_2_name.clone()).unwrap().as_str().unwrap()
    );
}
