use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::EntityInstance;
use crate::Extension;
use crate::PropertyType;
use crate::RelationInstance;

#[derive(Debug)]
pub struct FlowTypeCreationError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowType {
    /// The namespace the entity type belongs to.
    #[serde(default = "String::new")]
    pub namespace: String,

    /// The name of the flow type.
    #[serde(default = "String::new")]
    pub name: String,

    /// Textual description of the flow type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The wrapper entity instance.
    pub wrapper_entity_instance: EntityInstance,

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
    #[allow(clippy::too_many_arguments)]
    pub fn new<S: Into<String>>(
        namespace: S,
        name: S,
        description: S,
        wrapper_entity_instance: EntityInstance,
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> FlowType {
        FlowType {
            namespace: namespace.into(),
            name: name.into(),
            description: description.into(),
            wrapper_entity_instance,
            entity_instances,
            relation_instances,
            variables,
            extensions,
        }
    }

    pub fn id(&self) -> Uuid {
        self.wrapper_entity_instance.id
    }

    /// Returns the entity type namespace of the flow type
    pub fn type_namespace(&self) -> String {
        self.wrapper_entity_instance.namespace.clone()
    }

    /// Returns the entity type name of the flow type
    pub fn type_name(&self) -> String {
        self.wrapper_entity_instance.type_name.clone()
    }

    /// Returns the entity types which are used by the flow type
    pub fn uses_entity_types(&self) -> Vec<String> {
        let mut entity_type_names: Vec<String> = self.entity_instances.iter().map(|e| e.type_name.clone()).collect();
        entity_type_names.push(self.type_name());
        entity_type_names.dedup();
        entity_type_names
    }

    /// Returns the entity instances (including the wrapper entity instance) of the flow type
    pub fn entity_instances(&self) -> Vec<EntityInstance> {
        let mut entity_instances = self.entity_instances.to_vec();
        entity_instances.push(self.wrapper_entity_instance.clone());
        entity_instances
    }

    /// Returns the entity types which are used by the flow type
    pub fn uses_relation_types(&self) -> Vec<String> {
        let mut relation_type_names: Vec<String> = self.relation_instances.iter().map(|r| r.type_name.clone()).collect();
        relation_type_names.dedup();
        relation_type_names
    }

    /// Returns the relation instances of the flow type
    pub fn relation_instances(&self) -> Vec<RelationInstance> {
        self.relation_instances.to_vec()
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
