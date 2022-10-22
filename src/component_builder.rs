use serde_json::Value;

use crate::model::Component;
use crate::model::ComponentType;
use crate::model::DataType;
use crate::model::Extension;
use crate::model::PropertyType;

#[allow(dead_code)]
pub struct ComponentBuilder {
    ty: ComponentType,
    description: String,
    properties: Vec<PropertyType>,
    extensions: Vec<Extension>,
}

#[allow(dead_code)]
impl ComponentBuilder {
    pub fn new(ty: ComponentType) -> ComponentBuilder {
        ComponentBuilder {
            ty,
            description: String::new(),
            properties: Vec::new(),
            extensions: Vec::new(),
        }
    }

    /// Creates a builder for creating a new component with the given name and namespace.
    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> ComponentBuilder {
        ComponentBuilder::new(ComponentType::new_from_type(namespace, type_name))
    }

    /// Sets the description of the component.
    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut ComponentBuilder {
        self.description = description.into();
        self
    }

    /// Adds a property to the component with the given name and data type.
    pub fn property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), data_type));
        self
    }

    /// Adds a property to the component with the given name and data type. The socket type is INPUT.
    pub fn input_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::input(property_name.into(), data_type));
        self
    }

    /// Adds a property to the component with the given name and data type. The socket type is OUTPUT.
    pub fn output_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::output(property_name.into(), data_type));
        self
    }

    /// Adds a property to the component. Uses the given property type.
    pub fn property_from<S: Into<PropertyType>>(&mut self, property_type: S) -> &mut ComponentBuilder {
        self.properties.push(property_type.into());
        self
    }

    /// Adds a string property to the component with the given name.
    pub fn string_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::String));
        self
    }

    /// Adds a boolean property to the component with the given name.
    pub fn bool_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Bool));
        self
    }

    /// Adds a number property to the component with the given name.
    pub fn number_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Number));
        self
    }

    /// Adds an array property to the component with the given name.
    pub fn array_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Array));
        self
    }

    /// Adds an object property to the component with the given name.
    pub fn object_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Object));
        self
    }

    /// Adds an extension to the component with the given name.
    pub fn extension<S: Into<String>>(&mut self, name: S, extension: Value) -> &mut ComponentBuilder {
        self.extensions.push(Extension { name: name.into(), extension });
        self
    }

    /// Constructs the component with the previously defined builder data.
    pub fn build(&self) -> Component {
        Component {
            ty: self.ty.clone(),
            description: self.description.clone(),
            properties: self.properties.to_vec(),
            extensions: self.extensions.to_vec(),
        }
    }
}

#[allow(dead_code)]
pub struct ComponentsBuilder {
    namespace: String,
    components: Vec<Component>,
    builder: Option<ComponentBuilder>,
}

#[allow(dead_code)]
impl ComponentsBuilder {
    pub fn new<S: Into<String>>(namespace: S) -> ComponentsBuilder {
        ComponentsBuilder {
            namespace: namespace.into(),
            components: Vec::new(),
            builder: None,
        }
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.description(description.into());
        }
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.property(property_name, data_type);
        }
        self
    }

    pub fn input_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.input_property(property_name, data_type);
        }
        self
    }

    pub fn output_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.output_property(property_name, data_type);
        }
        self
    }

    pub fn property_from<S: Into<PropertyType>>(&mut self, property_type: S) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.property_from(property_type);
        }
        self
    }

    pub fn string_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.string_property(property_name);
        }
        self
    }

    pub fn bool_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.bool_property(property_name);
        }
        self
    }

    pub fn number_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.number_property(property_name);
        }
        self
    }

    pub fn array_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.array_property(property_name);
        }
        self
    }

    pub fn object_property<S: Into<String>>(&mut self, property_name: S) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.object_property(property_name);
        }
        self
    }

    pub fn extension<S: Into<String>>(&mut self, name: S, extension: Value) -> &mut ComponentsBuilder {
        if let Some(builder) = &mut self.builder {
            builder.extension(name, extension);
        }
        self
    }

    pub fn done(&mut self) -> &mut ComponentsBuilder {
        if let Some(builder) = &self.builder {
            let component = builder.build();
            self.components.push(component);
        }
        self.builder = None;
        self
    }

    pub fn next<S: Into<String>>(&mut self, name: S) -> &mut ComponentsBuilder {
        if self.builder.is_some() {
            self.done();
        }
        self.builder = Some(ComponentBuilder::new_from_type(self.namespace.clone(), name.into()));
        self
    }

    pub fn build(&mut self) -> Vec<Component> {
        if self.builder.is_some() {
            self.done();
        }
        self.components.clone()
    }
}
