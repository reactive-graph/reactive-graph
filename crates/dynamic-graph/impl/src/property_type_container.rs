use crate::field::property::datatype::to_input_type_ref;
use crate::filter_by_expected_value::filter_by_expected_value;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::ResolverContext;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::VariablesContainer;
use reactive_graph_model_core::reactive_graph::core::labeled::LabeledProperties::LABEL;

pub fn filter_instances_by_properties<T: PropertyTypeContainer, I: PropertyInstanceGetter>(
    ctx: &ResolverContext,
    property_type_container: &T,
    mut instances: Vec<I>,
) -> Vec<I> {
    for property in property_type_container.get_own_properties_cloned().iter() {
        let Some(expected_value) = ctx.args.get(&property.name) else {
            continue;
        };
        instances.retain(|instance| filter_by_expected_value(instance, property.value(), &expected_value));
    }
    instances
}

pub fn add_property_type_container_properties_as_field_arguments<T: PropertyTypeContainer>(
    field: Field,
    property_type_container: &T,
    is_optional: bool,
    exclude_label: bool,
) -> Field {
    add_properties_as_field_arguments(field, property_type_container.get_own_properties_cloned(), is_optional, exclude_label)
}

pub fn add_variables_container_properties_as_field_arguments<T: VariablesContainer>(
    field: Field,
    variables_container: &T,
    is_optional: bool,
    exclude_label: bool,
) -> Field {
    add_properties_as_field_arguments(field, variables_container.get_own_variables_cloned(), is_optional, exclude_label)
}

pub fn add_properties_as_field_arguments(mut field: Field, property_types: PropertyTypes, is_optional: bool, exclude_label: bool) -> Field {
    for property in property_types.iter() {
        if exclude_label && property.name == LABEL.as_ref() {
            continue;
        }
        if let Some(type_ref) = to_input_type_ref(property.value(), is_optional) {
            field = field.argument(InputValue::new(&property.name, type_ref));
        }
    }
    field
}
