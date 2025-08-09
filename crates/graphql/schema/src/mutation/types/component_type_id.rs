use std::fmt::Display;
use std::fmt::Formatter;

use async_graphql::InputObject;

use serde::Deserialize;
use serde::Serialize;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::NAMESPACE_SEPARATOR;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionConversionError;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "ComponentTypeId")]
pub struct ComponentTypeIdDefinition {
    /// The fully qualified namespace of the component.
    pub name: String,
    // /// The name of the component.
    // #[graphql(name = "name")]
    // pub type_name: String,
}

impl TryFrom<ComponentTypeIdDefinition> for ComponentTypeId {
    type Error = TypeDefinitionConversionError;

    fn try_from(ty: ComponentTypeIdDefinition) -> Result<Self, Self::Error> {
        let x = ComponentTypeId::try_from(ty.name);
        ComponentTypeId::new_from_type(ty.namespace, ty.type_name)
    }
}

impl From<ComponentTypeId> for ComponentTypeIdDefinition {
    fn from(ty: ComponentTypeId) -> Self {
        ComponentTypeIdDefinition {
            namespace: ty.namespace(),
            type_name: ty.type_name(),
        }
    }
}

impl Display for ComponentTypeIdDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "c{}{}{}{}", &NAMESPACE_SEPARATOR, &self.namespace, &NAMESPACE_SEPARATOR, &self.type_name)
    }
}

#[derive(Default)]
pub struct ComponentTypeIdDefinitions(pub Vec<ComponentTypeIdDefinition>);

impl ComponentTypeIdDefinitions {
    pub fn new(tys: Vec<ComponentTypeIdDefinition>) -> Self {
        Self(tys)
    }
}

impl From<ComponentTypeIdDefinitions> for ComponentTypeIds {
    fn from(tys: ComponentTypeIdDefinitions) -> Self {
        tys.0.into_iter().map(|ty| ty.into()).collect()
    }
}
