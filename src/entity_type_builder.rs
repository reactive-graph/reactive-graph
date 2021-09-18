use serde_json::Value;

use crate::model::{DataType, EntityType, Extension, PropertyType};

#[allow(dead_code)]
pub struct EntityTypeBuilder {
    type_name: String,
    group: String,
    components: Vec<String>,
    behaviours: Vec<String>,
    properties: Vec<PropertyType>,
    extensions: Vec<Extension>,
}

#[allow(dead_code)]
impl EntityTypeBuilder {
    pub fn new<S: Into<String>>(type_name: S) -> EntityTypeBuilder {
        EntityTypeBuilder {
            type_name: type_name.into(),
            group: String::new(),
            components: Vec::new(),
            behaviours: Vec::new(),
            properties: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn group<'a, S: Into<String>>(&'a mut self, group: S) -> &'a mut EntityTypeBuilder {
        self.group = group.into();
        self
    }

    pub fn component<'a, S: Into<String>>(
        &'a mut self,
        component_name: S,
    ) -> &'a mut EntityTypeBuilder {
        self.components.push(component_name.into());
        self
    }

    pub fn behaviour<'a, S: Into<String>>(
        &'a mut self,
        behaviour_name: S,
    ) -> &'a mut EntityTypeBuilder {
        self.behaviours.push(behaviour_name.into());
        self
    }

    pub fn property<'a, S: Into<String>>(
        &'a mut self,
        property_name: S,
        data_type: DataType,
    ) -> &'a mut EntityTypeBuilder {
        self.properties
            .push(PropertyType::new(property_name.into(), data_type));
        self
    }

    pub fn property_from<'a, S: Into<PropertyType>>(
        &'a mut self,
        property_type: S,
    ) -> &'a mut EntityTypeBuilder {
        self.properties.push(property_type.into());
        self
    }

    pub fn string_property<'a, S: Into<String>>(
        &'a mut self,
        property_name: S,
    ) -> &'a mut EntityTypeBuilder {
        self.properties
            .push(PropertyType::new(property_name.into(), DataType::String));
        self
    }

    pub fn bool_property<'a, S: Into<String>>(
        &'a mut self,
        property_name: S,
    ) -> &'a mut EntityTypeBuilder {
        self.properties
            .push(PropertyType::new(property_name.into(), DataType::Bool));
        self
    }

    pub fn number_property<'a, S: Into<String>>(
        &'a mut self,
        property_name: S,
    ) -> &'a mut EntityTypeBuilder {
        self.properties
            .push(PropertyType::new(property_name.into(), DataType::Number));
        self
    }

    pub fn extension<'a, S: Into<String>>(
        &'a mut self,
        name: S,
        extension: Value,
    ) -> &'a mut EntityTypeBuilder {
        self.extensions.push(Extension {
            name: name.into(),
            extension,
        });
        self
    }

    pub fn build<'a>(&'a mut self) -> EntityType {
        EntityType::new(
            self.type_name.clone(),
            self.group.clone(),
            self.components.to_vec(),
            self.behaviours.to_vec(),
            self.properties.to_vec(),
            self.extensions.to_vec(),
        )
    }
}
