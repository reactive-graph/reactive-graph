use crate::schema::types::extension::Extension;
use crate::schema::types::extension::Extensions;
use crate::schema::types::property_type::PropertyType;
use crate::schema::types::property_type::PropertyTypes;
use inexor_rgf_graph::NamespacedTypeGetter;
use std::ops::Deref;

#[derive(cynic::InputObject, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct ComponentTypeId {
    pub name: String,
    pub namespace: String,
}

impl From<crate::model::ComponentTypeId> for ComponentTypeId {
    fn from(ty: crate::model::ComponentTypeId) -> Self {
        ComponentTypeId {
            name: ty.type_name(),
            namespace: ty.namespace(),
        }
    }
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct Component {
    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    pub description: String,

    /// The property types.
    pub properties: Vec<PropertyType>,

    /// The extensions.
    pub extensions: Vec<Extension>,
}

impl From<Component> for crate::model::Component {
    fn from(component: Component) -> Self {
        let ty = crate::model::ComponentTypeId::new_from_type(component.namespace, component.name);
        crate::model::Component {
            ty,
            description: component.description,
            properties: PropertyTypes(component.properties).into(),
            extensions: Extensions(component.extensions).into(),
        }
    }
}

pub struct Components(pub Vec<Component>);

impl Deref for Components {
    type Target = Vec<Component>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Components> for Vec<crate::model::Component> {
    fn from(components: Components) -> Self {
        components.0.into_iter().map(From::from).collect()
    }
}
