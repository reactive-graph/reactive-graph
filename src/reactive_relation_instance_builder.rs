use std::sync::Arc;

use serde_json::Value;

use crate::model::ReactiveRelationInstance;
use crate::model::RelationType;
use crate::RelationInstanceBuilder;
use inexor_rgf_core_model::ReactiveEntityInstance;

#[allow(dead_code)]
pub struct ReactiveRelationInstanceBuilder {
    outbound: Arc<ReactiveEntityInstance>,
    type_name: String,
    inbound: Arc<ReactiveEntityInstance>,
    // properties: HashMap<String, Value>,
    builder: RelationInstanceBuilder,
}

#[allow(dead_code)]
impl ReactiveRelationInstanceBuilder {
    pub fn new<S: Into<String>>(
        outbound: Arc<ReactiveEntityInstance>,
        type_name: S,
        inbound: Arc<ReactiveEntityInstance>,
    ) -> ReactiveRelationInstanceBuilder {
        let type_name: String = type_name.into();
        let builder = RelationInstanceBuilder::new(outbound.id, type_name.clone(), inbound.id);
        ReactiveRelationInstanceBuilder {
            outbound,
            type_name: type_name.clone(),
            inbound,
            // properties: HashMap::new(),
            builder,
        }
    }

    pub fn property<'a, S: Into<String>>(
        &'a mut self,
        property_name: S,
        value: Value,
    ) -> &'a mut ReactiveRelationInstanceBuilder {
        self.builder.property(property_name.into(), value);
        // self.properties.insert(property_name.into(), value);
        self
    }

    pub fn set_properties_defaults<'a>(
        &'a mut self,
        relation_type: RelationType,
    ) -> &'a mut ReactiveRelationInstanceBuilder {
        for property_type in relation_type.properties {
            self.property(
                property_type.name.clone(),
                property_type.data_type.default_value(),
            );
        }
        self
    }

    pub fn get(&mut self) -> Arc<ReactiveRelationInstance> {
        Arc::new(ReactiveRelationInstance::from_instance(
            self.outbound.clone(),
            self.inbound.clone(),
            self.builder.get(),
        ))
    }
}
