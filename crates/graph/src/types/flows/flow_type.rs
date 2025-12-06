use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use const_format::formatcp;
use dashmap::DashMap;
use dashmap::iter::OwningIter;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use schemars::consts::meta_schemas::DRAFT2020_12;
use schemars::json_schema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::borrow::Cow;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::AddEntityInstanceError;
use crate::AddExtensionError;
use crate::AddRelationInstanceError;
use crate::AddVariableError;
use crate::EntityInstance;
use crate::EntityInstanceContainer;
use crate::EntityInstances;
use crate::EntityType;
use crate::EntityTypeId;
use crate::EntityTypeIds;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::Extensions;
use crate::FlowTypeAddEntityInstanceError;
use crate::FlowTypeAddExtensionError;
use crate::FlowTypeAddRelationInstanceError;
use crate::FlowTypeAddVariableError;
use crate::FlowTypeDoesNotExistError;
use crate::FlowTypeId;
use crate::FlowTypeIds;
use crate::FlowTypeJsonSchemaError;
use crate::FlowTypeMergeExtensionsError;
use crate::FlowTypeMergeVariablesError;
use crate::FlowTypeRemoveEntityInstanceError;
use crate::FlowTypeRemoveExtensionError;
use crate::FlowTypeRemoveRelationInstanceError;
use crate::FlowTypeRemoveVariableError;
use crate::FlowTypeUpdateEntityInstanceError;
use crate::FlowTypeUpdateExtensionError;
use crate::FlowTypeUpdateRelationInstanceError;
use crate::FlowTypeUpdateVariableError;
use crate::JSON_SCHEMA_ID_URI_PREFIX;
use crate::JsonSchemaIdGetter;
use crate::Namespace;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeContainer;
use crate::NamespacedTypeEntityInstanceContainer;
use crate::NamespacedTypeExtensionContainer;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypeRelationInstanceContainer;
use crate::NamespacedTypeVariablesContainer;
use crate::PropertyTypeContainer;
use crate::PropertyTypes;
use crate::RelationInstance;
use crate::RelationInstanceContainer;
use crate::RelationInstanceId;
use crate::RelationInstances;
use crate::RelationTypeIds;
use crate::RemoveEntityInstanceError;
use crate::RemoveExtensionError;
use crate::RemoveRelationInstanceError;
use crate::RemoveVariableError;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeDescriptionGetter;
use crate::TypeIdType;
use crate::UpdateEntityInstanceError;
use crate::UpdateExtensionError;
use crate::UpdateRelationInstanceError;
use crate::UpdateVariableError;
use crate::Variable;
use crate::Variables;
use crate::VariablesContainer;
use crate::divergent::DivergentPropertyTypes;

#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildType;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomInstance;
#[cfg(any(test, feature = "test"))]
use crate::RandomInstances;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedType;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypes;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;

pub const JSON_SCHEMA_ID_FLOW_TYPE: &str = formatcp!("{}/flow-type.schema.json", JSON_SCHEMA_ID_URI_PREFIX);

#[derive(Debug)]
pub struct FlowTypeCreationError;

/// Flow types defines the type of flow instance like a template
/// for flow instances.
///
/// They contain entity instances and relation instances. The wrapper
/// entity instance is mandatory and used for input and outputs.
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[serde(tag = "$id", rename = "https://schema.reactive-graph.io/schema/json/flow-type.schema.json")]
#[schemars(
    title = "FlowType",
    deny_unknown_fields,
    extend("$id" = JSON_SCHEMA_ID_FLOW_TYPE),
    transform = add_json_schema_id_property
)]
pub struct FlowType {
    /// The type definition of the entity type.
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub ty: FlowTypeId,

    /// Textual description of the flow type.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub description: String,

    /// The wrapper entity instance.
    #[builder(setter(into))]
    pub wrapper_entity_instance: EntityInstance,

    /// The entity instances which are contained in this flow.
    ///
    /// By default, no entity instances are contained in this flow type.
    #[serde(default = "EntityInstances::new", alias = "entities")]
    #[builder(default, setter(into))]
    pub entity_instances: EntityInstances,

    /// The relation instances which are contained in this flow.
    ///
    /// By default, no relation instances are contained in this flow type.
    #[serde(default = "RelationInstances::new", alias = "relations")]
    #[builder(default, setter(into))]
    pub relation_instances: RelationInstances,

    /// The variables. Variables will be replaced by instantiation of a flow instance.
    ///
    /// By default, the flow type has no variables.
    #[serde(default = "Variables::new")]
    #[builder(default, setter(into))]
    pub variables: Variables,

    /// Flow type specific extensions.
    #[serde(default = "Extensions::new")]
    #[builder(default, setter(into))]
    pub extensions: Extensions,
}

impl FlowType {
    pub fn new<T: Into<FlowTypeId>, D: Into<String>, EI: Into<EntityInstances>, RI: Into<RelationInstances>, V: Into<PropertyTypes>, E: Into<Extensions>>(
        ty: T,
        description: D,
        wrapper_entity_instance: EntityInstance,
        entity_instances: EI,
        relation_instances: RI,
        variables: V,
        extensions: E,
    ) -> FlowType {
        FlowType {
            ty: ty.into(),
            description: description.into(),
            wrapper_entity_instance,
            entity_instances: entity_instances.into(),
            relation_instances: relation_instances.into(),
            variables: variables.into(),
            extensions: extensions.into(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.wrapper_entity_instance.id
    }

    /// Returns the entity type namespace of the flow type.
    pub fn wrapper_type(&self) -> EntityTypeId {
        self.wrapper_entity_instance.ty.clone()
    }

    /// Returns the entity types which are used by the flow type.
    pub fn uses_entity_types(&self) -> EntityTypeIds {
        let entity_tys = self.entity_instances.get_type_ids();
        entity_tys.insert(self.wrapper_type());
        entity_tys
    }

    /// Returns the relation types which are used by the flow type.
    pub fn uses_relation_types(&self) -> RelationTypeIds {
        self.relation_instances.get_type_ids()
    }

    pub fn json_schema(&self, entity_type: &EntityType) -> Result<Schema, FlowTypeJsonSchemaError> {
        let entity_ty = self.wrapper_type();
        if entity_type.ty != entity_ty {
            return Err(FlowTypeJsonSchemaError::WrapperEntityTypeDoesNotMatch(self.ty.clone(), entity_ty, entity_type.ty.clone()));
        }
        let mut properties = entity_type.properties.as_json_schema_properties();
        properties.insert("$id".to_string(), self.json_schema_id_property());
        properties.insert(
            "id".to_string(),
            json!({
                "description": "The unique identifier of the instance",
                "type": "string",
                "format": "uuid"
            }),
        );
        let mut required = entity_type.properties.names();
        required.push("id".to_string());
        required.sort();
        let json_schema = json_schema!({
            "$schema": DRAFT2020_12,
            "$id": self.json_schema_id(),
            "type": "object",
            "title": self.type_name(),
            "description": self.description,
            "properties": properties,
            "required": required,
        });
        Ok(json_schema)
    }
}

impl EntityInstanceContainer for FlowType {
    /// Returns the entity instances (including the wrapper entity instance) of the flow type.
    fn entity_instances(&self) -> EntityInstances {
        let entity_instances = self.entity_instances.clone();
        entity_instances.push(self.wrapper_entity_instance.clone());
        entity_instances
    }

    /// Returns true, if the flow type has an entity instance with the given id.
    fn has_entity_instance(&self, id: Uuid) -> bool {
        self.entity_instances.contains_key(&id) || self.wrapper_entity_instance.id == id
    }

    fn add_entity_instance(&self, entity_instance: EntityInstance) -> Result<(), AddEntityInstanceError> {
        if self.has_entity_instance(entity_instance.id) {
            return Err(AddEntityInstanceError::EntityInstanceAlreadyExist(entity_instance.id));
        }
        self.entity_instances.push(entity_instance);
        Ok(())
    }

    fn update_entity_instance(&self, id: Uuid, entity_instance: EntityInstance) -> Result<(Uuid, EntityInstance), UpdateEntityInstanceError> {
        let Some(old_entity_instance) = self.entity_instances.remove(&id) else {
            return Err(UpdateEntityInstanceError::EntityInstanceDoesNotExist(entity_instance.id));
        };
        self.entity_instances.push(entity_instance);
        Ok(old_entity_instance)
    }

    fn remove_entity_instance(&self, id: Uuid) -> Result<Option<(Uuid, EntityInstance)>, RemoveEntityInstanceError> {
        if self.has_relation_which_uses_entity_instance(id) {
            // TODO: provide the RelationInstanceId in error type
            return Err(RemoveEntityInstanceError::EntityInstanceInUse(id));
        }
        Ok(self.entity_instances.remove(&id))
    }
}

impl RelationInstanceContainer for FlowType {
    /// Returns the relation instances of the flow type
    fn relation_instances(&self) -> RelationInstances {
        self.relation_instances.clone()
    }

    fn has_relation_which_uses_entity_instance(&self, id: Uuid) -> bool {
        self.relation_instances.iter().any(|r| r.outbound_id == id || r.inbound_id == id)
    }

    fn has_relation_instance(&self, id: &RelationInstanceId) -> bool {
        self.relation_instances.contains_key(id)
    }

    fn add_relation_instance(&self, relation_instance: RelationInstance) -> Result<(), AddRelationInstanceError> {
        let id = relation_instance.id();
        if self.relation_instances.contains_key(&id) {
            return Err(AddRelationInstanceError::RelationInstanceAlreadyExist(id));
        }
        // Check if outbound and inbound are available
        if !self.entity_instances.contains_key(&relation_instance.outbound_id) {
            return Err(AddRelationInstanceError::OutboundEntityInstanceDoesNotExist(relation_instance.outbound_id));
        }
        if !self.entity_instances.contains_key(&relation_instance.inbound_id) {
            return Err(AddRelationInstanceError::InboundEntityInstanceDoesNotExist(relation_instance.inbound_id));
        }
        self.relation_instances.push(relation_instance);
        Ok(())
    }

    fn update_relation_instance(
        &self,
        id: &RelationInstanceId,
        relation_instance: RelationInstance,
    ) -> Result<(RelationInstanceId, RelationInstance), UpdateRelationInstanceError> {
        let Some(old_relation_instance) = self.relation_instances.remove(id) else {
            return Err(UpdateRelationInstanceError::RelationInstanceDoesNotExist(id.clone()));
        };
        self.relation_instances.push(relation_instance);
        Ok(old_relation_instance)
    }

    fn remove_relation_instance(&self, id: &RelationInstanceId) -> Result<Option<(RelationInstanceId, RelationInstance)>, RemoveRelationInstanceError> {
        if !self.relation_instances.contains_key(id) {
            return Err(RemoveRelationInstanceError::RelationInstanceDoesNotExist(id.clone()));
        }
        Ok(self.relation_instances.remove(id))
    }
}

impl VariablesContainer for FlowType {
    fn has_variable<S: Into<String>>(&self, variable_name: S) -> bool {
        self.variables.has_own_property(variable_name)
    }

    fn get_variable<N: Into<String>>(&self, variable_name: N) -> Option<Variable> {
        self.variables.get_own_property(variable_name)
    }

    fn add_variable<V: Into<Variable>>(&self, variable: V) -> Result<Variable, AddVariableError> {
        self.variables.add_property(variable).map_err(|e| e.into())
    }

    fn update_variable<N: Into<String>, V: Into<Variable>>(&self, variable_name: N, variable: V) -> Result<Variable, UpdateVariableError> {
        self.variables.update_property(variable_name, variable).map_err(|e| e.into())
    }

    fn remove_variable<S: Into<String>>(&self, variable_name: S) -> Result<Variable, RemoveVariableError> {
        self.variables.remove_property(variable_name).map_err(|e| e.into())
    }

    fn merge_variables<V: Into<Variables>>(&mut self, variables_to_merge: V) {
        self.variables.merge_properties(variables_to_merge)
    }

    fn merge_non_existent_variables<V: Into<Variables>>(&self, variables_to_merge: V) -> DivergentPropertyTypes {
        self.variables.merge_non_existent_properties(variables_to_merge)
    }

    fn get_own_variables_cloned(&self) -> Variables {
        self.variables.clone()
    }
}

impl ExtensionContainer for FlowType {
    fn has_own_extension(&self, ty: &ExtensionTypeId) -> bool {
        self.extensions.has_own_extension(ty)
    }

    fn get_own_extension(&self, ty: &ExtensionTypeId) -> Option<Extension> {
        self.extensions.get_own_extension(ty)
    }

    fn add_extension<E: Into<Extension>>(&self, extension: E) -> Result<ExtensionTypeId, AddExtensionError> {
        self.extensions.add_extension(extension)
    }

    fn update_extension<T: Into<ExtensionTypeId>, E: Into<Extension>>(&self, ty: T, extension: E) -> Result<Extension, UpdateExtensionError> {
        self.extensions.update_extension(ty, extension)
    }

    fn remove_extension<T: Into<ExtensionTypeId>>(&self, ty: T) -> Result<Extension, RemoveExtensionError> {
        self.extensions.remove_extension(ty)
    }

    fn merge_extensions<E: Into<Extensions>>(&mut self, extensions_to_merge: E) {
        self.extensions.merge_extensions(extensions_to_merge)
    }

    fn get_own_extensions_cloned(&self) -> Extensions {
        self.extensions.clone()
    }
}

impl NamespacedTypeGetter for FlowType {
    fn namespaced_type(&self) -> NamespacedType {
        self.ty.namespaced_type()
    }

    fn namespace(&self) -> Namespace {
        self.ty.namespace()
    }

    fn path(&self) -> Namespace {
        self.ty.path()
    }

    fn type_name(&self) -> NamespaceSegment {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for FlowType {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::FlowType
    }
}

impl TypeDescriptionGetter for FlowType {
    fn description(&self) -> String {
        self.description.clone()
    }
}

impl AsRef<FlowTypeId> for FlowType {
    fn as_ref(&self) -> &FlowTypeId {
        &self.ty
    }
}

impl PartialEq<FlowTypeId> for FlowType {
    fn eq(&self, ty: &FlowTypeId) -> bool {
        self.ty == *ty
    }
}

impl PartialOrd<Self> for FlowType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FlowType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ty.cmp(&other.ty)
    }
}

impl From<&FlowType> for TypeDefinition {
    fn from(flow_type: &FlowType) -> Self {
        flow_type.type_definition()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FlowTypes(DashMap<FlowTypeId, FlowType>);

impl FlowTypes {
    #[inline]
    pub fn new() -> Self {
        NamespacedTypeContainer::new()
    }

    #[inline]
    pub fn flow<F: Into<FlowType>>(self, flow_type: F) -> Self {
        NamespacedTypeContainer::push(&self, flow_type);
        self
    }

    #[inline]
    pub fn push<F: Into<FlowType>>(&self, flow_type: F) -> Option<FlowType> {
        NamespacedTypeContainer::push(self, flow_type)
    }
}

impl NamespacedTypeContainer for FlowTypes {
    type TypeId = FlowTypeId;
    type TypeIds = FlowTypeIds;
    type Type = FlowType;
}

impl
    NamespacedTypeEntityInstanceContainer<
        FlowTypeId,
        FlowTypeDoesNotExistError,
        FlowTypeAddEntityInstanceError,
        FlowTypeUpdateEntityInstanceError,
        FlowTypeRemoveEntityInstanceError,
    > for FlowTypes
{
    fn entity_instances(&self, flow_ty: &FlowTypeId) -> Result<EntityInstances, FlowTypeDoesNotExistError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeDoesNotExistError(flow_ty.clone()));
        };
        Ok(flow_type.entity_instances())
    }

    /// Returns true, if an entity instance with the given id exists.
    fn has_entity_instance(&self, flow_ty: &FlowTypeId, id: Uuid) -> Result<bool, FlowTypeDoesNotExistError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeDoesNotExistError(flow_ty.clone()));
        };
        Ok(flow_type.has_entity_instance(id))
    }

    /// Adds the given entity instance.
    fn add_entity_instance(&self, flow_ty: &FlowTypeId, entity_instance: EntityInstance) -> Result<(), FlowTypeAddEntityInstanceError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeAddEntityInstanceError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type
            .add_entity_instance(entity_instance)
            .map_err(FlowTypeAddEntityInstanceError::AddEntityInstanceError)
    }

    fn update_entity_instance(
        &self,
        flow_ty: &FlowTypeId,
        id: Uuid,
        entity_instance: EntityInstance,
    ) -> Result<(Uuid, EntityInstance), FlowTypeUpdateEntityInstanceError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeUpdateEntityInstanceError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type
            .update_entity_instance(id, entity_instance)
            .map_err(FlowTypeUpdateEntityInstanceError::UpdateEntityInstanceError)
    }

    fn remove_entity_instance(&self, flow_ty: &FlowTypeId, id: Uuid) -> Result<Option<(Uuid, EntityInstance)>, FlowTypeRemoveEntityInstanceError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeRemoveEntityInstanceError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type
            .remove_entity_instance(id)
            .map_err(FlowTypeRemoveEntityInstanceError::RemoveEntityInstanceError)
    }
}

impl
    NamespacedTypeRelationInstanceContainer<
        FlowTypeId,
        FlowTypeDoesNotExistError,
        FlowTypeAddRelationInstanceError,
        FlowTypeUpdateRelationInstanceError,
        FlowTypeRemoveRelationInstanceError,
    > for FlowTypes
{
    fn relation_instances(&self, flow_ty: &FlowTypeId) -> Result<RelationInstances, FlowTypeDoesNotExistError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeDoesNotExistError(flow_ty.clone()));
        };
        Ok(flow_type.relation_instances())
    }

    fn has_relation_which_uses_entity_instance(&self, flow_ty: &FlowTypeId, id: Uuid) -> Result<bool, FlowTypeDoesNotExistError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeDoesNotExistError(flow_ty.clone()));
        };
        Ok(flow_type.has_relation_which_uses_entity_instance(id))
    }

    fn has_relation_instance(&self, flow_ty: &FlowTypeId, id: &RelationInstanceId) -> Result<bool, FlowTypeDoesNotExistError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeDoesNotExistError(flow_ty.clone()));
        };
        Ok(flow_type.has_relation_instance(id))
    }

    fn add_relation_instance(&self, flow_ty: &FlowTypeId, relation_instance: RelationInstance) -> Result<(), FlowTypeAddRelationInstanceError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeAddRelationInstanceError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type
            .add_relation_instance(relation_instance)
            .map_err(FlowTypeAddRelationInstanceError::AddRelationInstanceError)
    }

    fn update_relation_instance(
        &self,
        flow_ty: &FlowTypeId,
        id: &RelationInstanceId,
        relation_instance: RelationInstance,
    ) -> Result<(RelationInstanceId, RelationInstance), FlowTypeUpdateRelationInstanceError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeUpdateRelationInstanceError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type
            .update_relation_instance(id, relation_instance)
            .map_err(FlowTypeUpdateRelationInstanceError::UpdateRelationInstanceError)
    }

    fn remove_relation_instance(
        &self,
        flow_ty: &FlowTypeId,
        id: &RelationInstanceId,
    ) -> Result<Option<(RelationInstanceId, RelationInstance)>, FlowTypeRemoveRelationInstanceError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeRemoveRelationInstanceError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type
            .remove_relation_instance(id)
            .map_err(FlowTypeRemoveRelationInstanceError::RemoveRelationInstanceError)
    }
}

impl
    NamespacedTypeVariablesContainer<
        FlowTypeId,
        FlowTypeAddVariableError,
        FlowTypeUpdateVariableError,
        FlowTypeRemoveVariableError,
        FlowTypeMergeVariablesError,
    > for FlowTypes
{
    fn add_variable<P: Into<Variable>>(&self, flow_ty: &FlowTypeId, variable: P) -> Result<Variable, FlowTypeAddVariableError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeAddVariableError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type.add_variable(variable).map_err(FlowTypeAddVariableError::AddVariableError)
    }

    fn update_variable<N: Into<String>, V: Into<Variable>>(
        &self,
        flow_ty: &FlowTypeId,
        variable_name: N,
        variable: V,
    ) -> Result<Variable, FlowTypeUpdateVariableError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeUpdateVariableError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type
            .update_variable(variable_name, variable)
            .map_err(FlowTypeUpdateVariableError::UpdateVariableError)
    }

    fn remove_variable<N: Into<String>>(&self, flow_ty: &FlowTypeId, variable_name: N) -> Result<Variable, FlowTypeRemoveVariableError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeRemoveVariableError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type
            .remove_variable(variable_name)
            .map_err(FlowTypeRemoveVariableError::RemoveVariableError)
    }

    fn merge_variables<V: Into<Variables>>(&mut self, flow_ty: &FlowTypeId, variables_to_merge: V) -> Result<(), FlowTypeMergeVariablesError> {
        let Some(mut flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeMergeVariablesError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type.merge_variables(variables_to_merge);
        Ok(())
    }
}

impl
    NamespacedTypeExtensionContainer<
        FlowTypeId,
        FlowTypeAddExtensionError,
        FlowTypeUpdateExtensionError,
        FlowTypeRemoveExtensionError,
        FlowTypeMergeExtensionsError,
    > for FlowTypes
{
    fn add_extension<E: Into<Extension>>(&self, flow_ty: &FlowTypeId, extension: E) -> Result<ExtensionTypeId, FlowTypeAddExtensionError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeAddExtensionError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type.add_extension(extension).map_err(FlowTypeAddExtensionError::AddExtensionError)
    }

    fn update_extension<T: Into<ExtensionTypeId>, E: Into<Extension>>(
        &self,
        flow_ty: &FlowTypeId,
        extension_ty: T,
        extension: E,
    ) -> Result<Extension, FlowTypeUpdateExtensionError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeUpdateExtensionError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type
            .update_extension(extension_ty, extension)
            .map_err(FlowTypeUpdateExtensionError::UpdateExtensionError)
    }

    fn remove_extension<T: Into<ExtensionTypeId>>(&self, flow_ty: &FlowTypeId, extension_ty: T) -> Result<Extension, FlowTypeRemoveExtensionError> {
        let Some(flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeRemoveExtensionError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type
            .remove_extension(extension_ty)
            .map_err(FlowTypeRemoveExtensionError::RemoveExtensionError)
    }

    fn merge_extensions<E: Into<Extensions>>(&mut self, flow_ty: &FlowTypeId, extensions_to_merge: E) -> Result<(), FlowTypeMergeExtensionsError> {
        let Some(mut flow_type) = self.0.get_mut(flow_ty) else {
            return Err(FlowTypeMergeExtensionsError::FlowTypeDoesNotExist(flow_ty.clone()));
        };
        flow_type.merge_extensions(extensions_to_merge);
        Ok(())
    }
}

impl Deref for FlowTypes {
    type Target = DashMap<FlowTypeId, FlowType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FlowTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for FlowTypes {
    type Item = (FlowTypeId, FlowType);
    type IntoIter = OwningIter<FlowTypeId, FlowType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for FlowTypes {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().all(|self_entity_type| other.contains_key(&self_entity_type.ty))
            && other.iter().all(|other_entity_type| self.contains_key(&other_entity_type.ty))
    }
}

impl Eq for FlowTypes {}

impl Hash for FlowTypes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for FlowTypes {
    fn schema_name() -> Cow<'static, str> {
        "FlowTypes".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<FlowType>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
            "description": "Flow Types",
        })
    }
}

impl From<Vec<FlowType>> for FlowTypes {
    fn from(flow_types: Vec<FlowType>) -> Self {
        Self(flow_types.into_iter().map(|entity_type| (entity_type.ty.clone(), entity_type)).collect())
    }
}

impl From<FlowTypes> for Vec<FlowType> {
    fn from(flow_types: FlowTypes) -> Self {
        flow_types.to_vec()
    }
}

impl From<&FlowTypes> for Vec<FlowType> {
    fn from(flow_types: &FlowTypes) -> Self {
        flow_types.0.iter().map(|flow_type| flow_type.clone()).collect()
    }
}

impl From<DashMap<FlowTypeId, FlowType>> for FlowTypes {
    fn from(flow_types: DashMap<FlowTypeId, FlowType>) -> Self {
        Self(flow_types)
    }
}

impl From<&DashMap<FlowTypeId, FlowType>> for FlowTypes {
    fn from(flow_types: &DashMap<FlowTypeId, FlowType>) -> Self {
        Self(flow_types.clone())
    }
}

impl From<FlowTypes> for DashMap<FlowTypeId, FlowType> {
    fn from(flow_types: FlowTypes) -> Self {
        flow_types.0
    }
}

impl FromIterator<FlowType> for FlowTypes {
    fn from_iter<I: IntoIterator<Item = FlowType>>(iter: I) -> Self {
        let flow_types = Self::new();
        for flow_type in iter {
            flow_types.insert(flow_type.ty.clone(), flow_type);
        }
        flow_types
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedType for FlowType {
    type Error = NamespacedTypeError;
    type TypeId = FlowTypeId;

    fn random_type() -> Result<Self, NamespacedTypeError> {
        Self::random_child_type(&Namespace::random_path()?)
    }

    fn random_type_with_id(ty: &Self::TypeId) -> Result<Self, Self::Error> {
        let wrapper_entity_instance = EntityInstance::random_instance()?;
        Ok(Self::builder()
            .ty(ty)
            .description(r_string())
            .wrapper_entity_instance(wrapper_entity_instance)
            .entity_instances(EntityInstances::random_instances()?)
            .relation_instances(RelationInstances::random_instances()?)
            .extensions(Extensions::random_types(0..10)?)
            .variables(PropertyTypes::random_types(0..10)?)
            .build())
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomChildType for FlowType {
    type Error = NamespacedTypeError;

    fn random_child_type(namespace: &Namespace) -> Result<Self, Self::Error> {
        Self::random_type_with_id(&NamespacedType::random_child_type_id(namespace)?.into())
    }
}

fn add_json_schema_id_property(schema: &mut Schema) {
    crate::json_schema::add_json_schema_id_property(schema, JSON_SCHEMA_ID_FLOW_TYPE);
}

#[cfg(test)]
mod tests {
    use crate::ComponentTypeIds;
    use crate::EntityInstance;
    use crate::EntityInstances;
    use crate::EntityType;
    use crate::EntityTypeId;
    use crate::ExtensionContainer;
    use crate::ExtensionTypeId;
    use crate::Extensions;
    use crate::FlowType;
    use crate::FlowTypeId;
    use crate::InstanceId;
    use crate::NamespacedTypeGetter;
    use crate::PropertyInstanceGetter;
    use crate::PropertyTypes;
    use crate::RandomInstance;
    use crate::RandomInstances;
    use crate::RandomNamespacedType;
    use crate::RandomNamespacedTypeId;
    use crate::RandomNamespacedTypeIds;
    use crate::RandomNamespacedTypes;
    use crate::RelationInstanceId;
    use crate::RelationInstanceTypeId;
    use crate::RelationInstances;
    use crate::RelationTypeId;
    use crate::VariablesContainer;
    use reactive_graph_utils_test::r_string;
    use schemars::schema_for;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn build_flow_type_test() {
        let ty = FlowTypeId::random_type_id().unwrap();
        let description = r_string();
        let wrapper_entity_instance = EntityInstance::random_instance().unwrap();
        let entity_instances = EntityInstances::random_instances().unwrap();
        let relation_instances = RelationInstances::random_instances().unwrap();
        let variables = PropertyTypes::random_types(0..5).unwrap();
        let extensions = Extensions::random_types(0..10).unwrap();
        let flow_type = FlowType::builder()
            .ty(&ty)
            .description(&description)
            .wrapper_entity_instance(wrapper_entity_instance.clone())
            .entity_instances(entity_instances.clone())
            .relation_instances(relation_instances.clone())
            .variables(variables.clone())
            .extensions(extensions.clone())
            .build();
        assert_eq!(ty.namespace(), flow_type.namespace());
        assert_eq!(ty.path(), flow_type.path());
        assert_eq!(ty.type_name(), flow_type.type_name());
        assert_eq!(description, flow_type.description);
        assert_eq!(wrapper_entity_instance.ty, flow_type.wrapper_type());
        assert_eq!(wrapper_entity_instance, flow_type.wrapper_entity_instance);
        assert_eq!(entity_instances.len(), flow_type.entity_instances.len());
        assert_eq!(relation_instances.len(), flow_type.relation_instances.len());
        assert_eq!(variables.len(), flow_type.variables.len());
        assert_eq!(extensions.len(), flow_type.extensions.len());
    }

    #[test]
    fn create_flow_type_test() {
        let ty = FlowTypeId::random_type_id().unwrap();
        let description = r_string();
        let wrapper_entity_instance = EntityInstance::random_instance().unwrap();
        let entity_instances = EntityInstances::random_instances().unwrap();
        let relation_instances = RelationInstances::random_instances().unwrap();
        let variables = PropertyTypes::random_types(1..5).unwrap();
        let extensions = Extensions::random_types(1..3).unwrap();
        let flow_type = FlowType::new(
            ty.clone(),
            description.clone(),
            wrapper_entity_instance.clone(),
            entity_instances.clone(),
            relation_instances.clone(),
            variables.clone(),
            extensions.clone(),
        );
        assert_eq!(ty.namespace(), flow_type.namespace());
        assert_eq!(ty.path(), flow_type.path());
        assert_eq!(ty.type_name(), flow_type.type_name());
        assert_eq!(description, flow_type.description);
        assert_eq!(wrapper_entity_instance.ty, flow_type.wrapper_type());
        assert_eq!(wrapper_entity_instance, flow_type.wrapper_entity_instance);
        assert_eq!(entity_instances.len(), flow_type.entity_instances.len());
        assert_eq!(relation_instances.len(), flow_type.relation_instances.len());
        assert_eq!(variables.len(), flow_type.variables.len());
        assert_eq!(extensions.len(), flow_type.extensions.len());
    }

    #[test]
    fn flow_type_deserialize_fully_valid_test() {
        let flow_ty = FlowTypeId::from_str("fully::qualified::namespace::FlowType").unwrap();
        let wrapper_ty = EntityTypeId::from_str("fully::qualified::namespace::WrapperEntityType").unwrap();
        let wrapper_id = Uuid::from_str("bc21d797-da21-4252-a0c7-2958e5c5c7d1").unwrap();
        let entity_ty = EntityTypeId::from_str("fully::qualified::namespace::EntityType").unwrap();
        let entity_id = Uuid::from_str("a61fa341-e1fd-45ba-be3e-089b7acc7910").unwrap();
        let relation_ty = RelationTypeId::from_str("fully::qualified::namespace::RelationType").unwrap();
        let instance_id = InstanceId::Id(Uuid::from_str("1a451c8e-f9f8-41e8-9828-991aee8af642").unwrap());
        let relation_instance_type_id = RelationInstanceTypeId::new(relation_ty, instance_id.clone());
        let relation_instance_id = RelationInstanceId::new(entity_id, relation_instance_type_id.clone(), wrapper_id);
        let extension_ty = ExtensionTypeId::from_str("fully::qualified::namespace::Extension").unwrap();
        let flow_type = serde_json::from_str::<FlowType>(
            r#"{
          "type": "fully::qualified::namespace::FlowType",
          "description": "d",
          "wrapper_entity_instance": {
            "$id": "https://schema.reactive-graph.io/schema/json/entity-instance.schema.json",
            "type": "fully::qualified::namespace::WrapperEntityType",
            "id": "bc21d797-da21-4252-a0c7-2958e5c5c7d1",
            "properties": {
              "property_name": "property_value"
            },
            "components": [],
            "extensions": []
          },
          "entity_instances": [
            {
            "$id": "https://schema.reactive-graph.io/schema/json/entity-instance.schema.json",
            "type": "fully::qualified::namespace::EntityType",
            "id": "a61fa341-e1fd-45ba-be3e-089b7acc7910",
            "properties": {
              "property_name": "property_value"
            },
            "components": [],
            "extensions": []
            }
          ],
          "relation_instances": [
            {
              "$id": "https://schema.reactive-graph.io/schema/json/relation-instance.schema.json",
              "outbound_id": "a61fa341-e1fd-45ba-be3e-089b7acc7910",
              "type": "fully::qualified::namespace::RelationType",
              "instance_id": "1a451c8e-f9f8-41e8-9828-991aee8af642",
              "inbound_id": "bc21d797-da21-4252-a0c7-2958e5c5c7d1",
              "properties": {
                "property_name": "property_value"
              },
              "components": [],
              "extensions": []
            }
          ],
          "variables": [
            {
              "name": "variable_name",
              "description": "jXDlcgeZko",
              "data_type": "number",
              "socket_type": "output",
              "mutability": "immutable"
            }
          ],
          "extensions": [
            {
              "type": "fully::qualified::namespace::Extension",
              "extension": ""
            }
          ]
        }"#,
        )
        .expect("Failed to deserialize entity type");
        assert_eq!(flow_ty, flow_type.ty);
        assert_eq!("d", flow_type.description);
        assert_eq!(wrapper_ty, flow_type.wrapper_entity_instance.ty);
        assert_eq!(wrapper_id, flow_type.wrapper_entity_instance.id);
        assert_eq!(1, flow_type.wrapper_entity_instance.properties.len());
        assert_eq!("property_value", flow_type.wrapper_entity_instance.as_string("property_name").unwrap());
        assert_eq!(1, flow_type.entity_instances.len());
        assert!(flow_type.entity_instances.get(&entity_id).is_some());
        assert_eq!(entity_ty, flow_type.entity_instances.get(&entity_id).unwrap().ty);
        assert_eq!(entity_id, flow_type.entity_instances.get(&entity_id).unwrap().id);
        assert_eq!(1, flow_type.relation_instances.len());
        assert!(flow_type.relation_instances.get(&relation_instance_id).is_some());
        assert_eq!(relation_instance_type_id, flow_type.relation_instances.get(&relation_instance_id).unwrap().ty);
        assert_eq!(instance_id, flow_type.relation_instances.get(&relation_instance_id).unwrap().instance_id());
        assert_eq!(entity_id, flow_type.relation_instances.get(&relation_instance_id).unwrap().outbound_id);
        assert_eq!(wrapper_id, flow_type.relation_instances.get(&relation_instance_id).unwrap().inbound_id);
        assert_eq!(1, flow_type.variables.len());
        assert!(flow_type.get_variable("variable_name").is_some());
        assert_eq!(1, flow_type.extensions.len());
        assert!(flow_type.get_own_extension(&extension_ty).is_some());
    }

    // TODO: Implement unit tests:
    // - Deserialize description optional
    // - Deserialize missing wrapper entity instance
    // - Deserialize invalid entity instance
    // - Deserialize invalid relation instance
    // - Deserialize variables optional
    // - Deserialize invalid variable
    // - Deserialize extensions optional
    // - Deserialize invalid extension
    // - Deserialize invalid namespace
    // - Deserialize invalid type name
    // - Deserialize missing type

    #[test]
    fn flow_type_ser_test() {
        let flow_type = FlowType::random_type().unwrap();
        println!("{}", serde_json::to_string_pretty(&flow_type).expect("Failed to serialize flow type"));
    }

    #[test]
    fn flow_type_json_schema() {
        let schema = schema_for!(FlowType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

    #[test]
    fn flow_type_dynamic_json_schema() {
        let flow_type = FlowType::random_type().unwrap();
        let entity_type = EntityType::builder()
            .ty(flow_type.wrapper_type())
            .description(r_string())
            .components(ComponentTypeIds::random_type_ids().unwrap())
            .properties(PropertyTypes::random_types(1..5).unwrap())
            .extensions(Extensions::random_types(1..3).unwrap())
            .build();
        // The dynamic JSON schema of the flow type depends on the entity type of the wrapper entity instance.
        let schema = flow_type
            .json_schema(&entity_type)
            .expect("Failed to generate dynamic json schema for flow type!");
        println!("{}", serde_json::to_string_pretty(schema.as_value()).unwrap());
    }
}
