use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_behaviour_model_api::BehaviourTransitionError;
use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_model_api::ComponentBehaviourTypeId;
use reactive_graph_graph::Component;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveRelation;

#[injectable]
#[async_trait]
pub trait RelationComponentBehaviourManager: Send + Sync + Lifecycle {
    /// Adds new behaviours to the given relation instance.
    fn add_behaviours_to_relation(&self, relation_instance: ReactiveRelation);

    /// Possibly adds new behaviour to the given relation instance's component
    fn add_behaviours_to_relation_component(&self, relation_instance: ReactiveRelation, component: Component);

    /// Creates and adds the given behaviour to the given reactive entity instance's component.
    fn add_behaviour_to_relation_component(&self, relation_instance: ReactiveRelation, component_behaviour_ty: &ComponentBehaviourTypeId);

    /// Removes the given behaviour from the given reactive relation instance.
    fn remove_behaviour_from_relation(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId);

    /// Removes behaviours from the given relation instance.
    fn remove_behaviours_from_relation(&self, relation_instance: ReactiveRelation);

    /// Removes behaviour from the given relation instance's component
    fn remove_behaviours_from_relation_component(&self, relation_instance: ReactiveRelation, component: Component);

    /// Removes behaviours from the given relation instance by edge key.
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
