use crate::field::property::field_arguments::add_entity_type_properties_as_field_arguments;
use crate::field::property::filter::get_entity_instances_by_type_filter_by_properties;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_dynamic_graph_api::EntityInstanceIsNotOfType;
use reactive_graph_dynamic_graph_api::EntityInstanceNotFound;
use reactive_graph_graph::EntityType;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub fn entity_mutation_field(entity_type: &EntityType) -> Option<Field> {
    let ty = entity_type.ty.clone();
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut field = Field::new(dy_ty.field_name(), TypeRef::named_nn(dy_ty.mutation_type_name()), move |ctx| {
        let ty = ty.clone();
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
            // Multiple ids
            if let Ok(ids) = ctx.args.try_get("ids") {
                let mut entity_instances = Vec::new();
                for id in ids
                    .list()?
                    .iter()
                    .filter_map(|id| id.string().map(str::to_string).ok())
                    .filter_map(|id| Uuid::from_str(&id).ok())
                {
                    if let Some(entity_instance) = entity_instance_manager.get(id) {
                        if entity_instance.ty != ty {
                            return Err(EntityInstanceIsNotOfType(id, ty.clone()).into());
                        }
                        entity_instances.push(entity_instance);
                    }
                }
                let field_value = FieldValue::owned_any(entity_instances);
                return Ok(Some(field_value));
            }
            // Single ids
            if let Ok(id) = ctx.args.try_get("id") {
                let id = Uuid::from_str(id.string()?)?;
                let entity_instance = entity_instance_manager.get(id).ok_or(EntityInstanceNotFound(id))?;
                if entity_instance.ty != ty {
                    return Err(EntityInstanceIsNotOfType(id, ty.clone()).into());
                }
                let entity_instances = vec![entity_instance];
                let field_value = FieldValue::owned_any(entity_instances);
                return Ok(Some(field_value));
            }
            // TODO: implement label matching
            let instances = get_entity_instances_by_type_filter_by_properties(&ctx, &entity_type, entity_instance_manager);
            let field_value = FieldValue::owned_any(instances);
            Ok(Some(field_value))
        })
    })
    .description(entity_type.description.clone())
    .argument(InputValue::new("ids", TypeRef::named_nn_list(TypeRef::ID)))
    .argument(InputValue::new("id", TypeRef::named(TypeRef::ID)))
    // TODO: implement label matching
    .argument(InputValue::new("label", TypeRef::named(TypeRef::STRING)));
    field = add_entity_type_properties_as_field_arguments(field, entity_type, true, true);
    Some(field)
}
