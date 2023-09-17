use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;
use strum::Display;

/// The socket type defines if the property acts as an input or output socket
/// or is an hidden property
#[derive(Enum, Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Display)]
#[serde(rename_all = "lowercase")]
#[graphql(name = "SocketType", remote = "inexor_rgf_graph::SocketType")]
pub enum GraphQLSocketType {
    /// The property doesn't act as input or output socket.
    None,

    /// The property acts as input socket and accepts incoming connections.
    Input,

    /// The property acts as output socket and accepts outgoing connections.
    Output,
}

impl GraphQLSocketType {
    pub fn none() -> Self {
        GraphQLSocketType::None
    }
    pub fn input() -> Self {
        GraphQLSocketType::Input
    }
    pub fn output() -> Self {
        GraphQLSocketType::Output
    }
}
