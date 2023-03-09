use async_graphql::Schema;
use async_trait::async_trait;

use crate::api::ComponentManager;
use crate::api::EntityBehaviourManager;
use crate::api::EntityBehaviourRegistry;
use crate::api::EntityComponentBehaviourManager;
use crate::api::EntityComponentBehaviourRegistry;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::GraphQLSchemaManager;
use crate::api::InstanceService;
use crate::api::Lifecycle;
use crate::api::NamespaceManager;
use crate::api::PluginContainerManager;
use crate::api::PluginResolver;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveFlowInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationBehaviourManager;
use crate::api::RelationBehaviourRegistry;
use crate::api::RelationComponentBehaviourManager;
use crate::api::RelationComponentBehaviourRegistry;
use crate::api::RelationTypeManager;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Deferred;
use crate::di::Wrc;
use crate::graphql::directives;
use crate::graphql::InexorMutation;
use crate::graphql::InexorQuery;
use crate::graphql::InexorSchema;
use crate::graphql::InexorSubscription;

#[component]
pub struct GraphQLSchemaManagerImpl {
    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    flow_type_manager: Wrc<dyn FlowTypeManager>,

    namespace_manager: Wrc<dyn NamespaceManager>,

    entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,

    flow_instance_manager: Wrc<dyn ReactiveFlowInstanceManager>,

    entity_behaviour_registry: Wrc<dyn EntityBehaviourRegistry>,

    entity_component_behaviour_registry: Wrc<dyn EntityComponentBehaviourRegistry>,

    relation_behaviour_registry: Wrc<dyn RelationBehaviourRegistry>,

    relation_component_behaviour_registry: Wrc<dyn RelationComponentBehaviourRegistry>,

    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,

    entity_component_behaviour_manager: Wrc<dyn EntityComponentBehaviourManager>,

    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,

    relation_component_behaviour_manager: Wrc<dyn RelationComponentBehaviourManager>,

    plugin_container_manager: Wrc<dyn PluginContainerManager>,

    plugin_resolver: Deferred<Wrc<dyn PluginResolver>>,

    instance_service: Wrc<dyn InstanceService>,
}

impl GraphQLSchemaManagerImpl {}

#[async_trait]
#[provides]
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
            .data(self.plugin_container_manager.clone())
            .data(self.plugin_resolver.clone())
            .data(self.instance_service.clone())
            .directive(directives::concat)
            .directive(directives::random_uuid)
            .finish()
    }
}

impl Lifecycle for GraphQLSchemaManagerImpl {
    fn init(&self) {}

    fn post_init(&self) {}

    fn pre_shutdown(&self) {}

    fn shutdown(&self) {}
}
