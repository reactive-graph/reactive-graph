use apollo_encoder::EnumDefinition;
use apollo_encoder::EnumValue;

pub(crate) fn enum_definition<S: Into<String>>(name: S, enum_values: Vec<S>) -> EnumDefinition {
    let mut enum_type_type = EnumDefinition::new(name.into());
    for enum_value in enum_values {
        enum_type_type.value(EnumValue::new(enum_value.into()));
    }
    enum_type_type
}
