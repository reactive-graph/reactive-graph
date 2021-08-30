use crate::DataType;
use serde_json::Value;
use crate::tests::utils::r_string;

#[test]
fn data_type_should_be_created_using_static_method_call() {
    assert_eq!(DataType::Bool, DataType::bool());
    assert_eq!(DataType::Number, DataType::number());
    assert_eq!(DataType::String, DataType::string());
}

#[test]
fn data_type_should_return_the_correct_default_value() {
    assert_eq!(false, DataType::Bool.default_value());
    assert_eq!(0, DataType::Number.default_value());
    assert_eq!("", DataType::String.default_value());
    let empty_vec: Vec<Value> = Vec::new();
    assert!(DataType::Array.default_value().is_array());
    assert!(DataType::Array.default_value().as_array().is_some());
    assert_eq!(empty_vec, *DataType::Array.default_value().as_array().unwrap());
    let empty_map = serde_json::Map::new();
    assert!(DataType::Object.default_value().is_object());
    assert!(DataType::Object.default_value().as_object().is_some());
    assert_eq!(empty_map, *DataType::Object.default_value().as_object().unwrap());

    assert!(DataType::Any.default_value().is_string());
    assert_eq!("", DataType::Any.default_value());
}

#[test]
fn data_type_from_str() {
    assert_eq!(DataType::Bool, DataType::from("bool"));
    assert_eq!(DataType::Bool, DataType::from("Bool"));
    assert_eq!(DataType::Bool, DataType::from("BOOL"));

    assert_eq!(DataType::Number, DataType::from("number"));
    assert_eq!(DataType::Number, DataType::from("Number"));
    assert_eq!(DataType::Number, DataType::from("NUMBER"));

    assert_eq!(DataType::String, DataType::from("string"));
    assert_eq!(DataType::String, DataType::from("String"));
    assert_eq!(DataType::String, DataType::from("STRING"));

    assert_eq!(DataType::Array, DataType::from("array"));
    assert_eq!(DataType::Array, DataType::from("Array"));
    assert_eq!(DataType::Array, DataType::from("ARRAY"));

    assert_eq!(DataType::Object, DataType::from("object"));
    assert_eq!(DataType::Object, DataType::from("Object"));
    assert_eq!(DataType::Object, DataType::from("OBJECT"));

    assert_eq!(DataType::Any, DataType::from("any"));
    assert_eq!(DataType::Any, DataType::from("Any"));
    assert_eq!(DataType::Any, DataType::from("ANY"));

    // Fallback to String
    assert_eq!(DataType::String, DataType::from(r_string().as_str()));
}

#[test]
fn data_type_display() {
    assert_eq!("Bool", format!("{}", DataType::Bool));
    assert_eq!("Number", format!("{}", DataType::Number));
    assert_eq!("String", format!("{}", DataType::String));
    assert_eq!("Array", format!("{}", DataType::Array));
    assert_eq!("Object", format!("{}", DataType::Object));
}
