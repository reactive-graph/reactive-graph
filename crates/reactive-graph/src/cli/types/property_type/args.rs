use clap::Args;
use reactive_graph_graph::DataType;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::SocketType;

/// The property type.
#[derive(Args, Debug, Clone)]
pub(crate) struct PropertyTypeDefinitionArgs {
    /// The name of the property.
    pub property_name: String,

    /// The data type of the property.
    pub data_type: DataType,

    /// The socket type of the property.
    pub socket_type: SocketType,

    /// If the property is mutable or not.
    pub mutability: Mutability,

    /// Description of the property.
    pub description: Option<String>,
    // The extensions of the property
    // pub extensions: Option<Extensions>,
}
