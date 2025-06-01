use reactive_graph_graph::DataType;
use serde_json::Value;
use std::fmt::Display;

/// https://json-schema.org/understanding-json-schema/reference/type
pub struct JsonSchemaDataType(DataType);

impl JsonSchemaDataType {
    pub fn new(data_type: DataType) -> Self {
        Self(data_type)
    }

    pub fn to_value(&self) -> Value {
        match self.0 {
            DataType::Null => Value::String("null".to_owned()),
            DataType::Bool => Value::String("boolean".to_owned()),
            DataType::Number => Value::String("number".to_owned()),
            DataType::String => Value::String("string".to_owned()),
            DataType::Array => Value::String("array".to_owned()),
            DataType::Object => Value::String("object".to_owned()),
            DataType::Any => Value::Array(vec![
                Value::String("null".to_owned()),
                Value::String("boolean".to_owned()),
                Value::String("number".to_owned()),
                Value::String("string".to_owned()),
                Value::String("array".to_owned()),
                Value::String("object".to_owned()),
            ]),
        }
    }
}

impl Display for JsonSchemaDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_value())
    }
}
