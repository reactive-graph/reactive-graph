use core::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

#[cfg(any(test, feature = "test"))]
use rand_derive2::RandGen;

/// The socket type defines if the property acts as an input or output socket
/// or is an hidden property
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
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

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;

#[cfg(any(test, feature = "test"))]
impl DefaultTest for SocketType {
    fn default_test() -> Self {
        SocketType::generate_random()
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::test_utils::r_string;
    use crate::SocketType;

    #[test]
    fn socket_type_should_be_created_using_static_method_call() {
        assert_eq!(SocketType::None, SocketType::none());
        assert_eq!(SocketType::Input, SocketType::input());
        assert_eq!(SocketType::Output, SocketType::output());
    }

    #[test]
    fn socket_type_from_str() {
        assert_eq!(SocketType::None, SocketType::from("none"));
        assert_eq!(SocketType::None, SocketType::from("None"));
        assert_eq!(SocketType::None, SocketType::from("NONE"));
        assert_eq!(SocketType::None, SocketType::from(r_string().as_str()));

        assert_eq!(SocketType::Input, SocketType::from("input"));
        assert_eq!(SocketType::Input, SocketType::from("Input"));
        assert_eq!(SocketType::Input, SocketType::from("INPUT"));

        assert_eq!(SocketType::Output, SocketType::from("output"));
        assert_eq!(SocketType::Output, SocketType::from("Output"));
        assert_eq!(SocketType::Output, SocketType::from("OUTPUT"));
    }

    #[test]
    fn socket_type_display() {
        assert_eq!("None", format!("{}", SocketType::None));
        assert_eq!("Input", format!("{}", SocketType::Input));
        assert_eq!("Output", format!("{}", SocketType::Output));
    }

    #[test]
    fn socket_type_json_schema() {
        let schema = schema_for!(SocketType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
