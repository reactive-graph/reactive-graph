use std::fmt::Display;
use std::fmt::Formatter;

use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;

use crate::model::ComponentTypeId;
use crate::model::ComponentTypeIds;
use crate::model::TYPE_ID_TYPE_SEPARATOR;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "ComponentTypeId")]
pub struct ComponentTypeIdDefinition {
    /// The namespace of the component.
    pub namespace: String,

    /// The name of the component.
    #[graphql(name = "name")]
    pub type_name: String,
}

impl From<ComponentTypeIdDefinition> for ComponentTypeId {
    fn from(ty: ComponentTypeIdDefinition) -> Self {
        ComponentTypeId::new_from_type(ty.namespace, ty.type_name)
    }
}

impl Display for ComponentTypeIdDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "c{}{}{}{}", &TYPE_ID_TYPE_SEPARATOR, &self.namespace, &TYPE_ID_TYPE_SEPARATOR, &self.type_name)
    }
}

pub struct ComponentTypeIdDefinitions(pub Vec<ComponentTypeIdDefinition>);

impl ComponentTypeIdDefinitions {
    pub fn new(tys: Vec<ComponentTypeIdDefinition>) -> Self {
        Self(tys)
    }
}

impl Default for ComponentTypeIdDefinitions {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<ComponentTypeIdDefinitions> for ComponentTypeIds {
    fn from(tys: ComponentTypeIdDefinitions) -> Self {
        tys.0.into_iter().map(|ty| ty.into()).collect()
    }
}
