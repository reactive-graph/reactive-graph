use apollo_encoder::{Argument, Directive, ScalarDefinition, Value};

const SPECIFIED_BY: &str = "specifiedBy";

const SPECIFIED_BY_URL: &str = "url";

const SPECIFIED_BY_URL_DEFAULT: &str = "http://";

pub(crate) fn scalar_definition<S: Into<String>>(name: S, specified_by_url: Option<S>) -> ScalarDefinition {
    let mut scalar_definition_uuid = ScalarDefinition::new(name.into());
    let mut scalar_definition_uuid_specified_by = Directive::new(SPECIFIED_BY.to_string());
    let specified_by_url = match specified_by_url {
        Some(specified_by_url) => specified_by_url.into(),
        None => SPECIFIED_BY_URL_DEFAULT.to_string(),
    };
    scalar_definition_uuid_specified_by.arg(Argument::new(SPECIFIED_BY_URL.to_string(), Value::String(specified_by_url)));
    scalar_definition_uuid.directive(scalar_definition_uuid_specified_by);
    scalar_definition_uuid
}
