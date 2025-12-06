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
use schemars::json_schema;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

use crate::AddExtensionError;
use crate::AddPropertyError;
use crate::ComponentTypeId;
use crate::ComponentTypeIdContainer;
use crate::ComponentTypeIds;
use crate::Components;
use crate::EntityType;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::Extensions;
use crate::InboundOutboundType;
use crate::JSON_SCHEMA_ID_URI_PREFIX;
use crate::MatchingInboundOutboundType;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeComponentPropertiesContainer;
use crate::NamespacedTypeComponentTypeIdContainer;
use crate::NamespacedTypeContainer;
use crate::NamespacedTypeExtensionContainer;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypePropertyTypeContainer;
use crate::PropertyType;
use crate::PropertyTypeContainer;
use crate::PropertyTypes;
use crate::RelationComponentTypeId;
use crate::RelationComponentTypeIds;
use crate::RelationTypeAddComponentError;
use crate::RelationTypeAddExtensionError;
use crate::RelationTypeAddPropertyError;
use crate::RelationTypeId;
use crate::RelationTypeIds;
use crate::RelationTypeMergeComponentPropertiesError;
use crate::RelationTypeMergeError;
use crate::RelationTypeMergeExtensionsError;
use crate::RelationTypeMergePropertiesError;
use crate::RelationTypeRemoveComponentError;
use crate::RelationTypeRemoveExtensionError;
use crate::RelationTypeRemovePropertyError;
use crate::RelationTypeUpdateExtensionError;
use crate::RelationTypeUpdatePropertyError;
use crate::RemoveExtensionError;
use crate::RemovePropertyError;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeDefinitionJsonSchema;
use crate::TypeDefinitionJsonSchemaGetter;
use crate::TypeDescriptionGetter;
use crate::TypeIdType;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;
use crate::divergent::DivergentPropertyTypes;
use crate::namespace::Namespace;

#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildType;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedType;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeIds;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypes;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;

pub const JSON_SCHEMA_ID_RELATION_TYPE: &str = formatcp!("{}/relation-type.schema.json", JSON_SCHEMA_ID_URI_PREFIX);

/// A relation type defines the type of relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also, the relation type defines the properties of the relation instance.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[serde(tag = "$id", rename = "https://schema.reactive-graph.io/schema/json/relation-type.schema.json")]
#[schemars(
    title = "RelationType",
    deny_unknown_fields,
    extend("$id" = JSON_SCHEMA_ID_RELATION_TYPE),
    transform = add_json_schema_id_property
)]
pub struct RelationType {
    /// The outbound component or entity type.
    #[serde(rename = "outbound", alias = "outbound")]
    #[builder(setter(into))]
    pub outbound_type: InboundOutboundType,

    /// The fully qualified namespace of the relation type.
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub ty: RelationTypeId,

    /// The inbound component or entity type.
    #[serde(rename = "inbound", alias = "inbound")]
    #[builder(setter(into))]
    pub inbound_type: InboundOutboundType,

    /// Textual description of the relation type.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub description: String,

    /// The names of the components of the relation type.
    #[serde(default = "ComponentTypeIds::new")]
    #[builder(default, setter(into))]
    pub components: ComponentTypeIds,

    /// The properties which are defined by the relation type.
    #[serde(default = "PropertyTypes::new")]
    #[builder(default, setter(into))]
    pub properties: PropertyTypes,

    /// Relation type specific extensions.
    #[serde(default = "Extensions::new")]
    #[builder(default, setter(into))]
    pub extensions: Extensions,
}

impl RelationType {
    #[allow(clippy::too_many_arguments)]
    pub fn new<
        OT: Into<InboundOutboundType>,
        RT: Into<RelationTypeId>,
        IT: Into<InboundOutboundType>,
        D: Into<String>,
        C: Into<ComponentTypeIds>,
        P: Into<PropertyTypes>,
        E: Into<Extensions>,
    >(
        outbound_type: OT,
        ty: RT,
        inbound_type: IT,
        description: D,
        components: C,
        properties: P,
        extensions: E,
    ) -> RelationType {
        RelationType {
            outbound_type: outbound_type.into(),
            ty: ty.into(),
            inbound_type: inbound_type.into(),
            description: description.into(),
            components: components.into(),
            properties: properties.into(),
            extensions: extensions.into(),
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn builder_with_ty<O: Into<InboundOutboundType>, T: Into<RelationTypeId>, I: Into<InboundOutboundType>>(
        outbound_type: O,
        ty: T,
        inbound_type: I,
    ) -> RelationTypeBuilder<((InboundOutboundType,), (RelationTypeId,), (InboundOutboundType,), (), (), (), ())> {
        RelationType::builder().outbound_type(outbound_type).ty(ty).inbound_type(inbound_type)
    }

    pub fn is_outbound(&self, entity_type: &EntityType) -> bool {
        match &self.outbound_type {
            InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(outbound_component_ty)) => entity_type
                .components
                .iter()
                .any(|entity_component_ty| entity_component_ty.eq(outbound_component_ty)),
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)) => &entity_type.ty == ty,
            InboundOutboundType::Component(MatchingInboundOutboundType::Any) | InboundOutboundType::EntityType(MatchingInboundOutboundType::Any) => true,
        }
    }

    pub fn is_inbound(&self, entity_type: &EntityType) -> bool {
        match &self.inbound_type {
            InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(inbound_component_ty)) => entity_type
                .components
                .iter()
                .any(|entity_component_ty| entity_component_ty.eq(inbound_component_ty)),
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)) => &entity_type.ty == ty,
            InboundOutboundType::Component(MatchingInboundOutboundType::Any) | InboundOutboundType::EntityType(MatchingInboundOutboundType::Any) => true,
        }
    }
}

impl ComponentTypeIdContainer for RelationType {
    fn is_a(&self, ty: &ComponentTypeId) -> bool {
        self.components.is_a(ty)
    }

    fn add_component<T: Into<ComponentTypeId>>(&self, ty: T) -> bool {
        self.components.add_component(ty)
    }

    fn add_components<C: Into<ComponentTypeIds>>(&mut self, components_to_add: C) {
        self.components.add_components(components_to_add)
    }

    fn remove_component(&self, ty: &ComponentTypeId) -> Option<ComponentTypeId> {
        self.components.remove(ty)
    }

    fn remove_components<C: Into<ComponentTypeIds>>(&mut self, components_to_remove: C) {
        self.components.remove_components(components_to_remove)
    }

    fn get_components_cloned(&self) -> ComponentTypeIds {
        self.components.clone()
    }
}

impl PropertyTypeContainer for RelationType {
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool {
        self.properties.has_own_property(property_name)
    }

    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType> {
        self.properties.get_own_property(property_name)
    }

    fn add_property<S: Into<PropertyType>>(&self, property_type: S) -> Result<PropertyType, AddPropertyError> {
        self.properties.add_property(property_type)
    }

    fn update_property<N: Into<String>, S: Into<PropertyType>>(&self, property_name: N, property_type: S) -> Result<PropertyType, UpdatePropertyError> {
        self.properties.update_property(property_name, property_type)
    }

    fn remove_property<S: Into<String>>(&self, property_name: S) -> Result<PropertyType, RemovePropertyError> {
        self.properties.remove_property(property_name)
    }

    fn merge_properties<P: Into<PropertyTypes>>(&mut self, properties_to_merge: P) {
        self.properties.merge_properties(properties_to_merge)
    }

    fn merge_non_existent_properties<P: Into<PropertyTypes>>(&self, properties_to_merge: P) -> DivergentPropertyTypes {
        self.properties.merge_non_existent_properties(properties_to_merge)
    }

    fn get_own_properties_cloned(&self) -> PropertyTypes {
        self.properties.clone()
    }
}

impl ExtensionContainer for RelationType {
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

impl NamespacedTypeGetter for RelationType {
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

impl TypeDefinitionGetter for RelationType {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::RelationType
    }
}

impl TypeDescriptionGetter for RelationType {
    fn description(&self) -> String {
        self.description.clone()
    }
}

impl TypeDefinitionJsonSchemaGetter for RelationType {
    fn json_schema(&self) -> Schema {
        TypeDefinitionJsonSchema::new(self)
            .description(&self.description)
            .required_id_property("outbound_id")
            .required_string_property("instance_id")
            .required_id_property("inbound_id")
            .into()
    }
}

impl AsRef<RelationTypeId> for RelationType {
    fn as_ref(&self) -> &RelationTypeId {
        &self.ty
    }
}

impl PartialEq<RelationTypeId> for RelationType {
    fn eq(&self, ty: &RelationTypeId) -> bool {
        self.ty == *ty
    }
}

impl PartialOrd<Self> for RelationType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RelationType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ty.cmp(&other.ty)
    }
}

impl From<&RelationType> for TypeDefinition {
    fn from(relation_type: &RelationType) -> Self {
        relation_type.type_definition()
    }
}

impl From<&RelationType> for Schema {
    fn from(relation_type: &RelationType) -> Self {
        relation_type.json_schema()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RelationTypes(DashMap<RelationTypeId, RelationType>);

impl RelationTypes {
    #[inline]
    pub fn new() -> Self {
        NamespacedTypeContainer::new()
    }

    #[inline]
    pub fn relation<R: Into<RelationType>>(self, relation_type: R) -> Self {
        NamespacedTypeContainer::push(&self, relation_type);
        self
    }

    #[inline]
    pub fn push<R: Into<RelationType>>(&self, relation_type: R) -> Option<RelationType> {
        NamespacedTypeContainer::push(self, relation_type)
    }

    pub fn merge<C: Into<RelationType>>(&self, relation_type_to_merge: C) -> Result<RelationType, RelationTypeMergeError> {
        let relation_type_to_merge = relation_type_to_merge.into();
        let Some(mut relation_type) = self.get_mut(&relation_type_to_merge.ty) else {
            return Err(RelationTypeMergeError::RelationTypeDoesNotExist(relation_type_to_merge.ty));
        };
        relation_type.description = relation_type_to_merge.description;
        // TODO: inbound types
        // TODO: outbound types
        relation_type.add_components(relation_type_to_merge.components);
        relation_type.merge_properties(relation_type_to_merge.properties);
        relation_type.merge_extensions(relation_type_to_merge.extensions);
        Ok(relation_type.clone())
    }
}

impl NamespacedTypeContainer for RelationTypes {
    type TypeId = RelationTypeId;
    type TypeIds = RelationTypeIds;
    type Type = RelationType;
}

impl NamespacedTypeComponentTypeIdContainer<RelationTypeId, RelationTypeAddComponentError, RelationTypeRemoveComponentError> for RelationTypes {
    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> Self {
        self.0
            .iter()
            .filter(|relation_type| relation_type.is_a(component_ty))
            .map(|relation_type| relation_type.value().clone())
            .collect()
    }

    fn add_component(&self, relation_ty: &RelationTypeId, component_ty: &ComponentTypeId) -> Result<(), RelationTypeAddComponentError> {
        let Some(relation_type) = self.0.get_mut(relation_ty) else {
            return Err(RelationTypeAddComponentError::RelationTypeDoesNotExist(relation_ty.clone()));
        };
        if relation_type.is_a(component_ty) {
            return Err(RelationTypeAddComponentError::IsAlreadyA(component_ty.clone()));
        }
        let _ = relation_type.add_component(component_ty);
        Ok(())
    }

    fn remove_component(&self, relation_ty: &RelationTypeId, component_ty: &ComponentTypeId) -> Result<ComponentTypeId, RelationTypeRemoveComponentError> {
        let Some(relation_type) = self.0.get_mut(relation_ty) else {
            return Err(RelationTypeRemoveComponentError::RelationTypeDoesNotExist(relation_ty.clone()));
        };
        relation_type
            .remove_component(component_ty)
            .ok_or(RelationTypeRemoveComponentError::IsNotA(component_ty.clone()))
    }

    fn get_component_type_ids(&self) -> ComponentTypeIds {
        self.0.iter().map(|relation_type| relation_type.components.clone()).collect()
    }
}

impl
    NamespacedTypePropertyTypeContainer<
        RelationTypeId,
        RelationTypeAddPropertyError,
        RelationTypeUpdatePropertyError,
        RelationTypeRemovePropertyError,
        RelationTypeMergePropertiesError,
    > for RelationTypes
{
    fn add_property<P: Into<PropertyType>>(&self, relation_ty: &RelationTypeId, property_type: P) -> Result<PropertyType, RelationTypeAddPropertyError> {
        let Some(relation_type) = self.0.get_mut(relation_ty) else {
            return Err(RelationTypeAddPropertyError::RelationTypeDoesNotExist(relation_ty.clone()));
        };
        relation_type
            .add_property(property_type)
            .map_err(RelationTypeAddPropertyError::AddPropertyError)
    }

    fn update_property<N: Into<String>, P: Into<PropertyType>>(
        &self,
        relation_ty: &RelationTypeId,
        property_name: N,
        property_type: P,
    ) -> Result<PropertyType, RelationTypeUpdatePropertyError> {
        let Some(relation_type) = self.0.get_mut(relation_ty) else {
            return Err(RelationTypeUpdatePropertyError::RelationTypeDoesNotExist(relation_ty.clone()));
        };
        relation_type
            .update_property(property_name, property_type)
            .map_err(RelationTypeUpdatePropertyError::UpdatePropertyError)
    }

    fn remove_property<P: Into<String>>(&self, relation_ty: &RelationTypeId, property_name: P) -> Result<PropertyType, RelationTypeRemovePropertyError> {
        let Some(relation_type) = self.0.get_mut(relation_ty) else {
            return Err(RelationTypeRemovePropertyError::RelationTypeDoesNotExist(relation_ty.clone()));
        };
        relation_type
            .remove_property(property_name)
            .map_err(RelationTypeRemovePropertyError::RemovePropertyError)
    }

    fn merge_properties<P: Into<PropertyTypes>>(&self, relation_ty: &RelationTypeId, properties_to_merge: P) -> Result<(), RelationTypeMergePropertiesError> {
        let Some(mut relation_type) = self.0.get_mut(relation_ty) else {
            return Err(RelationTypeMergePropertiesError::RelationTypeDoesNotExist(relation_ty.clone()));
        };
        relation_type.merge_properties(properties_to_merge);
        Ok(())
    }
}

impl NamespacedTypeComponentPropertiesContainer<RelationTypeId, RelationTypeMergeComponentPropertiesError> for RelationTypes {
    fn merge_component_properties<C: Into<Components>>(&self, components: C) -> Result<(), RelationTypeMergeComponentPropertiesError> {
        let components = components.into();
        let lookup_tys = components.type_ids();

        // First check without modification
        let missing_components: RelationComponentTypeIds = self
            .0
            .iter()
            .map(|relation_type| (relation_type.key().clone(), relation_type.components.clone()))
            .flat_map(|(relation_ty, component_tys)| {
                component_tys
                    .into_iter()
                    .map(move |component_ty| RelationComponentTypeId::new(relation_ty.clone(), component_ty.clone()))
            })
            .filter(|relation_component_ty| !lookup_tys.contains(&relation_component_ty.component_ty))
            .collect();
        if !missing_components.is_empty() {
            return Err(RelationTypeMergeComponentPropertiesError::ComponentDoesNotExist(missing_components));
        }

        // Modification
        self.0.iter_mut().for_each(|relation_type| {
            for component_ty in relation_type.components.iter() {
                if let Some(component) = components.get(component_ty.key()) {
                    relation_type.merge_non_existent_properties(component.properties);
                }
            }
        });
        Ok(())
    }
}

impl
    NamespacedTypeExtensionContainer<
        RelationTypeId,
        RelationTypeAddExtensionError,
        RelationTypeUpdateExtensionError,
        RelationTypeRemoveExtensionError,
        RelationTypeMergeExtensionsError,
    > for RelationTypes
{
    fn add_extension<E: Into<Extension>>(&self, relation_ty: &RelationTypeId, extension: E) -> Result<ExtensionTypeId, RelationTypeAddExtensionError> {
        let Some(relation_type) = self.0.get_mut(relation_ty) else {
            return Err(RelationTypeAddExtensionError::RelationTypeDoesNotExist(relation_ty.clone()));
        };
        relation_type.add_extension(extension).map_err(RelationTypeAddExtensionError::AddExtensionError)
    }

    fn update_extension<T: Into<ExtensionTypeId>, E: Into<Extension>>(
        &self,
        relation_ty: &RelationTypeId,
        extension_ty: T,
        extension: E,
    ) -> Result<Extension, RelationTypeUpdateExtensionError> {
        let Some(relation_type) = self.0.get_mut(relation_ty) else {
            return Err(RelationTypeUpdateExtensionError::RelationTypeDoesNotExist(relation_ty.clone()));
        };
        relation_type
            .update_extension(extension_ty, extension)
            .map_err(RelationTypeUpdateExtensionError::UpdateExtensionError)
    }

    fn remove_extension<T: Into<ExtensionTypeId>>(&self, relation_ty: &RelationTypeId, extension_ty: T) -> Result<Extension, RelationTypeRemoveExtensionError> {
        let Some(relation_type) = self.0.get_mut(relation_ty) else {
            return Err(RelationTypeRemoveExtensionError::RelationTypeDoesNotExist(relation_ty.clone()));
        };
        relation_type
            .remove_extension(extension_ty)
            .map_err(RelationTypeRemoveExtensionError::RemoveExtensionError)
    }

    fn merge_extensions<E: Into<Extensions>>(&mut self, relation_ty: &RelationTypeId, extensions_to_merge: E) -> Result<(), RelationTypeMergeExtensionsError> {
        let Some(mut relation_type) = self.0.get_mut(relation_ty) else {
            return Err(RelationTypeMergeExtensionsError::RelationTypeDoesNotExist(relation_ty.clone()));
        };
        relation_type.merge_extensions(extensions_to_merge);
        Ok(())
    }
}

impl Deref for RelationTypes {
    type Target = DashMap<RelationTypeId, RelationType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RelationTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for RelationTypes {
    type Item = (RelationTypeId, RelationType);
    type IntoIter = OwningIter<RelationTypeId, RelationType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for RelationTypes {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().all(|self_entity_type| other.contains_key(&self_entity_type.ty))
            && other.iter().all(|other_entity_type| self.contains_key(&other_entity_type.ty))
    }
}

impl Eq for RelationTypes {}

impl Hash for RelationTypes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for RelationTypes {
    fn schema_name() -> Cow<'static, str> {
        "RelationTypes".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<RelationType>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
            "description": "Relation Types",
        })
    }
}

impl From<Vec<RelationType>> for RelationTypes {
    fn from(relation_types: Vec<RelationType>) -> Self {
        Self(
            relation_types
                .into_iter()
                .map(|relation_type| (relation_type.ty.clone(), relation_type))
                .collect(),
        )
    }
}

impl From<RelationTypes> for Vec<RelationType> {
    fn from(relation_types: RelationTypes) -> Self {
        relation_types.to_vec()
    }
}

impl From<&RelationTypes> for Vec<RelationType> {
    fn from(entity_types: &RelationTypes) -> Self {
        entity_types.0.iter().map(|entity_type| entity_type.clone()).collect()
    }
}

impl From<DashMap<RelationTypeId, RelationType>> for RelationTypes {
    fn from(entity_types: DashMap<RelationTypeId, RelationType>) -> Self {
        Self(entity_types)
    }
}

impl From<&DashMap<RelationTypeId, RelationType>> for RelationTypes {
    fn from(entity_types: &DashMap<RelationTypeId, RelationType>) -> Self {
        Self(entity_types.clone())
    }
}

impl From<RelationTypes> for DashMap<RelationTypeId, RelationType> {
    fn from(entity_types: RelationTypes) -> Self {
        entity_types.0
    }
}

impl FromIterator<RelationType> for RelationTypes {
    fn from_iter<I: IntoIterator<Item = RelationType>>(iter: I) -> Self {
        let entity_types = Self::new();
        for entity_type in iter {
            entity_types.insert(entity_type.ty.clone(), entity_type);
        }
        entity_types
    }
}

// Experimental
impl RelationTypeBuilder<((InboundOutboundType,), (RelationTypeId,), (InboundOutboundType,), (), (), (), ())> {
    #[allow(clippy::type_complexity)]
    pub fn component<C: Into<ComponentTypeId>>(
        self,
        component_ty: C,
    ) -> RelationTypeBuilder<((InboundOutboundType,), (RelationTypeId,), (InboundOutboundType,), (), (ComponentTypeIds,), (), ())> {
        let (outbound_type, ty, inbound_type, description, _, properties, extensions) = self.fields;
        RelationTypeBuilder {
            fields: (
                outbound_type,
                ty,
                inbound_type,
                description,
                (ComponentTypeIds::new().component(component_ty),),
                properties,
                extensions,
            ),
            phantom: self.phantom,
        }
    }
}

// Experimental
impl RelationTypeBuilder<((InboundOutboundType,), (RelationTypeId,), (InboundOutboundType,), (), (ComponentTypeIds,), (), ())> {
    #[allow(clippy::type_complexity)]
    pub fn component<C: Into<ComponentTypeId>>(
        self,
        ty: C,
    ) -> RelationTypeBuilder<((InboundOutboundType,), (RelationTypeId,), (InboundOutboundType,), (), (ComponentTypeIds,), (), ())> {
        self.fields.4.0.insert(ty.into());
        self
    }
}

// Experimental
impl RelationTypeBuilder<((InboundOutboundType,), (RelationTypeId,), (InboundOutboundType,), (), (ComponentTypeIds,), (), ())> {
    #[allow(clippy::type_complexity)]
    pub fn property<P: Into<PropertyType>>(
        self,
        property: P,
    ) -> RelationTypeBuilder<(
        (InboundOutboundType,),
        (RelationTypeId,),
        (InboundOutboundType,),
        (),
        (ComponentTypeIds,),
        (PropertyTypes,),
        (),
    )> {
        let (outbound_type, ty, inbound_type, description, components, _, extensions) = self.fields;
        RelationTypeBuilder {
            fields: (
                outbound_type,
                ty,
                inbound_type,
                description,
                components,
                (PropertyTypes::new().property(property),),
                extensions,
            ),
            phantom: self.phantom,
        }
    }
}

// Experimental
impl
    RelationTypeBuilder<(
        (InboundOutboundType,),
        (RelationTypeId,),
        (InboundOutboundType,),
        (),
        (ComponentTypeIds,),
        (PropertyTypes,),
        (),
    )>
{
    #[allow(clippy::type_complexity)]
    pub fn property<P: Into<PropertyType>>(
        self,
        property: P,
    ) -> RelationTypeBuilder<(
        (InboundOutboundType,),
        (RelationTypeId,),
        (InboundOutboundType,),
        (),
        (ComponentTypeIds,),
        (PropertyTypes,),
        (),
    )> {
        self.fields.5.0.push(property.into());
        self
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedType for RelationType {
    type Error = NamespacedTypeError;
    type TypeId = RelationTypeId;

    fn random_type() -> Result<Self, NamespacedTypeError> {
        Self::random_child_type(&Namespace::random_path()?)
    }

    fn random_type_with_id(ty: &Self::TypeId) -> Result<Self, Self::Error> {
        Ok(RelationType::builder()
            .outbound_type(InboundOutboundType::random_type_id()?)
            .ty(ty)
            .inbound_type(InboundOutboundType::random_type_id()?)
            .description(r_string())
            .components(ComponentTypeIds::random_type_ids()?)
            .properties(PropertyTypes::random_types(0..10)?)
            .extensions(Extensions::random_types(0..10)?)
            .build())
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomChildType for RelationType {
    type Error = NamespacedTypeError;

    fn random_child_type(namespace: &Namespace) -> Result<Self, Self::Error> {
        Self::random_type_with_id(&NamespacedType::random_child_type_id(namespace)?.into())
    }
}

#[cfg(any(test, feature = "test"))]
impl RelationTypeBuilder<((InboundOutboundType,), (RelationTypeId,), (InboundOutboundType,), (), (), (), ())> {
    pub fn build_with_defaults(self) -> Result<RelationType, NamespacedTypeError> {
        Ok(self
            .description(r_string())
            .components(ComponentTypeIds::random_type_ids()?)
            .properties(PropertyTypes::random_types(0..10)?)
            .extensions(Extensions::random_types(0..10)?)
            .build())
    }
}

fn add_json_schema_id_property(schema: &mut Schema) {
    crate::json_schema::add_json_schema_id_property(schema, JSON_SCHEMA_ID_RELATION_TYPE);
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use std::str::FromStr;

    use crate::ComponentTypeId;
    use crate::ComponentTypeIdContainer;
    use crate::ComponentTypeIds;
    use crate::EntityTypeId;
    use crate::Extension;
    use crate::ExtensionContainer;
    use crate::ExtensionTypeId;
    use crate::Extensions;
    use crate::InboundOutboundType;
    use crate::MatchingInboundOutboundType;
    use crate::NamespacedTypeGetter;
    use crate::PropertyType;
    use crate::PropertyTypeContainer;
    use crate::PropertyTypes;
    use crate::RandomNamespacedType;
    use crate::RandomNamespacedTypeId;
    use crate::RandomNamespacedTypeIds;
    use crate::RandomNamespacedTypes;
    use crate::RelationType;
    use crate::RelationTypeId;
    use crate::TypeDefinitionJsonSchemaGetter;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn build_relation_type() {
        let description = r_string();
        let outbound_ty = EntityTypeId::random_type_id().unwrap();
        let ty = RelationTypeId::random_type_id().unwrap();
        let inbound_ty = EntityTypeId::random_type_id().unwrap();
        let components = ComponentTypeIds::random_type_ids().unwrap();
        let properties = PropertyTypes::random_types(2..5).unwrap();
        let extensions = Extensions::random_types(1..2).unwrap();
        let relation_type = RelationType::builder()
            .outbound_type(outbound_ty.clone())
            .ty(&ty)
            .inbound_type(inbound_ty.clone())
            .description(&description)
            .components(components.clone())
            .properties(properties.clone())
            .extensions(extensions.clone())
            .build();
        assert_eq!(ty.namespace(), relation_type.namespace());
        assert_eq!(ty.path(), relation_type.path());
        assert_eq!(ty.type_name(), relation_type.type_name());
        assert_eq!(
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(outbound_ty)),
            relation_type.outbound_type
        );
        assert_eq!(
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(inbound_ty)),
            relation_type.inbound_type
        );
        assert_eq!(description, relation_type.description);
        assert_eq!(components.len(), relation_type.components.len());
        assert_eq!(properties.len(), relation_type.properties.len());
        assert_eq!(extensions.len(), relation_type.extensions.len());
    }

    #[test]
    fn create_relation_type() {
        let description = r_string();
        let outbound_ty = EntityTypeId::random_type_id().unwrap();
        let ty = RelationTypeId::random_type_id().unwrap();
        let inbound_ty = EntityTypeId::random_type_id().unwrap();
        let components = ComponentTypeIds::random_type_ids().unwrap();
        let properties = PropertyTypes::random_types(2..5).unwrap();
        let extensions = Extensions::random_types(1..2).unwrap();
        let relation_type = RelationType::new(
            outbound_ty.clone(),
            &ty,
            inbound_ty.clone(),
            &description,
            components.clone(),
            properties.clone(),
            extensions.clone(),
        );
        assert_eq!(ty.namespace(), relation_type.namespace());
        assert_eq!(ty.path(), relation_type.path());
        assert_eq!(ty.type_name(), relation_type.type_name());
        assert_eq!(
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(outbound_ty)),
            relation_type.outbound_type
        );
        assert_eq!(
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(inbound_ty)),
            relation_type.inbound_type
        );
        assert_eq!(description, relation_type.description);
        assert_eq!(components.len(), relation_type.components.len());
        assert_eq!(properties.len(), relation_type.properties.len());
        assert_eq!(extensions.len(), relation_type.extensions.len());
    }

    #[test]
    fn relation_type_is_a_test() {
        let component_ty = ComponentTypeId::from_str("fully::qualified::namespace::Component").unwrap();
        let component_tys = ComponentTypeIds::new().component(&component_ty);
        let relation_type = RelationType::builder()
            .outbound_type(InboundOutboundType::random_type_id().unwrap())
            .ty(RelationTypeId::random_type_id().unwrap())
            .inbound_type(InboundOutboundType::random_type_id().unwrap())
            .components(component_tys)
            .build();
        assert_eq!(1, relation_type.components.len());
        assert!(relation_type.is_a(&component_ty));
        assert!(!relation_type.is_a(&ComponentTypeId::random_type_id().unwrap()));
    }

    #[test]
    fn relation_type_has_property_test() {
        let property_name = r_string();
        let properties = PropertyTypes::new().property(PropertyType::string(&property_name));
        let relation_type = RelationType::builder()
            .outbound_type(InboundOutboundType::random_type_id().unwrap())
            .ty(RelationTypeId::random_type_id().unwrap())
            .inbound_type(InboundOutboundType::random_type_id().unwrap())
            .properties(properties)
            .build();
        assert_eq!(1, relation_type.properties.len());
        assert!(relation_type.has_own_property(property_name));
        assert!(!relation_type.has_own_property(r_string()));
    }

    #[test]
    fn relation_type_has_extension_test() {
        let extension = Extension::random_type().unwrap();
        let extensions = Extensions::new().extension(extension.clone());
        let relation_type = RelationType::builder()
            .outbound_type(InboundOutboundType::random_type_id().unwrap())
            .ty(RelationTypeId::random_type_id().unwrap())
            .inbound_type(InboundOutboundType::random_type_id().unwrap())
            .extensions(extensions)
            .build();
        assert_eq!(1, relation_type.extensions.len());
        assert!(relation_type.has_own_extension(&extension.ty));
        assert!(!relation_type.has_own_extension(&ExtensionTypeId::random_type_id().unwrap()));
    }

    #[test]
    fn relation_type_deserialize_fully_valid_test() {
        let outbound_entity_ty = EntityTypeId::from_str("fully::qualified::namespace::OutboundEntityType").unwrap();
        let outbound_ty = InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(outbound_entity_ty));
        let relation_ty = RelationTypeId::from_str("fully::qualified::namespace::RelationType").unwrap();
        let inbound_component_ty = ComponentTypeId::from_str("fully::qualified::namespace::InboundComponent").unwrap();
        let inbound_ty = InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(inbound_component_ty));
        let component_ty = ComponentTypeId::from_str("fully::qualified::namespace::Component").unwrap();
        let extension_ty = ExtensionTypeId::from_str("fully::qualified::namespace::Extension").unwrap();
        let relation_type = serde_json::from_str::<RelationType>(
            r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "fully::qualified::namespace::RelationType",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "components": [
            "fully::qualified::namespace::Component"
          ],
          "properties": [
                {
                  "name": "property_name",
                  "data_type": "string",
                  "socket_type": "input"
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
        .expect("Failed to deserialize relation type");
        assert_eq!(outbound_ty, relation_type.outbound_type);
        assert_eq!(relation_ty, relation_type.ty);
        assert_eq!(inbound_ty, relation_type.inbound_type);
        assert_eq!("d", relation_type.description);
        assert_eq!(1, relation_type.components.len());
        assert!(relation_type.is_a(&component_ty));
        assert_eq!(1, relation_type.properties.len());
        assert!(relation_type.get_own_property("property_name").is_some());
        assert_eq!(1, relation_type.extensions.len());
        assert!(relation_type.get_own_extension(&extension_ty).is_some());
    }

    #[test]
    fn relation_type_deserialize_description_optional_test() {
        assert!(
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "fully::qualified::namespace::Type",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "components": [],
          "properties": [],
          "extensions": []
        }"#
            )
            .expect("Failed to deserialize relation type")
            .description
            .is_empty()
        );
    }

    #[test]
    fn relation_type_deserialize_components_optional_test() {
        assert_eq!(
            0,
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "fully::qualified::namespace::Type",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "properties": [],
          "extensions": []
        }"#
            )
            .expect("Failed to deserialize relation type")
            .properties
            .len()
        );
    }

    #[test]
    fn relation_type_deserialize_invalid_component_test() {
        assert!(
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "fully::qualified::namespace::Type",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "components": [
            "type": "InvalidTypeName::namespace"
          ],
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn relation_type_deserialize_properties_optional_test() {
        assert_eq!(
            0,
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "fully::qualified::namespace::Type",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "components": [],
          "extensions": []
        }"#
            )
            .expect("Failed to deserialize relation type")
            .properties
            .len()
        );
    }

    #[test]
    fn relation_type_deserialize_invalid_property_test() {
        assert!(
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "fully::qualified::namespace::Type",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "components": [],
          "properties": [
                {
                  "name": "property_name",
                  "data_type": "strng",
                  "socket_type": "put"
                }
          ],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn relation_type_deserialize_extensions_optional_test() {
        assert_eq!(
            0,
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "fully::qualified::namespace::Type",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "components": [],
          "properties": []
        }"#
            )
            .expect("Failed to deserialize relation type")
            .extensions
            .len()
        );
    }

    #[test]
    fn relation_type_deserialize_invalid_extension_test() {
        assert!(
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "fully::qualified::namespace::Type",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "components": [],
          "properties": [],
          "extensions": [
                {
                  "type": "InvalidTypeName::namespace",
                  "extension": ""
                }
          ]
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn relation_type_deserialize_invalid_namespace_test() {
        assert!(
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "invalid::namespace",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "components": [],
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn relation_type_deserialize_invalid_type_test() {
        assert!(
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "InvalidTypeName",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "components": [],
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn relation_type_deserialize_missing_type_test() {
        assert!(
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "components": [],
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn relation_type_deserialize_missing_outbound_type_test() {
        assert!(
            serde_json::from_str::<RelationType>(
                r#"{
          "type": "fully::qualified::namespace::RelationType",
          "inbound": {
            "component": "fully::qualified::namespace::InboundComponent"
          },
          "description": "d",
          "components": [],
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn relation_type_deserialize_missing_inbound_type_test() {
        assert!(
            serde_json::from_str::<RelationType>(
                r#"{
          "outbound": {
            "entity_type": "fully::qualified::namespace::OutboundEntityType"
          },
          "type": "fully::qualified::namespace::RelationType",
          "description": "d",
          "components": [],
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn relation_type_ser_test() {
        let relation_type = RelationType::random_type().unwrap();
        println!("{}", serde_json::to_string_pretty(&relation_type).expect("Failed to serialize relation type"));
    }

    #[test]
    fn relation_type_json_schema() {
        let schema = schema_for!(RelationType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

    #[test]
    fn relation_type_dynamic_json_schema() {
        let relation_type = RelationType::random_type().unwrap();
        let schema = relation_type.json_schema();
        println!("{}", serde_json::to_string_pretty(schema.as_value()).unwrap());
    }
}
