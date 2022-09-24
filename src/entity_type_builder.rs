use serde_json::Value;

use crate::model::DataType;
use crate::model::EntityType;
use crate::model::Extension;
use crate::model::PropertyType;

#[allow(dead_code)]
pub struct EntityTypeBuilder {
    type_name: String,
    namespace: String,
    description: String,
    components: Vec<String>,
    properties: Vec<PropertyType>,
    extensions: Vec<Extension>,
}

#[allow(dead_code)]
impl EntityTypeBuilder {
    pub fn new<S: Into<String>>(type_name: S) -> EntityTypeBuilder {
        EntityTypeBuilder {
            type_name: type_name.into(),
            namespace: String::new(),
            description: String::new(),
            components: Vec::new(),
            properties: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn namespace<S: Into<String>>(&mut self, namespace: S) -> &mut EntityTypeBuilder {
        self.namespace = namespace.into();
        self
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut EntityTypeBuilder {
        self.description = description.into();
        self
    }

    pub fn component<S: Into<String>>(&mut self, component_name: S) -> &mut EntityTypeBuilder {
        self.components.push(component_name.into());
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut EntityTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), data_type));
        self
    }

    pub fn input_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut EntityTypeBuilder {
        self.properties.push(PropertyType::input(property_name.into(), data_type));
        self
    }

    pub fn output_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut EntityTypeBuilder {
        self.properties.push(PropertyType::output(property_name.into(), data_type));
        self
    }

    pub fn property_from<S: Into<PropertyType>>(&mut self, property_type: S) -> &mut EntityTypeBuilder {
        self.properties.push(property_type.into());
        self
    }

    pub fn string_property<S: Into<String>>(&mut self, property_name: S) -> &mut EntityTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::String));
        self
    }

    pub fn bool_property<S: Into<String>>(&mut self, property_name: S) -> &mut EntityTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Bool));
        self
    }

    pub fn number_property<S: Into<String>>(&mut self, property_name: S) -> &mut EntityTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Number));
        self
    }

    pub fn extension<S: Into<String>>(&mut self, name: S, extension: Value) -> &mut EntityTypeBuilder {
        self.extensions.push(Extension { name: name.into(), extension });
        self
    }

    pub fn build(&mut self) -> EntityType {
        EntityType::new(
            self.type_name.clone(),
            self.namespace.clone(),
            self.description.clone(),
            self.components.to_vec(),
            self.properties.to_vec(),
            self.extensions.to_vec(),
        )
    }
}
