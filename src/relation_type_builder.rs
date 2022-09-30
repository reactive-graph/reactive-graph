use serde_json::Value;

use crate::model::DataType;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::model::RelationType;

#[allow(dead_code)]
pub struct RelationTypeBuilder {
    namespace: String,
    outbound_type: String,
    type_name: String,
    inbound_type: String,
    description: String,
    components: Vec<String>,
    properties: Vec<PropertyType>,
    extensions: Vec<Extension>,
}

#[allow(dead_code)]
impl RelationTypeBuilder {
    pub fn new<S: Into<String>>(namespace: S, outbound_type: S, type_name: S, inbound_type: S) -> RelationTypeBuilder {
        RelationTypeBuilder {
            namespace: namespace.into(),
            outbound_type: outbound_type.into(),
            type_name: type_name.into(),
            inbound_type: inbound_type.into(),
            description: String::new(),
            components: Vec::new(),
            properties: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut RelationTypeBuilder {
        self.description = description.into();
        self
    }

    pub fn component<S: Into<String>>(&mut self, component_name: S) -> &mut RelationTypeBuilder {
        self.components.push(component_name.into());
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), data_type));
        self
    }

    pub fn input_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::input(property_name.into(), data_type));
        self
    }

    pub fn output_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::output(property_name.into(), data_type));
        self
    }

    pub fn property_from<S: Into<PropertyType>>(&mut self, property_type: S) -> &mut RelationTypeBuilder {
        self.properties.push(property_type.into());
        self
    }

    pub fn string_property<S: Into<String>>(&mut self, property_name: S) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::String));
        self
    }

    pub fn bool_property<S: Into<String>>(&mut self, property_name: S) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Bool));
        self
    }

    pub fn number_property<S: Into<String>>(&mut self, property_name: S) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Number));
        self
    }

    pub fn array_property<S: Into<String>>(&mut self, property_name: S) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Array));
        self
    }

    pub fn object_property<S: Into<String>>(&mut self, property_name: S) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Object));
        self
    }

    pub fn extension<S: Into<String>>(&mut self, name: S, extension: Value) -> &mut RelationTypeBuilder {
        self.extensions.push(Extension { name: name.into(), extension });
        self
    }

    pub fn build(&self) -> RelationType {
        RelationType::new(
            self.namespace.clone(),
            self.outbound_type.clone(),
            self.type_name.clone(),
            self.inbound_type.clone(),
            self.description.clone(),
            self.components.to_vec(),
            self.properties.to_vec(),
            self.extensions.to_vec(),
        )
    }
}
