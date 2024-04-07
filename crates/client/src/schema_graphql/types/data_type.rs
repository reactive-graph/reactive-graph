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

impl From<DataType> for reactive_graph_graph::DataType {
    fn from(data_type: DataType) -> Self {
        match data_type {
            DataType::Null => reactive_graph_graph::DataType::Null,
            DataType::Bool => reactive_graph_graph::DataType::Bool,
            DataType::Number => reactive_graph_graph::DataType::Number,
            DataType::String => reactive_graph_graph::DataType::String,
            DataType::Array => reactive_graph_graph::DataType::Array,
            DataType::Object => reactive_graph_graph::DataType::Object,
            DataType::Any => reactive_graph_graph::DataType::Any,
        }
    }
}

impl From<reactive_graph_graph::DataType> for DataType {
    fn from(data_type: reactive_graph_graph::DataType) -> Self {
        match data_type {
            reactive_graph_graph::DataType::Null => DataType::Null,
            reactive_graph_graph::DataType::Bool => DataType::Bool,
            reactive_graph_graph::DataType::Number => DataType::Number,
            reactive_graph_graph::DataType::String => DataType::String,
            reactive_graph_graph::DataType::Array => DataType::Array,
            reactive_graph_graph::DataType::Object => DataType::Object,
            reactive_graph_graph::DataType::Any => DataType::Any,
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", reactive_graph_graph::DataType::from(*self))
    }
}
