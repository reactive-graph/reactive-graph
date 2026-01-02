use crate::schema_graphql::types::extension::Extension;
use crate::schema_graphql::types::extension::Extensions;
use crate::schema_graphql::types::property_type::PropertyType;
use crate::schema_graphql::types::property_type::PropertyTypes;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::InvalidComponentError;
use reactive_graph_graph::NamespacedTypeParseError;
use serde_json::Value;
use std::ops::Deref;
use std::str::FromStr;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct Component {
    /// The fully qualified namespace of the component.
    #[cynic(rename = "type")]
    pub _type: String,

    /// Textual description of the component.
    pub description: String,

    /// The property types.
    pub properties: Vec<PropertyType>,

    /// The extensions.
    pub extensions: Vec<Extension>,

    /// The JSON schema.
    pub json_schema: Value,
}

impl Component {
    pub fn ty(&self) -> Result<ComponentTypeId, NamespacedTypeParseError> {
        ComponentTypeId::from_str(&self._type)
    }

    // pub fn extensions(&self) -> Extensions {
    //     Extensions(self.extensions.clone())
    // }
}

impl TryFrom<Component> for reactive_graph_graph::Component {
    type Error = InvalidComponentError;

    fn try_from(component: Component) -> Result<Self, Self::Error> {
        Ok(reactive_graph_graph::Component {
            ty: ComponentTypeId::from_str(&component._type).map_err(InvalidComponentError::InvalidComponent)?,
            description: component.description,
            properties: reactive_graph_graph::PropertyTypes::try_from(PropertyTypes(component.properties))
                .map_err(InvalidComponentError::InvalidPropertyType)?,
            extensions: reactive_graph_graph::Extensions::try_from(Extensions(component.extensions)).map_err(InvalidComponentError::InvalidExtension)?,
        })
    }
}

impl TryFrom<Component> for ComponentTypeId {
    type Error = NamespacedTypeParseError;

    fn try_from(component: Component) -> Result<Self, Self::Error> {
        ComponentTypeId::from_str(&component._type)
    }
}

pub struct Components(pub Vec<Component>);

impl Deref for Components {
    type Target = Vec<Component>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<Components> for reactive_graph_graph::Components {
    type Error = InvalidComponentError;

    fn try_from(components: Components) -> Result<Self, Self::Error> {
        let components_2 = reactive_graph_graph::Components::new();
        for component in components.0 {
            components_2.push(reactive_graph_graph::Component::try_from(component)?);
        }
        Ok(components_2)
    }
}

impl From<Vec<Component>> for Components {
    fn from(components: Vec<Component>) -> Self {
        Components(components)
    }
}
