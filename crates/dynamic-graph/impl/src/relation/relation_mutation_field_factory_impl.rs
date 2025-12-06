use crate::field::create::FIELD_NAME_PREFIX_CREATE;
use crate::field::property_instance::create_properties_from_field_arguments;
use crate::object_type_name::object_type_name;
use crate::object_type_name::object_type_ref;
use crate::property_type_container::add_property_type_container_properties_as_field_arguments;
use crate::query_arguments::QUERY_ARGUMENT_INBOUND_ID;
use crate::query_arguments::QUERY_ARGUMENT_INSTANCE_ID;
use crate::query_arguments::QUERY_ARGUMENT_OUTBOUND_ID;
use crate::query_arguments::query_argument_inbound_id;
use crate::query_arguments::query_argument_instance_id;
use crate::query_arguments::query_argument_outbound_id;
use crate::sort::sort_by_key;
use async_graphql::Error;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_trait::async_trait;
use convert_case::Case::Pascal;
use convert_case::Casing;
use itertools::Itertools;
use reactive_graph_dynamic_graph_api::RelationMutationFieldFactory;
use reactive_graph_dynamic_graph_api::RelationQueryFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_dynamic_graph_api::RootObjectType::Query;
use reactive_graph_graph::InstanceId;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_graph::RelationType;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveProperties;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveRelationCreationError;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_reactive_service_api::ReactiveRelationRegistrationError;
use reactive_graph_type_system_api::RelationTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
pub struct RelationMutationFieldFactoryImpl {
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    relation_query_field_factory: Arc<dyn RelationQueryFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RelationMutationFieldFactory for RelationMutationFieldFactoryImpl {
    fn get_mutation_fields(&self, namespace: &Namespace) -> Vec<Field> {
        let mut fields = Vec::new();
        for relation_type in self.relation_type_manager.get_by_namespace(&namespace).iter().sorted_by(sort_by_key) {
            fields.push(self.relation_query_field_factory.create_query_field(&relation_type, RootObjectType::Mutation));
            fields.push(self.create_creation_field(&relation_type));
        }
        fields
    }

    fn create_creation_field(&self, relation_type: &RelationType) -> Field {
        let ty = relation_type.ty.clone();
        let relation_type_inner = relation_type.clone();
        let field_name = format!("{}{}", FIELD_NAME_PREFIX_CREATE, ty.type_name().to_case(Pascal));
        let mut field = Field::new(field_name, object_type_ref(&relation_type.ty, Query), move |ctx| {
            let ty = ty.clone();
            let relation_type = relation_type_inner.clone();
            FieldFuture::new(async move {
                let reactive_entity_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
                let reactive_relation_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;

                let outbound_id = Uuid::from_str(ctx.args.try_get(QUERY_ARGUMENT_OUTBOUND_ID)?.string()?)?;
                let inbound_id = Uuid::from_str(ctx.args.try_get(QUERY_ARGUMENT_INBOUND_ID)?.string()?)?;
                let rty = match ctx.args.get(QUERY_ARGUMENT_INSTANCE_ID) {
                    Some(instance_id) => RelationInstanceTypeId::new(ty, InstanceId::parse_named(instance_id.string()?)?),
                    None => RelationInstanceTypeId::new_with_random_instance_id(ty),
                };
                let id = RelationInstanceId::builder().outbound_id(outbound_id).ty(&rty).inbound_id(inbound_id).build();

                if reactive_relation_manager.has(&id) {
                    return Err(ReactiveRelationRegistrationError::RelationInstanceAlreadyExists(id.clone()).into());
                }

                let outbound = reactive_entity_manager
                    .get(outbound_id)
                    .ok_or::<Error>(ReactiveRelationCreationError::MissingOutboundEntityInstance(outbound_id).into())?;

                let inbound = reactive_entity_manager
                    .get(inbound_id)
                    .ok_or::<Error>(ReactiveRelationCreationError::MissingInboundEntityInstance(inbound_id).into())?;

                let properties = create_properties_from_field_arguments(&ctx, &relation_type.properties, false)?;
                let properties = ReactiveProperties::new_with_id_from_properties(id, properties);
                let reactive_relation = ReactiveRelation::builder()
                    .outbound(outbound)
                    .ty(&rty)
                    .inbound(inbound)
                    .properties(properties)
                    .build();
                if let Ok(reactive_relation) = reactive_relation_manager.register_reactive_instance(reactive_relation) {
                    return Ok(Some(FieldValue::owned_any(reactive_relation)));
                }
                Ok(None)
            })
        })
        .description(format!("Create a new {} relation", object_type_name(&relation_type.ty, Query)))
        .argument(query_argument_outbound_id())
        .argument(query_argument_inbound_id())
        .argument(query_argument_instance_id());
        field = add_property_type_container_properties_as_field_arguments(field, relation_type, false, false);
        field
    }
}

#[async_trait]
impl Lifecycle for RelationMutationFieldFactoryImpl {}
