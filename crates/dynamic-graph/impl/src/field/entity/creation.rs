use crate::field::create_properties_from_field_arguments;
use crate::field::property::field_arguments::add_entity_type_properties_as_field_arguments;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::Error;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_model_impl::ReactiveProperties;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub fn entity_creation_field(entity_type: &EntityType) -> Option<Field> {
    let ty = entity_type.ty.clone();
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&ty);
    let mut field = Field::new(dy_ty.mutation_field_name("create"), TypeRef::named_nn(dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
            let id = if let Some(id) = ctx.args.get("id") {
                let id = Uuid::from_str(id.string()?)?;
                if entity_instance_manager.has(id) {
                    return Err(Error::new(format!("Uuid {id} is already taken")));
                }
                id
            } else {
                Uuid::new_v4()
            };
            let properties = create_properties_from_field_arguments(&ctx, &entity_type.properties, false)?;
            let properties = ReactiveProperties::new_with_id_from_properties(id, properties);
            let reactive_entity = ReactiveEntity::builder().ty(&ty).id(id).properties(properties).build();
            if let Ok(reactive_entity) = entity_instance_manager.register_reactive_instance(reactive_entity) {
                return Ok(Some(FieldValue::owned_any(reactive_entity)));
            }
            Ok(None)
        })
    })
    .description(format!("Create a new {} entity", entity_type.type_name()))
    .argument(InputValue::new("id", TypeRef::named(TypeRef::ID)));
    field = add_entity_type_properties_as_field_arguments(field, entity_type, false, false);
    Some(field)
}
