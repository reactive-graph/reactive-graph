use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::PropertyType;

use crate::query::GraphQLDataType;
use crate::query::GraphQLExtension;
use crate::query::GraphQLExtensions;
use crate::query::GraphQLMutability;
use crate::query::GraphQLSocketType;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphQLPropertyType {
    property_type: PropertyType,
}

/// Property types defines the type of property instance.
/// The property type defines the name, the data type and
/// the socket type of property. A property type does not
/// contain any value.
#[Object(name = "PropertyType")]
impl GraphQLPropertyType {
    /// The name of the component.
    async fn name(&self) -> String {
        self.property_type.name.clone()
    }

    /// Textual description of the component.
    async fn description(&self) -> String {
        self.property_type.description.clone()
    }

    /// The data type of the property instances.
    async fn data_type(&self) -> GraphQLDataType {
        self.property_type.data_type.into()
    }

    /// The socket type of the property instances.
    async fn socket_type(&self) -> GraphQLSocketType {
        self.property_type.socket_type.into()
    }

    /// The property instance is mutable or immutable.
    async fn mutability(&self) -> GraphQLMutability {
        self.property_type.mutability.into()
    }

    /// The extensions which are defined by the property type.
    async fn extensions(
        &self,
        #[graphql(name = "name")] namespace: Option<String>,
        #[graphql(desc = "If true, the extensions are sorted by type")] sort: Option<bool>,
    ) -> Result<Vec<GraphQLExtension>> {
        let ty = match namespace {
            Some(extension_namespace) => Some(ExtensionTypeId::parse_namespace(&extension_namespace)?),
            None => None,
        };
        let extensions: GraphQLExtensions = self
            .property_type
            .extensions
            .iter()
            .filter(|extension| match &ty {
                Some(ty) => &extension.ty == ty,
                None => true,
            })
            .map(|extension| extension.value().clone())
            .collect();
        Ok(if sort.unwrap_or_default() { extensions.sorted() } else { extensions.into() })
    }
}

impl From<PropertyType> for GraphQLPropertyType {
    fn from(property_type: PropertyType) -> Self {
        GraphQLPropertyType { property_type }
    }
}

impl From<&PropertyType> for GraphQLPropertyType {
    fn from(property_type: &PropertyType) -> Self {
        GraphQLPropertyType {
            property_type: property_type.clone(),
        }
    }
}

impl From<GraphQLPropertyType> for PropertyType {
    fn from(property_type: GraphQLPropertyType) -> Self {
        PropertyType::new_with_all(
            property_type.property_type.name,
            property_type.property_type.description,
            property_type.property_type.data_type,
            property_type.property_type.socket_type,
            property_type.property_type.mutability,
            property_type.property_type.extensions,
        )
    }
}

impl From<&GraphQLPropertyType> for PropertyType {
    fn from(property_type: &GraphQLPropertyType) -> Self {
        PropertyType::new_with_all(
            property_type.property_type.name.clone(),
            property_type.property_type.description.clone(),
            property_type.property_type.data_type,
            property_type.property_type.socket_type,
            property_type.property_type.mutability,
            property_type.property_type.extensions.clone(),
        )
    }
}
