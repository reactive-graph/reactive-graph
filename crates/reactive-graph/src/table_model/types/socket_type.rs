use std::fmt::Display;
use std::fmt::Formatter;
use tabled::Tabled;

#[derive(Copy, Clone, Debug, Tabled)]
pub enum SocketType {
    /// The property doesn't act as input or output socket.
    None,

    /// The property acts as input socket and accepts incoming connections.
    Input,

    /// The property acts as output socket and accepts outgoing connections.
    Output,
}

impl From<SocketType> for reactive_graph_graph::SocketType {
    fn from(socket_type: SocketType) -> Self {
        match socket_type {
            SocketType::None => reactive_graph_graph::SocketType::None,
            SocketType::Input => reactive_graph_graph::SocketType::Input,
            SocketType::Output => reactive_graph_graph::SocketType::Output,
        }
    }
}

impl From<reactive_graph_graph::SocketType> for SocketType {
    fn from(socket_type: reactive_graph_graph::SocketType) -> Self {
        match socket_type {
            reactive_graph_graph::SocketType::None => SocketType::None,
            reactive_graph_graph::SocketType::Input => SocketType::Input,
            reactive_graph_graph::SocketType::Output => SocketType::Output,
        }
    }
}

impl Display for SocketType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", reactive_graph_graph::SocketType::from(*self))
    }
}
