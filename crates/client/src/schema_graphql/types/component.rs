use crate::schema_graphql::types::extension::Extension;
use crate::schema_graphql::types::extension::Extensions;
use crate::schema_graphql::types::property_type::PropertyType;
use crate::schema_graphql::types::property_type::PropertyTypes;
use reactive_graph_graph::NamespacedTypeGetter;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;

#[derive(cynic::InputObject, Clone, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
pub struct ComponentTypeId {
    pub name: String,
    pub namespace: String,
}

impl From<reactive_graph_graph::ComponentTypeId> for ComponentTypeId {
    fn from(ty: reactive_graph_graph::ComponentTypeId) -> Self {
        ComponentTypeId {
            name: ty.type_name(),
            namespace: ty.namespace(),
        }
    }
}

impl From<ComponentTypeId> for reactive_graph_graph::ComponentTypeId {
    fn from(ty: ComponentTypeId) -> Self {
        reactive_graph_graph::ComponentTypeId::new_from_type(&ty.namespace, &ty.name)
    }
}

#[derive(Clone, Debug)]
pub struct ComponentTypeIds(pub Vec<ComponentTypeId>);

impl From<ComponentTypeIds> for reactive_graph_graph::ComponentTypeIds {
    fn from(component_type_ids: ComponentTypeIds) -> Self {
        component_type_ids
            .0
            .into_iter()
            .map(|ty: ComponentTypeId| {
                let ty: reactive_graph_graph::ComponentTypeId = ty.into();
                ty
            })
            .collect()
    }
}

impl FromIterator<Component> for reactive_graph_graph::ComponentTypeIds {
    fn from_iter<I: IntoIterator<Item = Component>>(iter: I) -> Self {
        let tys = reactive_graph_graph::ComponentTypeIds::new();
        for component in iter {
            tys.insert(component.ty().into());
        }
        tys
    }
}

impl fmt::Display for ComponentTypeIds {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)
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

impl Component {
    pub fn ty(self) -> ComponentTypeId {
        ComponentTypeId {
            namespace: self.namespace.clone(),
            name: self.name.clone(),
        }
    }
}

impl From<Component> for reactive_graph_graph::Component {
    fn from(component: Component) -> Self {
        let ty = reactive_graph_graph::ComponentTypeId::new_from_type(component.namespace, component.name);
        reactive_graph_graph::Component {
            ty,
            description: component.description,
            properties: PropertyTypes(component.properties).into(),
            extensions: Extensions(component.extensions).into(),
        }
    }
}

impl From<Component> for reactive_graph_graph::ComponentTypeId {
    fn from(component: Component) -> Self {
        reactive_graph_graph::ComponentTypeId::new_from_type(component.namespace, component.name)
    }
}

pub struct Components(pub Vec<Component>);

impl Deref for Components {
    type Target = Vec<Component>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Components> for Vec<reactive_graph_graph::Component> {
    fn from(components: Components) -> Self {
        components.0.into_iter().map(From::from).collect()
    }
}

impl From<Components> for reactive_graph_graph::ComponentTypeIds {
    fn from(components: Components) -> Self {
        components
            .0
            .into_iter()
            .map(|c| {
                let ty: reactive_graph_graph::ComponentTypeId = c.into();
                ty
            })
            .collect()
    }
}

impl From<Vec<Component>> for Components {
    fn from(components: Vec<Component>) -> Self {
        Components(components)
    }
}
