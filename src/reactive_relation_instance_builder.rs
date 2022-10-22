use std::sync::Arc;

use serde_json::Value;

use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationType;
use crate::model::RelationTypeType;
use crate::RelationInstanceBuilder;

#[allow(dead_code)]
pub struct ReactiveRelationInstanceBuilder {
    outbound: Arc<ReactiveEntityInstance>,
    ty: RelationTypeType,
    inbound: Arc<ReactiveEntityInstance>,
    builder: RelationInstanceBuilder,
}

#[allow(dead_code)]
impl ReactiveRelationInstanceBuilder {
    pub fn new(outbound: Arc<ReactiveEntityInstance>, ty: RelationTypeType, inbound: Arc<ReactiveEntityInstance>) -> ReactiveRelationInstanceBuilder {
        let builder = RelationInstanceBuilder::new(outbound.id, ty.clone(), inbound.id);
        ReactiveRelationInstanceBuilder {
            outbound,
            ty,
            inbound,
            builder,
        }
    }

    pub fn new_from_type<S: Into<String>>(
        namespace: S,
        outbound: Arc<ReactiveEntityInstance>,
        type_name: S,
        inbound: Arc<ReactiveEntityInstance>,
    ) -> ReactiveRelationInstanceBuilder {
        ReactiveRelationInstanceBuilder::new(outbound, RelationTypeType::new_from_type(namespace, type_name), inbound)
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, value: Value) -> &mut ReactiveRelationInstanceBuilder {
        self.builder.property(property_name.into(), value);
        self
    }

    pub fn set_properties_defaults(&mut self, relation_type: RelationType) -> &mut ReactiveRelationInstanceBuilder {
        for property_type in relation_type.properties {
            self.property(property_type.name.clone(), property_type.data_type.default_value());
        }
        self
    }

    pub fn build(&self) -> Arc<ReactiveRelationInstance> {
        Arc::new(ReactiveRelationInstance::new_from_instance(self.outbound.clone(), self.inbound.clone(), self.builder.build()))
    }
}
