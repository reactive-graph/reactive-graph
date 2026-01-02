use crate::field::reactive_instance::reactive_instance_component;
use crate::json_schema_field_factory_impl::json_schema_field_name;
use crate::object_type_name::object_type_ref_list;
use crate::property_type_container::add_property_type_container_properties_as_field_arguments;
use crate::property_type_container::filter_instances_by_properties;
use crate::sort::sort_by_key;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_trait::async_trait;
use convert_case::Case::Pascal;
use convert_case::Casing;
use itertools::Itertools;
use log::info;
use log::trace;
use reactive_graph_dynamic_graph_api::ComponentQueryFieldFactory;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::Component;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_type_system_api::ComponentManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct ComponentQueryFieldFactoryImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl ComponentQueryFieldFactory for ComponentQueryFieldFactoryImpl {
    fn create_query_fields(&self, namespace: &Namespace) -> Vec<Field> {
        let mut fields = Vec::new();
        for component in self.component_manager.get_by_namespace(&namespace).iter().sorted_by(sort_by_key) {
            fields.push(self.create_query_field(&component, RootObjectType::Interface, RootObjectType::Query));
            if let Some(field) = self
                .json_schema_field_factory
                .get_json_schema_field(&json_schema_field_name(component.key()), &component.type_definition())
            {
                fields.push(field);
            }
        }
        fields
    }

    fn create_query_field(&self, component: &Component, interface_root_object_type: RootObjectType, instance_root_object_type: RootObjectType) -> Field {
        let ty = component.ty.clone();
        let component_inner = component.clone();
        let field_name = ty.type_name().to_case(Pascal);
        trace!("Create {instance_root_object_type} field for {interface_root_object_type} for {}", &component.ty);
        let mut field = Field::new(field_name, object_type_ref_list(&ty, interface_root_object_type), move |ctx| {
            let ty = ty.clone();
            let component = component_inner.clone();
            FieldFuture::new(async move {
                let reactive_entity_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
                let reactive_relation_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
                let reactive_entities = reactive_entity_manager.get_by_component(&ty);
                let reactive_entities = filter_instances_by_properties(&ctx, &component, reactive_entities);
                info!("reactive_entities; {}", reactive_entities.len());
                let reactive_entities = reactive_entities
                    .into_iter()
                    .map(|reactive_entity| reactive_instance_component(reactive_entity, &ty, instance_root_object_type));
                let reactive_relations = reactive_relation_manager.get_by_component(&ty);
                let reactive_relations = filter_instances_by_properties(&ctx, &component, reactive_relations);
                info!("reactive_relations; {}", reactive_relations.len());
                let reactive_relations = reactive_relations
                    .into_iter()
                    .map(|reactive_relation| reactive_instance_component(reactive_relation, &ty, instance_root_object_type));
                let field_values = reactive_entities.chain(reactive_relations);
                let field_values1 = field_values.clone();
                info!("{}", field_values1.count());
                // The field value stores the dyn trait (Arc::<Box<dyn ReactiveInstanceUnidentifiable>>)
                Ok(Some(FieldValue::list(field_values)))
            })
        })
        .description(component.description.clone());
        field = add_property_type_container_properties_as_field_arguments(field, component, true, true);
        field
    }
}

#[async_trait]
impl Lifecycle for ComponentQueryFieldFactoryImpl {}
