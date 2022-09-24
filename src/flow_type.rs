use serde::Deserialize;
use serde::Serialize;

use crate::EntityInstance;
use crate::Extension;
use crate::PropertyType;
use crate::RelationInstance;

#[derive(Debug)]
pub struct FlowTypeCreationError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowType {
    /// The entity type of the flow.
    #[serde(alias = "type")]
    pub type_name: String,

    /// The name of the flow type.
    #[serde(default = "String::new")]
    pub name: String,

    /// The namespace the entity type belongs to.
    #[serde(default = "String::new")]
    pub namespace: String,

    /// Textual description of the flow type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The entity instances which are contained in this flow.
    ///
    /// By default, no relation instances are contained in this flow type.
    #[serde(default = "Vec::new", alias = "entities")]
    pub entity_instances: Vec<EntityInstance>,

    /// The relation instances which are contained in this flow.
    ///
    /// By default, no relation instances are contained in this flow type.
    #[serde(default = "Vec::new", alias = "relations")]
    pub relation_instances: Vec<RelationInstance>,

    /// The variables. Variables will be replaced by instantiation of a flow instance.
    ///
    /// By default, the flow type has no variables.
    #[serde(default = "Vec::new")]
    pub variables: Vec<PropertyType>,

    /// Entity type specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl FlowType {
    pub fn new<S: Into<String>>(
        type_name: S,
        name: S,
        namespace: S,
        description: S,
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> FlowType {
        FlowType {
            type_name: type_name.into(),
            name: name.into(),
            namespace: namespace.into(),
            description: description.into(),
            entity_instances,
            relation_instances,
            variables,
            extensions,
        }
    }

    /// Returns the entity types which are used by the flow type
    pub fn uses_entity_types(&self) -> Vec<String> {
        let mut entity_type_names: Vec<String> = self.entity_instances.iter().map(|e| e.type_name.clone()).collect();
        entity_type_names.push(self.type_name.clone());
        entity_type_names.dedup();
        entity_type_names
    }

    /// Returns the entity types which are used by the flow type
    pub fn uses_relation_types(&self) -> Vec<String> {
        let mut relation_type_names: Vec<String> = self.relation_instances.iter().map(|r| r.type_name.clone()).collect();
        relation_type_names.dedup();
        relation_type_names
    }

    /// Returns true, if the flow type contains an variable with the given name.
    pub fn has_variable<S: Into<String>>(&self, variable_name: S) -> bool {
        let variable_name = variable_name.into();
        self.variables.iter().any(|p| p.name == variable_name)
    }

    /// Returns true, if the flow type contains an extension with the given name.
    pub fn has_extension<S: Into<String>>(&self, extension_name: S) -> bool {
        let extension_name = extension_name.into();
        self.extensions.iter().any(|extension| extension.name == extension_name)
    }
}
