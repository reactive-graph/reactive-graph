use std::fmt::Display;
use std::fmt::Formatter;

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
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
            DataType::Null => inexor_rgf_graph::DataType::Null,
            DataType::Bool => inexor_rgf_graph::DataType::Bool,
            DataType::Number => inexor_rgf_graph::DataType::Number,
            DataType::String => inexor_rgf_graph::DataType::String,
            DataType::Array => inexor_rgf_graph::DataType::Array,
            DataType::Object => inexor_rgf_graph::DataType::Object,
            DataType::Any => inexor_rgf_graph::DataType::Any,
        }
    }
}

impl From<inexor_rgf_graph::DataType> for DataType {
    fn from(data_type: inexor_rgf_graph::DataType) -> Self {
        match data_type {
            inexor_rgf_graph::DataType::Null => DataType::Null,
            inexor_rgf_graph::DataType::Bool => DataType::Bool,
            inexor_rgf_graph::DataType::Number => DataType::Number,
            inexor_rgf_graph::DataType::String => DataType::String,
            inexor_rgf_graph::DataType::Array => DataType::Array,
            inexor_rgf_graph::DataType::Object => DataType::Object,
            inexor_rgf_graph::DataType::Any => DataType::Any,
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", inexor_rgf_graph::DataType::from(*self))
    }
}
