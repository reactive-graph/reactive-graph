use async_graphql::CustomValidator;
use async_graphql::InputValueError;
use reactive_graph_graph::InstanceId;
use std::str::FromStr;

pub struct InstanceIdValidator;

impl InstanceIdValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomValidator<String> for InstanceIdValidator {
    fn check(&self, instance_id: &String) -> Result<(), InputValueError<String>> {
        match InstanceId::from_str(instance_id) {
            Ok(_) => Ok(()),
            Err(e) => Err(InputValueError::custom(format!("Failed to parse instance id \"{instance_id}\": {e}"))),
        }
    }
}
