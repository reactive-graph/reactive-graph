use serde_json::Value;

use crate::model::DataType;
use crate::model::EntityInstance;
use crate::model::Extension;
use crate::model::FlowType;
use crate::model::PropertyType;
use crate::model::RelationInstance;

#[allow(dead_code)]
pub struct FlowTypeBuilder {
    type_name: String,
    name: String,
    namespace: String,
    description: String,
    entity_instances: Vec<EntityInstance>,
    relation_instances: Vec<RelationInstance>,
    variables: Vec<PropertyType>,
    extensions: Vec<Extension>,
}

#[allow(dead_code)]
impl FlowTypeBuilder {
    pub fn new<S: Into<String>>(type_name: S, name: S) -> FlowTypeBuilder {
        FlowTypeBuilder {
            type_name: type_name.into(),
            name: name.into(),
            namespace: String::new(),
            description: String::new(),
            entity_instances: Vec::new(),
            relation_instances: Vec::new(),
            variables: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn namespace<S: Into<String>>(&mut self, namespace: S) -> &mut FlowTypeBuilder {
        self.namespace = namespace.into();
        self
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut FlowTypeBuilder {
        self.description = description.into();
        self
    }

    pub fn entity_instance(&mut self, entity_instance: EntityInstance) -> &mut FlowTypeBuilder {
        self.entity_instances.push(entity_instance);
        self
    }

    pub fn relation_instance(&mut self, relation_instance: RelationInstance) -> &mut FlowTypeBuilder {
        self.relation_instances.push(relation_instance);
        self
    }

    pub fn variable<S: Into<String>>(&mut self, variable_name: S, data_type: DataType) -> &mut FlowTypeBuilder {
        self.variables.push(PropertyType::new(variable_name.into(), data_type));
        self
    }

    pub fn extension<S: Into<String>>(&mut self, name: S, extension: Value) -> &mut FlowTypeBuilder {
        self.extensions.push(Extension { name: name.into(), extension });
        self
    }

    pub fn build(&mut self) -> FlowType {
        FlowType::new(
            self.type_name.clone(),
            self.name.clone(),
            self.namespace.clone(),
            self.description.clone(),
            self.entity_instances.to_vec(),
            self.relation_instances.to_vec(),
            self.variables.to_vec(),
            self.extensions.to_vec(),
        )
    }
}
