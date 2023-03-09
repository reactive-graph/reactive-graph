use std::sync::Arc;

use serde_json::Value;

use crate::model::ComponentContainer;
use crate::model::ComponentTypeId;
use crate::model::PropertyTypeDefinition;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationInstanceTypeId;
use crate::model::RelationType;
use crate::model::RelationTypeId;
use crate::RelationInstanceBuilder;

#[allow(dead_code)]
pub struct ReactiveRelationInstanceBuilder {
    outbound: Arc<ReactiveEntityInstance>,
    ty: RelationInstanceTypeId,
    inbound: Arc<ReactiveEntityInstance>,
    components: Vec<ComponentTypeId>,
    builder: RelationInstanceBuilder,
}

#[allow(dead_code)]
impl ReactiveRelationInstanceBuilder {
    pub fn new<RIT: Into<RelationInstanceTypeId>>(
        outbound: Arc<ReactiveEntityInstance>,
        ty: RIT,
        inbound: Arc<ReactiveEntityInstance>,
    ) -> ReactiveRelationInstanceBuilder {
        let ty = ty.into();
        let builder = RelationInstanceBuilder::new(outbound.id, ty.clone(), inbound.id);
        ReactiveRelationInstanceBuilder {
            outbound,
            ty,
            inbound,
            components: Vec::new(),
            builder,
        }
    }

    pub fn new_unique_id<RT: Into<RelationTypeId>>(
        outbound: Arc<ReactiveEntityInstance>,
        ty: RT,
        inbound: Arc<ReactiveEntityInstance>,
    ) -> ReactiveRelationInstanceBuilder {
        let ty = RelationInstanceTypeId::new_unique_id(ty.into());
        let builder = RelationInstanceBuilder::new(outbound.id, ty.clone(), inbound.id);
        ReactiveRelationInstanceBuilder {
            outbound,
            ty,
            inbound,
            components: Vec::new(),
            builder,
        }
    }

    pub fn new_unique_for_instance_id<RT: Into<RelationTypeId>, S: Into<String>>(
        outbound: Arc<ReactiveEntityInstance>,
        ty: RT,
        instance_id: S,
        inbound: Arc<ReactiveEntityInstance>,
    ) -> ReactiveRelationInstanceBuilder {
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(ty.into(), instance_id);
        let builder = RelationInstanceBuilder::new(outbound.id, ty.clone(), inbound.id);
        ReactiveRelationInstanceBuilder {
            outbound,
            ty,
            inbound,
            components: Vec::new(),
            builder,
        }
    }

    pub fn new_with_random_instance_id<RT: Into<RelationTypeId>>(
        outbound: Arc<ReactiveEntityInstance>,
        ty: RT,
        inbound: Arc<ReactiveEntityInstance>,
    ) -> ReactiveRelationInstanceBuilder {
        let ty = RelationInstanceTypeId::new_with_random_instance_id(ty.into());
        let builder = RelationInstanceBuilder::new(outbound.id, ty.clone(), inbound.id);
        ReactiveRelationInstanceBuilder {
            outbound,
            ty,
            inbound,
            components: Vec::new(),
            builder,
        }
    }

    pub fn new_from_type_unique_id<S: Into<String>>(
        outbound: Arc<ReactiveEntityInstance>,
        namespace: S,
        type_name: S,
        inbound: Arc<ReactiveEntityInstance>,
    ) -> ReactiveRelationInstanceBuilder {
        ReactiveRelationInstanceBuilder::new(outbound, RelationInstanceTypeId::new_from_type_unique_id(namespace, type_name), inbound)
    }

    pub fn new_from_type_unique_for_instance_id<S: Into<String>>(
        outbound: Arc<ReactiveEntityInstance>,
        namespace: S,
        type_name: S,
        instance_id: S,
        inbound: Arc<ReactiveEntityInstance>,
    ) -> ReactiveRelationInstanceBuilder {
        ReactiveRelationInstanceBuilder::new(
            outbound,
            RelationInstanceTypeId::new_from_type_unique_for_instance_id(namespace, type_name, instance_id),
            inbound,
        )
    }

    pub fn new_from_type_with_random_instance_id<S: Into<String>>(
        outbound: Arc<ReactiveEntityInstance>,
        namespace: S,
        type_name: S,
        inbound: Arc<ReactiveEntityInstance>,
    ) -> ReactiveRelationInstanceBuilder {
        ReactiveRelationInstanceBuilder::new(outbound, RelationInstanceTypeId::new_from_type_with_random_instance_id(namespace, type_name), inbound)
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, value: Value) -> &mut ReactiveRelationInstanceBuilder {
        self.builder.property(property_name.into(), value);
        self
    }

    pub fn property_with_default(&mut self, property: Box<dyn PropertyTypeDefinition>) -> &mut ReactiveRelationInstanceBuilder {
        self.builder.property_with_default(property);
        self
    }

    pub fn set_properties_defaults(&mut self, relation_type: RelationType) -> &mut ReactiveRelationInstanceBuilder {
        for property_type in relation_type.properties {
            self.property(property_type.name.clone(), property_type.data_type.default_value());
        }
        self
    }

    pub fn component<C: Into<ComponentTypeId>>(&mut self, ty: C) -> &mut ReactiveRelationInstanceBuilder {
        self.components.push(ty.into());
        self
    }

    pub fn build(&self) -> Arc<ReactiveRelationInstance> {
        let relation_instance = ReactiveRelationInstance::new_from_instance(self.outbound.clone(), self.inbound.clone(), self.builder.build());
        for component in self.components.iter() {
            relation_instance.add_component(component.clone());
        }
        Arc::new(relation_instance)
    }
}
