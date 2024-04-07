use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_behaviour_model_api::BehaviourTransitionError;
use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveRelation;

#[injectable]
#[async_trait]
pub trait RelationBehaviourManager: Send + Sync + Lifecycle {
    /// Adds all behaviours to the given reactive relation instance.
    fn add_behaviours(&self, relation_instance: ReactiveRelation);

    /// Creates and adds the given behaviour to the given reactive entity instance.
    fn add_behaviour(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId);

    /// Removes the given behaviour from the given reactive relation instance.
    fn remove_behaviour(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId);

    /// Removes all behaviours from the given reactive relation instance.
    fn remove_behaviours(&self, relation_instance: ReactiveRelation);

    /// Removes all behaviours from the reactive relation instance with the given edge key.
    fn remove_behaviours_by_key(&self, edge_key: &RelationInstanceId);

    /// Removes all behaviours of the given behaviour type.
    fn remove_behaviours_by_behaviour(&self, behaviour_ty: &BehaviourTypeId);

    /// Returns true, if the relation instance has the given behaviour.
    fn has(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> bool;

    /// Returns the behaviours of the given relation instance.
    fn get_all(&self, relation_instance: ReactiveRelation) -> Vec<BehaviourTypeId>;

    /// Returns the relation instances with the given behaviour.
    fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<ReactiveRelation>;

    /// Connect
    fn connect(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Disconnect
    fn disconnect(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Reconnect
    fn reconnect(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;
}
