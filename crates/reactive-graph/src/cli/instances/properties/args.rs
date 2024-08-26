use clap::Args;
use serde_json::Value;
use std::str::FromStr;

/// The property type.
#[derive(Args, Debug, Clone)]
pub(crate) struct PropertyInstanceArgs {
    /// The name of the property.
    pub property_name: String,

    /// The value of the property.
    pub property_value: Value,
}

impl PropertyInstanceArgs {
    pub fn new(property_name: String, property_value: Value) -> Self {
        Self { property_name, property_value }
    }
}

impl FromStr for PropertyInstanceArgs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (property_name, property_value) = s.split_once("=").ok_or(())?;
        let property_value = property_value.parse::<Value>().map_err(|_| ())?;
        Ok(PropertyInstanceArgs::new(property_name.to_string(), property_value))
    }
}
