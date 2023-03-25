use async_graphql::*;

use crate::graphql::mutation::ExtensionTypeIdDefinition;
use crate::graphql::query::GraphQLDataType;
use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::GraphQLMutability;
use crate::graphql::query::GraphQLSocketType;
use crate::model::PropertyType;

pub struct GraphQLPropertyType {
    property_type: PropertyType,
}

/// Property types defines the type of a property instance.
/// The property type defines the name, the data type and
/// the socket type of a property. A property type does not
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

    /// The extensions which are defined by the entity type.
    async fn extensions(&self, #[graphql(name = "type")] extension_ty: Option<ExtensionTypeIdDefinition>) -> Vec<GraphQLExtension> {
        if let Some(extension_ty) = extension_ty {
            let extension_ty = extension_ty.into();
            return self
                .property_type
                .extensions
                .to_vec()
                .iter()
                .filter(|extension| extension.ty == extension_ty)
                .cloned()
                .map(|extension| extension.into())
                .collect();
        }
        self.property_type
            .extensions
            .iter()
            .cloned()
            .map(|property_type| property_type.into())
            .collect()
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
