use async_graphql::InputValueError;
use async_graphql::InputValueResult;
use async_graphql::Object;
use async_graphql::Scalar;
use async_graphql::ScalarType;
use async_graphql::Value;
use serde::Deserialize;
use serde::Serialize;

use reactive_graph_graph::NamespacedType;
use reactive_graph_graph::NamespacedTypeConstructor;
use reactive_graph_graph::NamespacedTypeGetter;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct GraphQLNamespace<TY>(TY)
where
    TY: NamespacedTypeGetter + NamespacedTypeConstructor;
impl<TY> GraphQLNamespace<TY>
where
    TY: NamespacedTypeGetter + NamespacedTypeConstructor,
{
    pub fn ty(&self) -> &TY {
        &self.0
    }
}

#[Scalar]
impl<TY> ScalarType for GraphQLNamespace<TY>
where
    TY: NamespacedTypeGetter + NamespacedTypeConstructor + Send + Sync,
{
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(value) => {
                let ty = TY::parse_namespace(&value).map_err(|e| InputValueError::from(e))?;
                Ok(GraphQLNamespace(ty))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn is_valid(value: &Value) -> bool {
        match value {
            Value::String(value) => TY::parse_namespace(value).is_ok(),
            _ => false,
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.namespace().to_string())
    }
}

pub struct GraphQLNamespacedType {
    namespaced_type: NamespacedType,
}

/// Components are composable parts which can be used by types (entity type, relation type).
#[Object(name = "NamespacedType")]
impl GraphQLNamespacedType {
    /// The fully qualified namespace.
    async fn namespace(&self) -> String {
        self.namespaced_type.namespace().to_string()
    }

    /// The parent namespace.
    async fn path(&self) -> String {
        self.namespaced_type.path().to_string()
    }

    /// The type name.
    async fn type_name(&self) -> String {
        self.namespaced_type.type_name().to_string()
    }
}

impl From<NamespacedType> for GraphQLNamespacedType {
    fn from(namespaced_type: NamespacedType) -> Self {
        GraphQLNamespacedType { namespaced_type }
    }
}
