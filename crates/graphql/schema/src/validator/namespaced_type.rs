use async_graphql::CustomValidator;
use async_graphql::InputValueError;
use reactive_graph_graph::NamespacedType;
use std::str::FromStr;

pub struct NamespacedTypeValidator;

impl NamespacedTypeValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomValidator<String> for NamespacedTypeValidator {
    fn check(&self, _type: &String) -> Result<(), InputValueError<String>> {
        match NamespacedType::from_str(_type) {
            Ok(_) => Ok(()),
            Err(e) => Err(InputValueError::custom(format!("Failed to parse type \"{_type}\": {e}"))),
        }
    }
}
