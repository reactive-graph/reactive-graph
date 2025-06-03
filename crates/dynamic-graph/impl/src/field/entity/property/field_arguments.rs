use crate::field::to_input_type_ref;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::InputValue;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_runtime_model::LabeledProperties::LABEL;

pub fn add_entity_type_properties_as_field_arguments(mut field: Field, entity_type: &EntityType, is_optional: bool, exclude_label: bool) -> Field {
    for property in entity_type.properties.iter() {
        if exclude_label && property.name == LABEL.property_name() {
            continue;
        }
        if let Some(type_ref) = to_input_type_ref(property.value(), is_optional) {
            field = field.argument(InputValue::new(&property.name, type_ref));
        }
    }
    field
}
