use crate::json_schema_field_factory_impl::json_schema_field_name;
use crate::object_type_name::object_type_ref_list;
use crate::property_type_container::add_property_type_container_properties_as_field_arguments;
use crate::property_type_container::filter_instances_by_properties;
use crate::query_arguments::query_argument_inbound_id;
use crate::query_arguments::query_argument_outbound_id;
use crate::sort::sort_by_key;
use async_graphql::Error;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::ResolverContext;
use async_trait::async_trait;
use convert_case::Case::Pascal;
use convert_case::Casing;
use itertools::Itertools;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RelationQueryFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_type_system_api::RelationTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
pub struct RelationQueryFieldFactoryImpl {
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RelationQueryFieldFactory for RelationQueryFieldFactoryImpl {
    fn create_query_fields(&self, namespace: &Namespace) -> Vec<Field> {
        let mut fields = Vec::new();
        for relation_type in self.relation_type_manager.get_by_namespace(&namespace).iter().sorted_by(sort_by_key) {
            fields.push(self.create_query_field(&relation_type, RootObjectType::Query));
            if let Some(field) = self
                .json_schema_field_factory
                .get_json_schema_field(&json_schema_field_name(relation_type.key()), &relation_type.type_definition())
            {
                fields.push(field);
            }
        }
        fields
    }

    fn create_query_field(&self, relation_type: &RelationType, root_object_type: RootObjectType) -> Field {
        let ty = relation_type.ty.clone();
        let relation_type_inner = relation_type.clone();
        let field_name = ty.type_name().to_case(Pascal);
        let mut field = Field::new(field_name, object_type_ref_list(&ty, root_object_type), move |ctx| {
            let relation_type = relation_type_inner.clone();
            FieldFuture::new(async move {
                let reactive_relation_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
                let reactive_relations = reactive_relation_manager.get_by_type(&relation_type.ty);
                let reactive_relations = filter_instances_by_properties(&ctx, &relation_type, reactive_relations);
                let reactive_relations = filter_instances_by_outbound_id(&ctx, reactive_relations)?;
                let reactive_relations = filter_instances_by_inbound_id(&ctx, reactive_relations)?;
                Ok(Some(FieldValue::list(reactive_relations.into_iter().map(FieldValue::owned_any))))
            })
        })
        .description(relation_type.description.clone())
        .argument(query_argument_outbound_id())
        .argument(query_argument_inbound_id());
        field = add_property_type_container_properties_as_field_arguments(field, relation_type, true, true);
        field
    }
}

#[async_trait]
impl Lifecycle for RelationQueryFieldFactoryImpl {}

fn filter_instances_by_outbound_id(ctx: &ResolverContext, mut instances: Vec<ReactiveRelation>) -> Result<Vec<ReactiveRelation>, Error> {
    if let Ok(outbound_id) = ctx.args.try_get("outbound_id") {
        let outbound_id = Uuid::from_str(outbound_id.string()?)?;
        instances.retain(|reactive_relation| reactive_relation.outbound.id == outbound_id);
    }
    Ok(instances)
}

fn filter_instances_by_inbound_id(ctx: &ResolverContext, mut instances: Vec<ReactiveRelation>) -> Result<Vec<ReactiveRelation>, Error> {
    if let Ok(inbound_id) = ctx.args.try_get("inbound_id") {
        let inbound_id = Uuid::from_str(inbound_id.string()?)?;
        instances.retain(|reactive_relation| reactive_relation.outbound.id == inbound_id);
    }
    Ok(instances)
}
