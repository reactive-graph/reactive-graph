use crate::json_schema_field_factory_impl::json_schema_field_name;
use crate::object_type_name::object_type_ref_list;
use crate::property_type_container::add_property_type_container_properties_as_field_arguments;
use crate::property_type_container::filter_instances_by_properties;
use crate::query_arguments::QUERY_ARGUMENT_ID;
use crate::query_arguments::QUERY_ARGUMENT_IDS;
use crate::query_arguments::QUERY_ARGUMENT_LABEL;
use crate::query_arguments::query_argument_id;
use crate::query_arguments::query_argument_ids;
use crate::query_arguments::query_argument_label;
use crate::sort::sort_by_key;
use async_graphql::Error;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_trait::async_trait;
use convert_case::Case::Pascal;
use convert_case::Casing;
use itertools::Itertools;
use reactive_graph_dynamic_graph_api::EntityQueryFieldFactory;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::QueryEntityInstanceError;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_type_system_api::EntityTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
pub struct EntityQueryFieldFactoryImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl EntityQueryFieldFactory for EntityQueryFieldFactoryImpl {
    fn create_query_fields(&self, namespace: &Namespace) -> Vec<Field> {
        let mut fields = Vec::new();
        for entity_type in self.entity_type_manager.get_by_namespace(&namespace).iter().sorted_by(sort_by_key) {
            fields.push(self.create_query_field(&entity_type, RootObjectType::Query));
            if let Some(field) = self
                .json_schema_field_factory
                .get_json_schema_field(&json_schema_field_name(entity_type.key()), &entity_type.type_definition())
            {
                fields.push(field);
            }
        }
        fields
    }

    fn create_query_field(&self, entity_type: &EntityType, root_object_type: RootObjectType) -> Field {
        let ty = entity_type.ty.clone();
        let entity_type_inner = entity_type.clone();
        // Within the namespace object the short type name is used
        let field_name = ty.type_name().to_case(Pascal);
        let mut field = Field::new(field_name, object_type_ref_list(&ty, root_object_type), move |ctx| {
            let ty = ty.clone();
            let entity_type = entity_type_inner.clone();
            FieldFuture::new(async move {
                let reactive_entity_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;

                // Select single instance by id and type
                if let Ok(id) = ctx.args.try_get(QUERY_ARGUMENT_ID) {
                    let id = Uuid::from_str(id.string()?)?;
                    let reactive_entity = reactive_entity_manager
                        .get(id)
                        .ok_or(QueryEntityInstanceError::EntityInstanceDoesNotExist(id))
                        .map_err(Error::new_with_source)?;
                    if reactive_entity.ty != ty {
                        return Err(QueryEntityInstanceError::EntityInstanceIsNotOfType(id, ty.clone()).into());
                    }
                    return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(reactive_entity)])));
                }
                // Select single instance by label and type
                if let Ok(label) = ctx.args.try_get(QUERY_ARGUMENT_LABEL) {
                    let label = label.string()?.to_string();
                    let reactive_entity = reactive_entity_manager
                        .get_by_label(&label)
                        .ok_or(QueryEntityInstanceError::EntityInstanceWithLabelDoesNotExist(label))?;
                    if reactive_entity.ty != ty {
                        return Err(QueryEntityInstanceError::EntityInstanceIsNotOfType(reactive_entity.id, ty.clone()).into());
                    }
                    return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(reactive_entity)])));
                }
                // Select instances by ids and type and properties
                if let Ok(ids) = ctx.args.try_get(QUERY_ARGUMENT_IDS) {
                    let mut instances = Vec::new();
                    for id in ids
                        .list()?
                        .iter()
                        .filter_map(|id| id.string().map(str::to_string).ok())
                        .filter_map(|id| Uuid::from_str(&id).ok())
                    {
                        if let Some(reactive_entity) = reactive_entity_manager.get(id) {
                            if reactive_entity.ty != ty {
                                return Err(QueryEntityInstanceError::EntityInstanceIsNotOfType(reactive_entity.id, ty.clone()).into());
                            }
                            instances.push(reactive_entity);
                        }
                    }
                    let instances = filter_instances_by_properties(&ctx, &entity_type, instances);
                    return Ok(Some(FieldValue::list(instances.into_iter().map(FieldValue::owned_any))));
                }
                // Select instances by type only
                let instances = reactive_entity_manager.get_by_type(&entity_type.ty);
                let instances = filter_instances_by_properties(&ctx, &entity_type, instances);
                Ok(Some(FieldValue::list(instances.into_iter().map(FieldValue::owned_any))))
            })
        })
        .description(entity_type.description.clone())
        .argument(query_argument_id())
        .argument(query_argument_ids())
        .argument(query_argument_label());
        field = add_property_type_container_properties_as_field_arguments(field, entity_type, true, true);
        field
    }
}

#[async_trait]
impl Lifecycle for EntityQueryFieldFactoryImpl {}
