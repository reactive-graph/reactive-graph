use reactive_graph_behaviour_model_api::prelude::*;

use reactive_graph_graph::RelationInstanceId;
use reactive_graph_reactive_model_api::ReactiveInstanceContainer;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_reactive_model_impl::ReactiveRelation;

pub trait RelationPropertyValidator: ReactiveInstanceContainer<RelationInstanceId, ReactiveRelation> {
    /// Validates the outbound property with the given name.
    fn validate_outbound_property(&self, property_name: &str) -> Result<(), BehaviourPropertyInvalid> {
        if !self.get_reactive_instance().outbound.has_property(property_name) {
            return Err(BehaviourPropertyInvalid::OutboundPropertyMissing(property_name.to_owned()));
        }
        Ok(())
    }

    /// Validates the inbound property with the given name.
    fn validate_inbound_property(&self, property_name: &str) -> Result<(), BehaviourPropertyInvalid> {
        if !self.get_reactive_instance().inbound.has_property(property_name) {
            return Err(BehaviourPropertyInvalid::InboundPropertyMissing(property_name.to_owned()));
        }
        Ok(())
    }
}
