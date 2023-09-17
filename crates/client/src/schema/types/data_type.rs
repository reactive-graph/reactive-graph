use std::fmt::Display;
use std::fmt::Formatter;

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
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

impl From<DataType> for inexor_rgf_graph::DataType {
    fn from(data_type: DataType) -> Self {
        match data_type {
            DataType::Null => crate::model::DataType::Null,
            DataType::Bool => crate::model::DataType::Bool,
            DataType::Number => crate::model::DataType::Number,
            DataType::String => crate::model::DataType::String,
            DataType::Array => crate::model::DataType::Array,
            DataType::Object => crate::model::DataType::Object,
            DataType::Any => crate::model::DataType::Any,
        }
    }
}

impl From<crate::model::DataType> for DataType {
    fn from(data_type: crate::model::DataType) -> Self {
        match data_type {
            crate::model::DataType::Null => DataType::Null,
            crate::model::DataType::Bool => DataType::Bool,
            crate::model::DataType::Number => DataType::Number,
            crate::model::DataType::String => DataType::String,
            crate::model::DataType::Array => DataType::Array,
            crate::model::DataType::Object => DataType::Object,
            crate::model::DataType::Any => DataType::Any,
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crate::model::DataType::from(*self))
    }
}
