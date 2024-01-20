use std::sync::Arc;

use async_graphql::Schema;
use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use inexor_rgf_behaviour_service_api::EntityBehaviourManager;
use inexor_rgf_behaviour_service_api::EntityBehaviourRegistry;
use inexor_rgf_behaviour_service_api::EntityComponentBehaviourManager;
use inexor_rgf_behaviour_service_api::EntityComponentBehaviourRegistry;
use inexor_rgf_behaviour_service_api::RelationBehaviourManager;
use inexor_rgf_behaviour_service_api::RelationBehaviourRegistry;
use inexor_rgf_behaviour_service_api::RelationComponentBehaviourManager;
use inexor_rgf_behaviour_service_api::RelationComponentBehaviourRegistry;
use inexor_rgf_graphql_api::GraphQLSchemaManager;
use inexor_rgf_graphql_schema::directives;
use inexor_rgf_graphql_schema::InexorMutation;
use inexor_rgf_graphql_schema::InexorQuery;
use inexor_rgf_graphql_schema::InexorSchema;
use inexor_rgf_graphql_schema::InexorSubscription;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_service_api::ReactiveEntityManager;
use inexor_rgf_reactive_service_api::ReactiveFlowManager;
use inexor_rgf_reactive_service_api::ReactiveRelationManager;
use inexor_rgf_type_system_api::ComponentManager;
use inexor_rgf_type_system_api::EntityTypeManager;
use inexor_rgf_type_system_api::FlowTypeManager;
use inexor_rgf_type_system_api::NamespaceManager;
use inexor_rgf_type_system_api::RelationTypeManager;

#[derive(Component)]
pub struct GraphQLSchemaManagerImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,

    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,

    namespace_manager: Arc<dyn NamespaceManager + Send + Sync>,

    entity_instance_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,

    relation_instance_manager: Arc<dyn ReactiveRelationManager + Send + Sync>,

    flow_instance_manager: Arc<dyn ReactiveFlowManager + Send + Sync>,

    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,

    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,

    relation_behaviour_registry: Arc<dyn RelationBehaviourRegistry + Send + Sync>,

    relation_component_behaviour_registry: Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>,

    entity_behaviour_manager: Arc<dyn EntityBehaviourManager + Send + Sync>,

    entity_component_behaviour_manager: Arc<dyn EntityComponentBehaviourManager + Send + Sync>,

    relation_behaviour_manager: Arc<dyn RelationBehaviourManager + Send + Sync>,

    relation_component_behaviour_manager: Arc<dyn RelationComponentBehaviourManager + Send + Sync>,
}

impl GraphQLSchemaManagerImpl {}

#[async_trait]
#[component_alias]
impl GraphQLSchemaManager for GraphQLSchemaManagerImpl {
    fn get_schema(&self) -> InexorSchema {
        Schema::build(InexorQuery, InexorMutation, InexorSubscription)
            .data(self.component_manager.clone())
            .data(self.entity_type_manager.clone())
            .data(self.relation_type_manager.clone())
            .data(self.flow_type_manager.clone())
            .data(self.namespace_manager.clone())
            .data(self.entity_instance_manager.clone())
            .data(self.relation_instance_manager.clone())
            .data(self.flow_instance_manager.clone())
            .data(self.entity_behaviour_registry.clone())
            .data(self.entity_component_behaviour_registry.clone())
            .data(self.relation_behaviour_registry.clone())
            .data(self.relation_component_behaviour_registry.clone())
            .data(self.entity_behaviour_manager.clone())
            .data(self.entity_component_behaviour_manager.clone())
            .data(self.relation_behaviour_manager.clone())
            .data(self.relation_component_behaviour_manager.clone())
            .directive(directives::concat)
            .directive(directives::random_uuid)
            .finish()
    }
}

#[async_trait]
impl Lifecycle for GraphQLSchemaManagerImpl {
    async fn init(&self) {}

    async fn post_init(&self) {}

    async fn pre_shutdown(&self) {}

    async fn shutdown(&self) {}
}
