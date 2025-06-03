use crate::field::property::field_arguments::add_entity_type_properties_as_field_arguments;
use crate::field::property::filter::get_entity_instances_by_type_filter_by_properties;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::Error;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_graph::EntityType;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub fn entity_query_field(entity_type: &EntityType) -> Field {
    let ty = entity_type.ty.clone();
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&ty);
    let mut field = Field::new(dy_ty.field_name(), TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
            if let Ok(id) = ctx.args.try_get("id") {
                let id = Uuid::from_str(id.string()?)?;
                let entity_instance = entity_instance_manager.get(id).ok_or(Error::new("Uuid not found"))?;
                if entity_instance.ty != ty {
                    return Err(Error::new(format!("Entity {} is not a {}", id, &ty)));
                }
                return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(entity_instance)])));
            }
            if let Ok(label) = ctx.args.try_get("label") {
                let entity_instance = entity_instance_manager.get_by_label(label.string()?).ok_or(Error::new("Label not found"))?;
                if entity_instance.ty != ty {
                    return Err(Error::new(format!("Entity {} is not a {}", entity_instance.id, &ty)));
                }
                return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(entity_instance)])));
            }
            let instances = get_entity_instances_by_type_filter_by_properties(&ctx, &entity_type, entity_instance_manager);
            Ok(Some(FieldValue::list(instances.into_iter().map(FieldValue::owned_any))))
        })
    })
    .description(entity_type.description.clone())
    .argument(InputValue::new("id", TypeRef::named(TypeRef::STRING)))
    .argument(InputValue::new("label", TypeRef::named(TypeRef::STRING)));
    field = add_entity_type_properties_as_field_arguments(field, entity_type, true, true);
    field
}
