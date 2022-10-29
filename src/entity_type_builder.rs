use serde_json::Value;

use crate::model::ComponentTypeId;
use crate::model::DataType;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model::Extension;
use crate::model::PropertyType;

#[allow(dead_code)]
pub struct EntityTypeBuilder {
    ty: EntityTypeId,
    description: String,
    components: Vec<ComponentTypeId>,
    properties: Vec<PropertyType>,
    extensions: Vec<Extension>,
}

#[allow(dead_code)]
impl EntityTypeBuilder {
    pub fn new(ty: EntityTypeId) -> EntityTypeBuilder {
        EntityTypeBuilder {
            ty,
            description: String::new(),
            components: Vec::new(),
            properties: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> EntityTypeBuilder {
        EntityTypeBuilder::new(EntityTypeId::new_from_type(namespace, type_name))
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut EntityTypeBuilder {
        self.description = description.into();
        self
    }

    pub fn component(&mut self, ty: ComponentTypeId) -> &mut EntityTypeBuilder {
        self.components.push(ty);
        self
    }

    pub fn component_from_type<S: Into<String>>(&mut self, namespace: S, component_name: S) -> &mut EntityTypeBuilder {
        self.components.push(ComponentTypeId::new_from_type(namespace, component_name));
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

    pub fn array_property<S: Into<String>>(&mut self, property_name: S) -> &mut EntityTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Array));
        self
    }

    pub fn object_property<S: Into<String>>(&mut self, property_name: S) -> &mut EntityTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Object));
        self
    }

    pub fn extension<S: Into<String>>(&mut self, name: S, extension: Value) -> &mut EntityTypeBuilder {
        self.extensions.push(Extension { name: name.into(), extension });
        self
    }

    pub fn build(&self) -> EntityType {
        EntityType::new(
            self.ty.clone(),
            self.description.clone(),
            self.components.to_vec(),
            self.properties.to_vec(),
            self.extensions.to_vec(),
        )
    }
}
