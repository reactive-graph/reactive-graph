use crate::object_type_name::object_type_name;
use async_graphql::dynamic::Union;
use async_trait::async_trait;
use log::trace;
use reactive_graph_dynamic_graph_api::RootObjectType::Query;
use reactive_graph_dynamic_graph_api::UNION_ALL_ENTITIES;
use reactive_graph_dynamic_graph_api::UNION_ALL_FLOWS;
use reactive_graph_dynamic_graph_api::UNION_ALL_RELATIONS;
use reactive_graph_dynamic_graph_api::UnionManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct UnionManagerImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
}

impl UnionManagerImpl {}

#[async_trait]
#[component_alias]
impl UnionManager for UnionManagerImpl {
    fn get_unions(&self) -> Vec<Union> {
        vec![self.all_entities(), self.all_relations(), self.all_flows()]
    }

    fn all_entities(&self) -> Union {
        let mut union = Union::new(UNION_ALL_ENTITIES).description("Any entity.");
        for ty in self.entity_type_manager.get_type_ids() {
            let object_type_name = object_type_name(&ty, Query);
            trace!("AllEntities += {object_type_name}");
            union = union.possible_type(object_type_name);
        }
        union
    }

    fn all_relations(&self) -> Union {
        let mut union = Union::new(UNION_ALL_RELATIONS).description("Any relation.");
        for ty in self.relation_type_manager.get_type_ids() {
            let object_type_name = object_type_name(&ty, Query);
            trace!("AllRelations += {object_type_name}");
            union = union.possible_type(object_type_name);
        }
        union
    }

    fn all_flows(&self) -> Union {
        let mut union = Union::new(UNION_ALL_FLOWS).description("Any flow.");
        for ty in self.flow_type_manager.get_type_ids() {
            let object_type_name = object_type_name(&ty, Query);
            trace!("AllFlows += {object_type_name}");
            union = union.possible_type(object_type_name);
        }
        union
    }

    // TODO: Make it possible to define unions in the type system
    // struct Union(NamespacedTypes),
    // let union = Union::new(namespaced_types);
    // TODO: Make it possible to allow InboundOutboundTypes with a list of entities:
    // InboundOutboundTypes(Union(union))
}

#[async_trait]
impl Lifecycle for UnionManagerImpl {}
