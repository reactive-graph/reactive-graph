use crate::field::create::FIELD_NAME_PREFIX_CREATE;
use crate::field::property_instance::create_properties_from_field_arguments;
use crate::object_type_name::object_type_ref_list;
use crate::property_type_container::add_property_type_container_properties_as_field_arguments;
use crate::property_type_container::add_variables_container_properties_as_field_arguments;
use crate::query_arguments::QUERY_ARGUMENT_ID;
use crate::query_arguments::query_argument_id;
use crate::sort::sort_by_key;
use async_graphql::Error;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_trait::async_trait;
use convert_case::Case::Pascal;
use convert_case::Casing;
use itertools::Itertools;
use reactive_graph_dynamic_graph_api::FlowMutationFieldFactory;
use reactive_graph_dynamic_graph_api::FlowQueryFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_dynamic_graph_api::RootObjectType::Query;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_service_api::ReactiveFlowCreationError;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
pub struct FlowMutationFieldFactoryImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
    flow_query_field_factory: Arc<dyn FlowQueryFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl FlowMutationFieldFactory for FlowMutationFieldFactoryImpl {
    fn create_mutation_fields(&self, namespace: &Namespace) -> Vec<Field> {
        let mut fields = Vec::new();
        for flow_type in self.flow_type_manager.get_by_namespace(&namespace).iter().sorted_by(sort_by_key) {
            if let Ok(field) = self.flow_query_field_factory.create_query_field(&flow_type, RootObjectType::Mutation) {
                fields.push(field);
            }
        }
        fields
    }

    // TODO: improve error handling
    fn create_creation_field(&self, flow_type: &FlowType) -> Result<Field, ()> {
        let ty = flow_type.ty.clone();
        let wrapper_ty = flow_type.wrapper_type();
        let flow_type_inner = flow_type.clone();
        let wrapper_entity_type = self.entity_type_manager.get(&wrapper_ty).ok_or(())?;
        let wrapper_entity_type_inner = wrapper_entity_type.clone();
        let field_name = format!("{}{}", FIELD_NAME_PREFIX_CREATE, ty.type_name().to_case(Pascal));
        let mut field = Field::new(field_name, object_type_ref_list(&ty, Query), move |ctx| {
            let ty = flow_type_inner.ty.clone();
            let flow_type = flow_type_inner.clone();
            let wrapper_entity_type = wrapper_entity_type_inner.clone();
            FieldFuture::new(async move {
                let flow_instance_manager = ctx.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
                let id = ctx
                    .args
                    .get(QUERY_ARGUMENT_ID)
                    .and_then(|id| id.string().ok().and_then(|s| Uuid::from_str(s).ok()));
                if let Some(id) = id {
                    if flow_instance_manager.has(id) {
                        return Err(ReactiveFlowCreationError::UuidTaken(id).into());
                    }
                }

                let variables = create_properties_from_field_arguments(&ctx, &flow_type.variables, false)?;
                let properties = create_properties_from_field_arguments(&ctx, &wrapper_entity_type.properties, true)?;

                match flow_instance_manager.create_from_type(&ty, id, variables, properties) {
                    Ok(reactive_flow) => Ok(Some(FieldValue::owned_any(reactive_flow))),
                    Err(e) => Err(Error::new(format!("Failed to create reactive flow: {e:?}"))),
                }
            })
        })
        .description(format!("Create a new {} flow", flow_type.type_name()))
        .argument(query_argument_id());
        field = add_variables_container_properties_as_field_arguments(field, flow_type, false, false);
        field = add_property_type_container_properties_as_field_arguments(field, &wrapper_entity_type, true, true);
        Ok(field)
    }
}

#[async_trait]
impl Lifecycle for FlowMutationFieldFactoryImpl {}
