use async_graphql::Schema;
use async_trait::async_trait;

use crate::api::ComponentManager;
use crate::api::DynamicGraph;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::GraphQLSchemaManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveFlowInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationTypeManager;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;
use crate::graphql::directives;
use crate::graphql::InexorMutation;
use crate::graphql::InexorQuery;
use crate::graphql::InexorSchema;
use crate::graphql::InexorSubscription;

#[component]
pub struct GraphQLSchemaManagerImpl {
    component_manager: Wrc<dyn ComponentManager>,

    dynamic_graph: Wrc<dyn DynamicGraph>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    flow_type_manager: Wrc<dyn FlowTypeManager>,

    entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,

    flow_manager: Wrc<dyn ReactiveFlowInstanceManager>,
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
            .data(self.entity_instance_manager.clone())
            .data(self.relation_instance_manager.clone())
            .data(self.flow_manager.clone())
            .data(self.dynamic_graph.clone())
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
