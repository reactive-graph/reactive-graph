use crate::extension::field_description::DynamicGraphFieldDescriptionExtension;
use crate::extension::field_name::DynamicGraphFieldNameExtension;
use crate::field::entity::optional_field_to_vec;
use crate::field::outbound::inbound_entities_field::outbound_entity_to_inbound_components_field;
use crate::field::outbound::inbound_entities_field::outbound_entity_to_inbound_entities_field;
use crate::field::outbound::inbound_entities_field::outbound_entity_to_inbound_entities_union_field;
use crate::union::entity::UNION_ALL_ENTITIES;
use crate::union::entity::namespace_entities_union_type_name;
use async_graphql::dynamic::Field;
use log::trace;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::ComponentOrEntityTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;

pub fn outbound_entity_to_inbound_field(
    outbound_relation_type: &RelationType,
    field_names: &DynamicGraphFieldNameExtension,
    field_descriptions: &DynamicGraphFieldDescriptionExtension,
    context: &SchemaBuilderContext,
) -> Vec<Field> {
    let Some(relation_type) = context.relation_type_manager.get(&outbound_relation_type.ty) else {
        return Vec::new();
    };
    let field_name = field_names.from_outbound_entity_to_inbound_entity.clone();
    trace!("from outbound {} to inbound {:?} {}", &outbound_relation_type.ty, field_name, &relation_type.inbound_type);
    let field_description = field_descriptions.from_outbound_entity_to_inbound_entity.clone();

    match &relation_type.inbound_type {
        ComponentOrEntityTypeId::EntityType(ty) => {
            if ty.namespace() == "*" {
                optional_field_to_vec(outbound_entity_to_inbound_entities_union_field(
                    &relation_type.ty,
                    UNION_ALL_ENTITIES,
                    field_name,
                    field_description,
                ))
            } else if ty.type_name() == "*" {
                optional_field_to_vec(outbound_entity_to_inbound_entities_union_field(
                    &relation_type.ty,
                    &namespace_entities_union_type_name(&ty.namespace()),
                    field_name,
                    field_description,
                ))
            } else {
                optional_field_to_vec(outbound_entity_to_inbound_entities_field(&relation_type.ty, ty, field_name, field_description))
            }
        }
        ComponentOrEntityTypeId::Component(ty) => {
            if ty.namespace() == "*" {
                context
                    .component_manager
                    .get_type_ids()
                    .into_iter()
                    .filter_map(|ty| outbound_entity_to_inbound_components_field(&relation_type.ty, &ty, None, None))
                    .collect()
            } else if ty.type_name() == "*" {
                context
                    .component_manager
                    .get_types_by_namespace(&ty.namespace())
                    .into_iter()
                    .filter_map(|ty| outbound_entity_to_inbound_components_field(&relation_type.ty, &ty, None, None))
                    .collect()
            } else {
                optional_field_to_vec(outbound_entity_to_inbound_components_field(&relation_type.ty, ty, field_name, field_description))
            }
        }
    }
}
