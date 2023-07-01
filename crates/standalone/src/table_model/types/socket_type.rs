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

impl From<SocketType> for crate::model::SocketType {
    fn from(socket_type: SocketType) -> Self {
        match socket_type {
            SocketType::None => crate::model::SocketType::None,
            SocketType::Input => crate::model::SocketType::Input,
            SocketType::Output => crate::model::SocketType::Output,
        }
    }
}

impl From<crate::model::SocketType> for SocketType {
    fn from(socket_type: crate::model::SocketType) -> Self {
        match socket_type {
            crate::model::SocketType::None => SocketType::None,
            crate::model::SocketType::Input => SocketType::Input,
            crate::model::SocketType::Output => SocketType::Output,
        }
    }
}

impl Display for SocketType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crate::model::SocketType::from(*self))
    }
}
