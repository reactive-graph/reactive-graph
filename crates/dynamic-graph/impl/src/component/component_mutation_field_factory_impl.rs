use crate::sort::sort_by_key;
use async_graphql::dynamic::Field;
use async_trait::async_trait;
use itertools::Itertools;
use reactive_graph_dynamic_graph_api::ComponentMutationFieldFactory;
use reactive_graph_dynamic_graph_api::ComponentQueryFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::Namespace;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct ComponentMutationFieldFactoryImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    component_query_field_factory: Arc<dyn ComponentQueryFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl ComponentMutationFieldFactory for ComponentMutationFieldFactoryImpl {
    fn create_mutation_fields(&self, namespace: &Namespace) -> Vec<Field> {
        let mut fields = Vec::new();
        for component in self.component_manager.get_by_namespace(&namespace).iter().sorted_by(sort_by_key) {
            fields.push(
                self.component_query_field_factory
                    .create_query_field(&component, RootObjectType::Mutation, RootObjectType::Interface),
            );
        }
        fields
    }
}

#[async_trait]
impl Lifecycle for ComponentMutationFieldFactoryImpl {}
