use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::EntityInstance;
use crate::EntityTypeId;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::FlowTypeId;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::RelationInstance;
use crate::RelationTypeId;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

#[derive(Debug)]
pub struct FlowTypeCreationError;

/// Flow types defines the type of an flow instance like a template
/// for flow instances.
///
/// They contain entity instances and relation instances. The wrapper
/// entity instance is mandatory and used for input and outputs.
///
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct FlowType {
    /// The type definition of the entity type.
    #[serde(flatten)]
    pub ty: FlowTypeId,

    /// Textual description of the flow type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The wrapper entity instance.
    pub wrapper_entity_instance: EntityInstance,

    /// The entity instances which are contained in this flow.
    ///
    /// By default, no relation instances are contained in this flow type.
    #[serde(default = "Vec::<EntityInstance>::new", alias = "entities")]
    pub entity_instances: Vec<EntityInstance>,

    /// The relation instances which are contained in this flow.
    ///
    /// By default, no relation instances are contained in this flow type.
    #[serde(default = "Vec::<RelationInstance>::new", alias = "relations")]
    pub relation_instances: Vec<RelationInstance>,

    /// The variables. Variables will be replaced by instantiation of a flow instance.
    ///
    /// By default, the flow type has no variables.
    #[serde(default = "Vec::<PropertyType>::new")]
    pub variables: Vec<PropertyType>,

    /// Flow type specific extensions.
    #[serde(default = "Vec::<Extension>::new")]
    pub extensions: Vec<Extension>,
}

impl FlowType {
    pub fn new<T: Into<FlowTypeId>, S: Into<String>>(
        ty: T,
        description: S,
        wrapper_entity_instance: EntityInstance,
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> FlowType {
        FlowType {
            ty: ty.into(),
            description: description.into(),
            wrapper_entity_instance,
            entity_instances,
            relation_instances,
            variables,
            extensions,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_from_type<S: Into<String>>(
        namespace: S,
        type_name: S,
        description: S,
        wrapper_entity_instance: EntityInstance,
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> FlowType {
        FlowType {
            ty: FlowTypeId::new_from_type(namespace, type_name),
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
    pub fn wrapper_type(&self) -> EntityTypeId {
        self.wrapper_entity_instance.ty.clone()
    }

    /// Returns the entity types which are used by the flow type
    pub fn uses_entity_types(&self) -> Vec<EntityTypeId> {
        let mut entity_types: Vec<EntityTypeId> = self.entity_instances.iter().map(|e| e.ty.clone()).collect();
        entity_types.push(self.wrapper_type());
        entity_types.dedup();
        entity_types
    }

    /// Returns the entity instances (including the wrapper entity instance) of the flow type
    pub fn entity_instances(&self) -> Vec<EntityInstance> {
        let mut entity_instances = self.entity_instances.to_vec();
        entity_instances.push(self.wrapper_entity_instance.clone());
        entity_instances
    }

    pub fn has_entity_instance(&self, id: Uuid) -> bool {
        self.entity_instances.iter().any(|e| e.id == id)
    }

    pub fn has_relation_which_uses_entity_instance(&self, id: Uuid) -> bool {
        self.relation_instances.iter().any(|r| r.outbound_id == id || r.inbound_id == id)
    }

    /// Adds the given entity instance.
    pub fn add_entity_instance(&mut self, entity_instance: EntityInstance) {
        self.entity_instances.push(entity_instance);
    }

    /// Removes the entity instance with the given id from the flow type
    pub fn remove_entity_instance(&mut self, id: Uuid) {
        if !self.has_relation_which_uses_entity_instance(id) {
            self.entity_instances.retain(|e| e.id != id);
        }
    }

    /// Returns the entity types which are used by the flow type
    pub fn uses_relation_types(&self) -> Vec<RelationTypeId> {
        let mut relation_type_names: Vec<RelationTypeId> = self.relation_instances.iter().map(|r| r.relation_type_id()).collect();
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

    /// Adds the given variable.
    pub fn add_variable(&mut self, property: PropertyType) {
        self.variables.push(property)
    }

    /// Removes the variable with the given name from the flow type.
    pub fn remove_variable(&mut self, variable_name: &str) {
        self.variables.retain(|v| v.name != variable_name)
    }

    /// Returns true, if the flow type contains an extension with the given type.
    pub fn has_extension(&self, extension_ty: &ExtensionTypeId) -> bool {
        self.extensions.iter().any(|extension| &extension.ty == extension_ty)
    }

    /// Adds an extension to the flow type.
    pub fn add_extension(&mut self, extension: Extension) {
        self.extensions.push(extension)
    }

    /// Removes the extension with the given type from the flow type.
    pub fn remove_extension(&mut self, extension_ty: &ExtensionTypeId) {
        self.extensions.retain(|extension| &extension.ty != extension_ty)
    }
}

impl ExtensionContainer for FlowType {
    fn has_own_extension(&self, extension_ty: &ExtensionTypeId) -> bool {
        self.extensions.iter().any(|extension| &extension.ty == extension_ty)
    }

    fn get_own_extension(&self, extension_ty: &ExtensionTypeId) -> Option<Extension> {
        self.extensions.iter().find(|extension| &extension.ty == extension_ty).cloned()
    }

    fn merge_extensions(&mut self, extensions_to_merge: Vec<Extension>) {
        for extension_to_merge in extensions_to_merge {
            if !self.has_own_extension(&extension_to_merge.ty) {
                self.extensions.push(extension_to_merge);
            } else if let Some(existing_extension) = self.extensions.iter_mut().find(|e| e.ty == extension_to_merge.ty) {
                existing_extension.description = extension_to_merge.description.clone();
                existing_extension.extension = extension_to_merge.extension.clone();
            }
        }
    }
}

impl NamespacedTypeGetter for FlowType {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for FlowType {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}

impl From<&FlowType> for TypeDefinition {
    fn from(flow_type: &FlowType) -> Self {
        TypeDefinition {
            type_id_type: TypeIdType::FlowType,
            namespace: flow_type.namespace(),
            type_name: flow_type.type_name(),
        }
    }
}
