use async_graphql::*;

use crate::graphql::query::GraphQLPropertyType;
use crate::model::Component;

pub struct GraphQLComponent {
    component: Component,
}

/// Entity types defines the type of an entity instance.
#[Object(name = "Component")]
impl GraphQLComponent {
    /// The name of the component.
    async fn name(&self) -> String {
        self.component.name.clone()
    }

    /// Textual description of the component.
    async fn description(&self) -> String {
        self.component.description.clone()
    }

    /// The properties which are applied on entity or relation instances.
    async fn properties(&self, name: Option<String>) -> Vec<GraphQLPropertyType> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .component
                .properties
                .to_vec()
                .iter()
                .filter(|property_type| property_type.name == name.clone())
                .cloned()
                .map(|property_type| property_type.into())
                .collect();
        }
        self.component
            .properties
            .to_vec()
            .into_iter()
            .map(|property_type| property_type.into())
            .collect()
    }
}

impl From<Component> for GraphQLComponent {
    fn from(component: Component) -> Self {
        GraphQLComponent { component }
    }
}
