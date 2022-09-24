use serde_json::json;
use uuid::Uuid;

use crate::model::DataType;
use crate::model::PropertyInstanceGetter;
use crate::tests::utils::r_string;
use crate::EntityTypeBuilder;
use crate::ReactiveEntityInstanceBuilder;

#[test]
fn reactive_entity_instance_builder_test() {
    let type_name = r_string();
    let id = Uuid::new_v4();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let entity_instance = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .id(id)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .get();
    assert_eq!(type_name, entity_instance.type_name);
    assert_eq!(id, entity_instance.id);
    assert_eq!(property_1_value.clone().as_str(), entity_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
    let entity_instance = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .get();
    assert_eq!(type_name, entity_instance.type_name);
    assert_ne!(id, entity_instance.id);
    assert_eq!(property_1_value.clone().as_str(), entity_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
}

#[test]
fn reactive_entity_instance_from_type_test() {
    let type_name = r_string();
    let id = Uuid::new_v4();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let property_2_name = r_string();
    let entity_type = EntityTypeBuilder::new(type_name.clone())
        .property(property_1_name.clone(), DataType::String)
        .property(property_2_name.clone(), DataType::String)
        .build();
    let entity_instance = ReactiveEntityInstanceBuilder::from(entity_type.clone())
        .id(id)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .get();
    assert_eq!(type_name, entity_instance.type_name);
    assert_eq!(id, entity_instance.id);
    assert_eq!(property_1_value.clone().as_str(), entity_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
    // Should return the default value of the data type
    assert_eq!(
        DataType::String.default_value().as_str().unwrap(),
        entity_instance.get(property_2_name.clone()).unwrap().as_str().unwrap()
    );
}
