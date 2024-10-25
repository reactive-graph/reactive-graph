use core::fmt;
use fmt::Display;
use std::collections::HashMap;
use std::fmt::Formatter;

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use rand_derive2::RandGen;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

#[cfg(any(test, feature = "test"))]
use reactive_graph_test_utils::r_string;

/// Derived from serde_json::Value but without value payload.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub enum DataType {
    /// Represents a JSON null value.
    Null,

    /// Represents a JSON boolean.
    Bool,

    /// Represents a JSON number, whether integer or floating point.
    Number,

    /// Represents a JSON string.
    String,

    /// Represents a JSON array.
    Array,

    /// Represents a JSON object.
    Object,

    /// Represents any type (relations).
    Any,
}

impl DataType {
    pub fn bool() -> Self {
        DataType::Bool
    }

    pub fn number() -> Self {
        DataType::Number
    }

    pub fn string() -> Self {
        DataType::String
    }

    pub fn default_value(&self) -> Value {
        match self {
            DataType::Bool => json!(false),
            DataType::Number => json!(0),
            DataType::String => json!(""),
            DataType::Array => json!(Vec::<Value>::new()),
            DataType::Object => json!(HashMap::<String, Value>::new()),
            _ => json!(""),
        }
    }
}

impl From<&str> for DataType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "bool" => Self::Bool,
            "number" => Self::Number,
            "string" => Self::String,
            "array" => Self::Array,
            "object" => Self::Object,
            "any" => Self::Any,
            _ => Self::String,
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for DataType {
    fn default_test() -> Self {
        DataType::generate_random()
    }
}

#[cfg(any(test, feature = "test"))]
impl DataType {
    /// Returns a "default value" for a data type, containing mocked values suitable for use
    /// in tests. Default values may contain literals, unique numbers, etc, to make test
    /// assertions easier to work with.
    pub fn default_value_test(&self) -> Value {
        let mut rng = rand::thread_rng();
        match self {
            DataType::Bool => {
                let b: bool = rng.gen();
                json!(b)
            }
            DataType::Number => {
                let number: i64 = rng.gen();
                json!(number)
            }
            DataType::String => json!(r_string()),
            DataType::Array => {
                let mut array = Vec::<Value>::new();
                for _ in 0..rng.gen_range(0..10) {
                    array.push(DataType::generate_random_primitive().default_value_test());
                }
                json!(array)
            }
            DataType::Object => {
                let mut object = HashMap::<String, Value>::new();
                for _ in 0..rng.gen_range(0..10) {
                    object.insert(r_string(), DataType::generate_random_primitive().default_value_test());
                }
                json!(object)
            }
            DataType::Any => DataType::generate_random().default_value_test(),
            DataType::Null => json!(0),
        }
    }

    /// Returns a random data type, but only primitives (bool, number, string) in
    /// order to avoid nesting.
    pub fn generate_random_primitive() -> Self {
        match DataType::generate_random() {
            DataType::Bool => DataType::Bool,
            DataType::Number => DataType::Number,
            DataType::String => DataType::String,
            _ => DataType::generate_random_primitive(),
        }
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use serde_json::Value;

    use crate::DataType;
    use reactive_graph_test_utils::r_string;

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

    #[test]
    fn data_type_json_schema() {
        let schema = schema_for!(DataType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
