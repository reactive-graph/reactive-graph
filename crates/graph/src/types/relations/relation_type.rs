use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::DashMap;
use dashmap::iter::OwningIter;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
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
use crate::ComponentOrEntityTypeId;
use crate::ComponentTypeId;
use crate::ComponentTypeIdContainer;
use crate::ComponentTypeIds;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::Extensions;
use crate::NamespacedTypeComponentTypeIdContainer;
use crate::NamespacedTypeContainer;
use crate::NamespacedTypeExtensionContainer;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypePropertyTypeContainer;
use crate::PropertyType;
use crate::PropertyTypeContainer;
use crate::PropertyTypes;
use crate::RelationTypeAddComponentError;
use crate::RelationTypeAddExtensionError;
use crate::RelationTypeAddPropertyError;
use crate::RelationTypeId;
use crate::RelationTypeIds;
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
use crate::TypeIdType;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;
#[cfg(any(test, feature = "test"))]
use reactive_graph_test_utils::r_string;

/// A relation type defines the type of relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also, the relation type defines the properties of the relation instance.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[schemars(deny_unknown_fields)]
pub struct RelationType {
    /// The outbound component or entity type.
    #[serde(rename = "outbound", alias = "outbound")]
    #[builder(setter(into))]
    pub outbound_type: ComponentOrEntityTypeId,

    /// The type definition contains the namespace and the type name.
    #[serde(flatten)]
    #[builder(setter(into))]
    pub ty: RelationTypeId,

    /// The inbound component or entity type.
    #[serde(rename = "inbound", alias = "inbound")]
    #[builder(setter(into))]
    pub inbound_type: ComponentOrEntityTypeId,

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
        OT: Into<ComponentOrEntityTypeId>,
        RT: Into<RelationTypeId>,
        IT: Into<ComponentOrEntityTypeId>,
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
    pub fn builder_with_ty<O: Into<ComponentOrEntityTypeId>, T: Into<RelationTypeId>, I: Into<ComponentOrEntityTypeId>>(
        outbound_type: O,
        ty: T,
        inbound_type: I,
    ) -> RelationTypeBuilder<((ComponentOrEntityTypeId,), (RelationTypeId,), (ComponentOrEntityTypeId,), (), (), (), ())> {
        RelationType::builder().outbound_type(outbound_type).ty(ty).inbound_type(inbound_type)
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
        self.properties.merge_properties(properties_to_merge);
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
}

impl NamespacedTypeGetter for RelationType {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for RelationType {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
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
        TypeDefinition {
            type_id_type: TypeIdType::RelationType,
            namespace: relation_type.namespace(),
            type_name: relation_type.type_name(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
// #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RelationTypes(DashMap<RelationTypeId, RelationType>);

impl RelationTypes {
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

    fn new() -> Self {
        Self(DashMap::new())
    }

    fn push<R: Into<RelationType>>(&self, relation_type: R) {
        let relation_type = relation_type.into();
        self.0.insert(relation_type.ty.clone(), relation_type);
    }
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
        let sub_schema: Schema = schema_generator.subschema_for::<RelationType>().into();
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
impl RelationTypeBuilder<((ComponentOrEntityTypeId,), (RelationTypeId,), (ComponentOrEntityTypeId,), (), (), (), ())> {
    #[allow(clippy::type_complexity)]
    pub fn component<C: Into<ComponentTypeId>>(
        self,
        component_ty: C,
    ) -> RelationTypeBuilder<((ComponentOrEntityTypeId,), (RelationTypeId,), (ComponentOrEntityTypeId,), (), (ComponentTypeIds,), (), ())> {
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
impl RelationTypeBuilder<((ComponentOrEntityTypeId,), (RelationTypeId,), (ComponentOrEntityTypeId,), (), (ComponentTypeIds,), (), ())> {
    #[allow(clippy::type_complexity)]
    pub fn component<C: Into<ComponentTypeId>>(
        self,
        ty: C,
    ) -> RelationTypeBuilder<((ComponentOrEntityTypeId,), (RelationTypeId,), (ComponentOrEntityTypeId,), (), (ComponentTypeIds,), (), ())> {
        self.fields.4.0.insert(ty.into());
        self
    }
}

// Experimental
impl RelationTypeBuilder<((ComponentOrEntityTypeId,), (RelationTypeId,), (ComponentOrEntityTypeId,), (), (ComponentTypeIds,), (), ())> {
    #[allow(clippy::type_complexity)]
    pub fn property<P: Into<PropertyType>>(
        self,
        property: P,
    ) -> RelationTypeBuilder<(
        (ComponentOrEntityTypeId,),
        (RelationTypeId,),
        (ComponentOrEntityTypeId,),
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
        (ComponentOrEntityTypeId,),
        (RelationTypeId,),
        (ComponentOrEntityTypeId,),
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
        (ComponentOrEntityTypeId,),
        (RelationTypeId,),
        (ComponentOrEntityTypeId,),
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
impl DefaultTest for RelationType {
    fn default_test() -> Self {
        RelationType::builder()
            .outbound_type(ComponentOrEntityTypeId::default_test())
            .ty(RelationTypeId::default_test())
            .inbound_type(ComponentOrEntityTypeId::default_test())
            .description(r_string())
            .components(ComponentTypeIds::default_test())
            .properties(PropertyTypes::default_test())
            .extensions(Extensions::default_test())
            .build()
    }
}

#[cfg(any(test, feature = "test"))]
impl RelationTypeBuilder<((ComponentOrEntityTypeId,), (RelationTypeId,), (ComponentOrEntityTypeId,), (), (), (), ())> {
    pub fn build_with_defaults(self) -> RelationType {
        self.description(r_string())
            .components(ComponentTypeIds::default_test())
            .properties(PropertyTypes::default_test())
            .extensions(Extensions::default_test())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use serde_json::json;

    use crate::ComponentTypeId;
    use crate::ComponentTypeIdContainer;
    use crate::DataType;
    use crate::EntityTypeId;
    use crate::Extension;
    use crate::ExtensionContainer;
    use crate::ExtensionTypeId;
    use crate::NamespacedTypeGetter;
    use crate::PropertyType;
    use crate::PropertyTypeContainer;
    use crate::RelationType;
    use crate::RelationTypeId;
    use crate::SocketType;
    use crate::TypeDefinitionGetter;
    use crate::TypeIdType;
    use reactive_graph_test_utils::r_string;

    #[test]
    fn create_relation_type_test() {
        let type_name = r_string();
        let outbound_type_name = r_string();
        let inbound_type_name = r_string();

        let namespace = r_string();
        let description = r_string();

        let component_name = r_string();
        let behaviour_name = r_string();
        let property_name = r_string();
        let mut component_names = Vec::new();
        let component_ty = ComponentTypeId::new_from_type(&namespace, &component_name);
        component_names.push(component_ty.clone());
        let mut behaviour_names = Vec::new();
        behaviour_names.push(behaviour_name.clone());
        let mut property_types = Vec::new();
        let property_type = PropertyType::new(property_name.clone(), DataType::String);
        property_types.push(property_type.clone());

        let mut extensions = Vec::new();
        let extension_namespace = r_string();
        let extension_name = r_string();
        let extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_name);
        let extension_value = json!("extension_value");
        let extension = Extension {
            ty: extension_ty.clone(),
            description: r_string(),
            extension: extension_value.clone(),
        };
        extensions.push(extension.clone());
        let other_extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
        let other_extension = Extension::new(&other_extension_ty, r_string(), extension_value.clone());
        extensions.push(other_extension);

        let ty = RelationTypeId::new_from_type(&namespace, &type_name);
        let outbound_type = EntityTypeId::new_from_type(&namespace, &outbound_type_name);
        let inbound_type = EntityTypeId::new_from_type(&namespace, &inbound_type_name);
        let relation_type = RelationType::new(
            outbound_type.clone(),
            ty,
            inbound_type.clone(),
            description.clone(),
            component_names,
            property_types,
            extensions,
        );

        assert_eq!(namespace, relation_type.namespace());
        assert_eq!(type_name, relation_type.type_name());
        assert_eq!(format!("r__{}__{}", &namespace, &type_name), relation_type.type_definition().to_string());
        assert_eq!(outbound_type, relation_type.outbound_type.clone().try_into().unwrap());
        assert_eq!(inbound_type, relation_type.inbound_type.clone().try_into().unwrap());
        assert_eq!(description, relation_type.description);

        assert!(relation_type.components.contains(&component_ty));
        assert!(relation_type.is_a(&component_ty));

        assert!(relation_type.properties.contains_key(&property_name));
        assert!(relation_type.has_own_property(&property_name));
        assert!(!relation_type.has_own_property(r_string()));
        assert_eq!(property_name, relation_type.get_own_property(&property_name).unwrap().name);
        assert_eq!(property_type.data_type, relation_type.get_own_property(&property_name).unwrap().data_type);
        assert_eq!(property_type, relation_type.get_own_property(&property_name).unwrap());

        assert!(relation_type.extensions.contains_key(&extension_ty));
        assert!(relation_type.has_own_extension(&extension_ty));
        assert_eq!(extension_value, relation_type.get_own_extension(&extension_ty).unwrap().extension);
        assert_eq!(extension.extension, relation_type.get_own_extension(&extension_ty).unwrap().extension);

        let non_existing_extension = ExtensionTypeId::new_from_type(r_string(), r_string());
        assert!(!relation_type.extensions.contains_key(&non_existing_extension));
        assert!(!relation_type.has_own_extension(&non_existing_extension));

        // assert_eq!(property_name, *relation_type.properties.first().unwrap().name);
        // assert!(relation_type.has_own_property(property_name.clone()));
        // assert!(!relation_type.has_own_property(r_string()));
        // assert_eq!(property_type.data_type, relation_type.get_own_property(property_name).unwrap().data_type);
        // assert_eq!(&extension_namespace, &relation_type.extensions.first().unwrap().namespace());
        // assert_eq!(&extension_name, &relation_type.extensions.first().unwrap().type_name());
        // assert!(relation_type.has_own_extension(&extension_ty));
        // let non_existing_extension = ExtensionTypeId::new_from_type(r_string(), r_string());
        // assert!(!relation_type.has_own_extension(&non_existing_extension));
        // assert_eq!(extension.extension, relation_type.get_own_extension(&extension_ty).unwrap().extension);
    }

    #[test]
    fn relation_type_ser_test() {
        let oty = EntityTypeId::new_from_type("ono", "oto");
        let rty = RelationTypeId::new_from_type("rnr", "rtr");
        let ity = EntityTypeId::new_from_type("ini", "iti");
        let ty = RelationType::new(oty, rty, ity, "", Vec::new(), Vec::new(), Vec::new());
        println!("{}", serde_json::to_string_pretty(&ty).expect("Failed to serialize relation type"));
    }

    #[test]
    fn relation_type_de_test() {
        let s = r#"{
  "outbound": {
    "entity_type": {
      "namespace": "ono",
      "type_name": "oto"
    }
  },

  "namespace": "rnr",
  "type_name": "rtr",

  "inbound": {
    "component": {
      "namespace": "ini",
      "type_name": "iti"
    }
  },

  "description": "d",
  "components": [
    {
      "namespace": "mno",
      "type_name": "pqr"
    }
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
      "namespace": "ext_namespace",
      "type_name": "ext_name",
      "extension": "ext_value"
    }
  ]
}"#;
        let relation_type: RelationType = serde_json::from_str(s).unwrap();
        assert_eq!("rnr", relation_type.namespace());
        assert_eq!("rtr", relation_type.type_name());
        assert_eq!("r__rnr__rtr", relation_type.ty.to_string());
        assert_eq!(TypeIdType::EntityType, relation_type.outbound_type.type_definition().type_id_type);
        assert_eq!("ono", relation_type.outbound_type.namespace());
        assert_eq!("oto", relation_type.outbound_type.type_name());
        assert_eq!(TypeIdType::Component, relation_type.inbound_type.type_definition().type_id_type);
        assert_eq!("ini", relation_type.inbound_type.namespace());
        assert_eq!("iti", relation_type.inbound_type.type_name());
        assert_eq!("d", relation_type.description);
        assert_eq!(1, relation_type.components.len());
        let component = relation_type.components.get(&ComponentTypeId::new_from_type("mno", "pqr")).unwrap();
        assert_eq!("mno", component.namespace());
        assert_eq!("pqr", component.type_name());
        assert_eq!(1, relation_type.properties.len());
        let property = relation_type.get_own_property("property_name").unwrap();
        assert_eq!("property_name", property.name);
        assert_eq!(DataType::String, property.data_type);
        assert_eq!(SocketType::Input, property.socket_type);
        assert_eq!(1, relation_type.extensions.len());
        let extension = relation_type
            .get_own_extension(&ExtensionTypeId::new_from_type("ext_namespace", "ext_name"))
            .unwrap();
        assert_eq!("ext_namespace", extension.ty.namespace());
        assert_eq!("ext_name", extension.ty.type_name());
        assert_eq!(json!("ext_value"), extension.extension);
    }

    #[test]
    fn relation_type_json_schema() {
        let schema = schema_for!(RelationType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
