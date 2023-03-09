use serde_json::Value;

use crate::model::DataType;
use crate::model::EntityInstance;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::FlowType;
use crate::model::FlowTypeId;
use crate::model::PropertyType;
use crate::model::RelationInstance;

#[allow(dead_code)]
pub struct FlowTypeBuilder {
    ty: FlowTypeId,
    description: String,
    wrapper_entity_instance: EntityInstance,
    entity_instances: Vec<EntityInstance>,
    relation_instances: Vec<RelationInstance>,
    variables: Vec<PropertyType>,
    extensions: Vec<Extension>,
}

#[allow(dead_code)]
impl FlowTypeBuilder {
    pub fn new<FT: Into<FlowTypeId>>(ty: FT, wrapper_entity_instance: EntityInstance) -> FlowTypeBuilder {
        FlowTypeBuilder {
            ty: ty.into(),
            description: String::new(),
            wrapper_entity_instance,
            entity_instances: Vec::new(),
            relation_instances: Vec::new(),
            variables: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S, wrapper_entity_instance: EntityInstance) -> FlowTypeBuilder {
        FlowTypeBuilder::new(FlowTypeId::new_from_type(namespace, type_name), wrapper_entity_instance)
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut FlowTypeBuilder {
        self.description = description.into();
        self
    }

    pub fn wrapper_entity_instance(&mut self, entity_instance: EntityInstance) -> &mut FlowTypeBuilder {
        self.entity_instances.push(entity_instance);
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

    pub fn extension<S: Into<String>>(&mut self, namespace: S, name: S, extension: Value) -> &mut FlowTypeBuilder {
        let ty = ExtensionTypeId::new_from_type(namespace.into(), name.into());
        self.extensions.push(Extension {
            ty,
            description: Default::default(),
            extension,
        });
        self
    }

    pub fn build(&self) -> FlowType {
        FlowType::new(
            self.ty.clone(),
            self.description.clone(),
            self.wrapper_entity_instance.clone(),
            self.entity_instances.to_vec(),
            self.relation_instances.to_vec(),
            self.variables.to_vec(),
            self.extensions.to_vec(),
        )
    }
}
