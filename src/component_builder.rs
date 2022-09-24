use serde_json::Value;

use crate::model::Component;
use crate::model::DataType;
use crate::model::Extension;
use crate::model::PropertyType;

#[allow(dead_code)]
pub struct ComponentBuilder {
    name: String,
    description: String,
    properties: Vec<PropertyType>,
    extensions: Vec<Extension>,
}

#[allow(dead_code)]
impl ComponentBuilder {
    pub fn new<S: Into<String>>(type_name: S) -> ComponentBuilder {
        ComponentBuilder {
            name: type_name.into(),
            description: String::new(),
            properties: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut ComponentBuilder {
        self.description = description.into();
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), data_type));
        self
    }

    pub fn input_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::input(property_name.into(), data_type));
        self
    }

    pub fn output_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::output(property_name.into(), data_type));
        self
    }

    pub fn property_from<S: Into<PropertyType>>(&mut self, property_type: S) -> &mut ComponentBuilder {
        self.properties.push(property_type.into());
        self
    }

    pub fn string_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::String));
        self
    }

    pub fn bool_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Bool));
        self
    }

    pub fn number_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Number));
        self
    }

    pub fn array_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Array));
        self
    }

    pub fn object_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Object));
        self
    }

    pub fn extension<S: Into<String>>(&mut self, name: S, extension: Value) -> &mut ComponentBuilder {
        self.extensions.push(Extension { name: name.into(), extension });
        self
    }

    pub fn build(&mut self) -> Component {
        Component {
            name: self.name.clone(),
            description: self.description.clone(),
            properties: self.properties.to_vec(),
            extensions: self.extensions.to_vec(),
        }
    }
}
