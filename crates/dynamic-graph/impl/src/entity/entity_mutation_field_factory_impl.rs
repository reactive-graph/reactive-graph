use crate::field::create::FIELD_NAME_PREFIX_CREATE;
use crate::field::property_instance::create_properties_from_field_arguments;
use crate::object_type_name::object_type_name;
use crate::object_type_name::object_type_ref;
use crate::property_type_container::add_property_type_container_properties_as_field_arguments;
use crate::query_arguments::QUERY_ARGUMENT_ID;
use crate::query_arguments::query_argument_id;
use crate::sort::sort_by_key;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_trait::async_trait;
use convert_case::Case::Pascal;
use convert_case::Casing;
use itertools::Itertools;
use reactive_graph_dynamic_graph_api::EntityMutationFieldFactory;
use reactive_graph_dynamic_graph_api::EntityQueryFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType::Mutation;
use reactive_graph_dynamic_graph_api::RootObjectType::Query;
use reactive_graph_graph::CreateEntityInstanceError;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_model_impl::ReactiveProperties;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_type_system_api::EntityTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
pub struct EntityMutationFieldFactoryImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    entity_query_field_factory: Arc<dyn EntityQueryFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl EntityMutationFieldFactory for EntityMutationFieldFactoryImpl {
    fn create_mutation_fields(&self, namespace: &Namespace) -> Vec<Field> {
        let mut fields = Vec::new();
        for entity_type in self.entity_type_manager.get_by_namespace(&namespace).iter().sorted_by(sort_by_key) {
            fields.push(self.entity_query_field_factory.create_query_field(&entity_type, Mutation));
            fields.push(self.create_creation_field(&entity_type));
        }
        fields
    }

    fn create_creation_field(&self, entity_type: &EntityType) -> Field {
        let ty = entity_type.ty.clone();
        let entity_type_inner = entity_type.clone();
        let field_name = format!("{}{}", FIELD_NAME_PREFIX_CREATE, ty.type_name().to_case(Pascal));
        let mut field = Field::new(field_name, object_type_ref(&entity_type.ty, Query), move |ctx| {
            let ty = ty.clone();
            let entity_type = entity_type_inner.clone();
            FieldFuture::new(async move {
                let reactive_entity_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
                let id = if let Some(id) = ctx.args.get(QUERY_ARGUMENT_ID) {
                    let id = Uuid::from_str(id.string()?)?;
                    if reactive_entity_manager.has(id) {
                        return Err(CreateEntityInstanceError::EntityInstanceAlreadyExist(id).into());
                    }
                    id
                } else {
                    Uuid::new_v4()
                };
                let properties = create_properties_from_field_arguments(&ctx, &entity_type.properties, false)?;
                let properties = ReactiveProperties::new_with_id_from_properties(id, properties);
                let reactive_entity = ReactiveEntity::builder().ty(&ty).id(id).properties(properties).build();
                if let Ok(reactive_entity) = reactive_entity_manager.register_reactive_instance(reactive_entity) {
                    return Ok(Some(FieldValue::owned_any(reactive_entity)));
                }
                Ok(None)
            })
        })
        .description(format!("Create a new {} entity", object_type_name(&entity_type.ty, Query)))
        .argument(query_argument_id());
        field = add_property_type_container_properties_as_field_arguments(field, entity_type, false, false);
        field
    }
}

#[async_trait]
impl Lifecycle for EntityMutationFieldFactoryImpl {}
