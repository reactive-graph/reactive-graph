use std::fmt::Display;
use std::fmt::Formatter;

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
pub enum SocketType {
    /// The property doesn't act as input or output socket.
    None,

    /// The property acts as input socket and accepts incoming connections.
    Input,

    /// The property acts as output socket and accepts outgoing connections.
    Output,
}

impl From<SocketType> for inexor_rgf_graph::SocketType {
    fn from(socket_type: SocketType) -> Self {
        match socket_type {
            SocketType::None => inexor_rgf_graph::SocketType::None,
            SocketType::Input => inexor_rgf_graph::SocketType::Input,
            SocketType::Output => inexor_rgf_graph::SocketType::Output,
        }
    }
}

impl From<inexor_rgf_graph::SocketType> for SocketType {
    fn from(socket_type: inexor_rgf_graph::SocketType) -> Self {
        match socket_type {
            inexor_rgf_graph::SocketType::None => SocketType::None,
            inexor_rgf_graph::SocketType::Input => SocketType::Input,
            inexor_rgf_graph::SocketType::Output => SocketType::Output,
        }
    }
}

impl Display for SocketType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", inexor_rgf_graph::SocketType::from(*self))
    }
}
