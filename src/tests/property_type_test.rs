use crate::tests::utils::r_string;
use crate::DataType;
use crate::PropertyType;
use crate::SocketType;

#[test]
fn property_type_test() {
    let property_name = r_string();

    let property_type = PropertyType {
        name: property_name.clone(),
        description: String::new(),
        data_type: DataType::String,
        socket_type: SocketType::None,
        extensions: Vec::new(),
    };

    assert_eq!(property_name.clone(), property_type.name);
    assert_eq!(DataType::String, property_type.data_type);
}

#[test]
fn property_type_serde_test() {
    let property_name = r_string();

    let property_type = PropertyType {
        name: property_name.clone(),
        description: String::new(),
        data_type: DataType::String,
        socket_type: SocketType::None,
        extensions: Vec::new(),
    };

    let result = serde_json::to_string_pretty(&property_type.clone());
    assert!(result.is_ok());
    let result_2 = serde_json::from_str(result.unwrap().as_str());
    assert!(result_2.is_ok());
    let property_type_2: PropertyType = result_2.unwrap();

    assert_eq!(property_name.clone(), property_type_2.name);
    assert_eq!(DataType::String, property_type_2.data_type);
    assert_eq!(SocketType::None, property_type_2.socket_type);
}

#[test]
fn property_type_new_test() {
    let property_name = r_string();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    assert_eq!(property_name.clone(), property_type.name);
    assert_eq!(DataType::String, property_type.data_type);
    assert_eq!(SocketType::None, property_type.socket_type);
}

#[test]
fn property_type_new_with_socket_test() {
    let property_name = r_string();
    let property_type = PropertyType::new_with_socket(property_name.clone(), DataType::String, SocketType::Input);
    assert_eq!(property_name.clone(), property_type.name);
    assert_eq!(DataType::String, property_type.data_type);
    assert_eq!(SocketType::Input, property_type.socket_type);
}

#[test]
fn property_type_input_socket_test() {
    let property_name = r_string();
    let property_type = PropertyType::input(property_name.clone(), DataType::String);
    assert_eq!(property_name.clone(), property_type.name);
    assert_eq!(DataType::String, property_type.data_type);
    assert_eq!(SocketType::Input, property_type.socket_type);
}

#[test]
fn property_type_output_socket_test() {
    let property_name = r_string();
    let property_type = PropertyType::output(property_name.clone(), DataType::String);
    assert_eq!(property_name.clone(), property_type.name);
    assert_eq!(DataType::String, property_type.data_type);
    assert_eq!(SocketType::Output, property_type.socket_type);
}

#[test]
fn property_type_new_with_all_test() {
    let property_name = r_string();
    let description = r_string();
    let property_type = PropertyType::new_with_all(property_name.clone(), description.clone(), DataType::String, SocketType::Input, Vec::new());
    assert_eq!(property_name.clone(), property_type.name);
    assert_eq!(description.clone(), property_type.description);
    assert_eq!(DataType::String, property_type.data_type);
    assert_eq!(SocketType::Input, property_type.socket_type);
}
