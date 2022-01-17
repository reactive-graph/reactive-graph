use core::fmt;
use std::fmt::{Display, Formatter};

use async_graphql::*;
use serde::{Deserialize, Serialize};

/// The socket type defines if the property acts as an input or output socket
/// or is an hidden property
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Enum, Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SocketType {
    /// The property doesn't act as input or output socket.
    None,

    /// The property acts as input socket and accepts incoming connections.
    Input,

    /// The property acts as output socket and accepts outgoing connections.
    Output,
}

impl SocketType {
    pub fn none() -> Self {
        SocketType::None
    }
    pub fn input() -> Self {
        SocketType::Input
    }
    pub fn output() -> Self {
        SocketType::Output
    }
}

impl From<&str> for SocketType {
    fn from(value: &str) -> Self {
        return match value.to_lowercase().as_str() {
            "none" => Self::None,
            "input" => Self::Input,
            "output" => Self::Output,
            _ => Self::None,
        };
    }
}

impl Display for SocketType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
