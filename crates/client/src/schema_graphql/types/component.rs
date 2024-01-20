use crate::schema_graphql::types::extension::Extension;
use crate::schema_graphql::types::extension::Extensions;
use crate::schema_graphql::types::property_type::PropertyType;
use crate::schema_graphql::types::property_type::PropertyTypes;
use inexor_rgf_graph::NamespacedTypeGetter;
use std::ops::Deref;

#[derive(cynic::InputObject, Clone, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
pub struct ComponentTypeId {
    pub name: String,
    pub namespace: String,
}

impl From<inexor_rgf_graph::ComponentTypeId> for ComponentTypeId {
    fn from(ty: inexor_rgf_graph::ComponentTypeId) -> Self {
        ComponentTypeId {
            name: ty.type_name(),
            namespace: ty.namespace(),
        }
    }
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
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

impl From<Component> for inexor_rgf_graph::Component {
    fn from(component: Component) -> Self {
        let ty = inexor_rgf_graph::ComponentTypeId::new_from_type(component.namespace, component.name);
        inexor_rgf_graph::Component {
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

impl From<Components> for Vec<inexor_rgf_graph::Component> {
    fn from(components: Components) -> Self {
        components.0.into_iter().map(From::from).collect()
    }
}
