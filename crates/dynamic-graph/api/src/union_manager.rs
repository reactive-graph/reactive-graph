use async_graphql::dynamic::Union;
use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;

pub const UNION_ALL_ENTITIES: &str = "AllEntities";
pub const UNION_NAMESPACE_ENTITIES_SUFFIX: &str = "Entities";

pub const UNION_ALL_RELATIONS: &str = "AllRelations";
pub const UNION_NAMESPACE_RELATIONS_SUFFIX: &str = "Relations";

pub const UNION_ALL_FLOWS: &str = "AllFlows";
pub const UNION_NAMESPACE_FLOWS_SUFFIX: &str = "Flows";

#[injectable]
#[async_trait]
pub trait UnionManager: Send + Sync + Lifecycle {
    /// Returns the unions of the dynamic graph.
    fn get_unions(&self) -> Vec<Union>;

    /// Returns the union for all entity types.
    fn all_entities(&self) -> Union;

    /// Returns the union for all relation types.
    fn all_relations(&self) -> Union;

    /// Returns the union for all flow types.
    fn all_flows(&self) -> Union;
}
