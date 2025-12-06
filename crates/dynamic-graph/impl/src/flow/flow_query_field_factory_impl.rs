use crate::json_schema_field_factory_impl::json_schema_field_name;
use crate::object_type_name::object_type_ref_list;
use crate::property_type_container::add_property_type_container_properties_as_field_arguments;
use crate::property_type_container::filter_instances_by_properties;
use crate::query_arguments::QUERY_ARGUMENT_ID;
use crate::query_arguments::QUERY_ARGUMENT_LABEL;
use crate::query_arguments::query_argument_id;
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
use reactive_graph_dynamic_graph_api::FlowQueryFieldFactory;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::QueryFlowInstanceError;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
pub struct FlowQueryFieldFactoryImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl FlowQueryFieldFactory for FlowQueryFieldFactoryImpl {
    fn create_query_fields(&self, namespace: &Namespace) -> Vec<Field> {
        let mut fields = Vec::new();
        for flow_type in self.flow_type_manager.get_by_namespace(&namespace).iter().sorted_by(sort_by_key) {
            if let Ok(field) = self.create_query_field(&flow_type, RootObjectType::Query) {
                fields.push(field);
                if let Some(field) = self
                    .json_schema_field_factory
                    .get_json_schema_field(&json_schema_field_name(flow_type.key()), &flow_type.type_definition())
                {
                    fields.push(field);
                }
            }
        }
        fields
    }

    fn create_query_field(&self, flow_type: &FlowType, root_object_type: RootObjectType) -> Result<Field, ()> {
        let ty = flow_type.ty.clone();
        let wrapper_ty = flow_type.wrapper_type();
        let flow_type_inner = flow_type.clone();
        let wrapper_entity_type = self.entity_type_manager.get(&wrapper_ty).ok_or(())?;
        let wrapper_entity_type_inner = wrapper_entity_type.clone();
        let field_name = ty.type_name().to_case(Pascal);
        let mut field = Field::new(field_name, object_type_ref_list(&ty, root_object_type), move |ctx| {
            let flow_type = flow_type_inner.clone();
            let wrapper_entity_type = wrapper_entity_type_inner.clone();
            FieldFuture::new(async move {
                let reactive_flow_manager = ctx.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
                if let Ok(id) = ctx.args.try_get(QUERY_ARGUMENT_ID) {
                    let id = Uuid::from_str(id.string()?)?;
                    let reactive_flow = reactive_flow_manager
                        .get(id)
                        .ok_or(QueryFlowInstanceError::FlowInstanceDoesNotExist(id))
                        .map_err(Error::new_with_source)?;
                    if reactive_flow.ty != wrapper_entity_type.ty {
                        return Err(QueryFlowInstanceError::FlowInstanceIsNotOfType(id, wrapper_entity_type.ty.clone()).into());
                    }
                    return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(reactive_flow)])));
                }
                if let Ok(label) = ctx.args.try_get(QUERY_ARGUMENT_LABEL) {
                    let label = label.string()?.to_string();
                    let reactive_flow = reactive_flow_manager
                        .get_by_label(&label)
                        .ok_or(QueryFlowInstanceError::FlowInstanceWithLabelDoesNotExist(label))?;
                    if reactive_flow.ty != wrapper_entity_type.ty {
                        return Err(QueryFlowInstanceError::FlowInstanceIsNotOfType(reactive_flow.id, wrapper_entity_type.ty.clone()).into());
                    }
                    return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(reactive_flow)])));
                }
                let reactive_flows = reactive_flow_manager.get_by_type(&flow_type.ty);
                let reactive_flows = filter_instances_by_properties(&ctx, &wrapper_entity_type, reactive_flows);
                Ok(Some(FieldValue::list(reactive_flows.into_iter().map(FieldValue::owned_any))))
            })
        })
        .description(flow_type.description.clone())
        .argument(query_argument_id())
        .argument(query_argument_label());
        field = add_property_type_container_properties_as_field_arguments(field, &wrapper_entity_type, true, true);
        Ok(field)
    }
}

#[async_trait]
impl Lifecycle for FlowQueryFieldFactoryImpl {}
