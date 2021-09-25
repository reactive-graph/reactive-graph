use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::Component;
use crate::model::PropertyType;
use crate::plugins::ComponentProvider;

#[async_trait]
pub trait ComponentManager: Send + Sync + Lifecycle {
    fn register(&self, component: Component);

    /// Returns all components
    fn get_components(&self) -> Vec<Component>;

    /// Returns true, if a component with the given name exists.
    fn has(&self, name: String) -> bool;

    /// Returns the component with the given name or empty.
    fn get(&self, name: String) -> Option<Component>;

    /// Creates a new component with the given name and the given properties.
    fn create(&self, name: String, properties: Vec<PropertyType>);

    /// Deletes the component with the given name.
    fn delete(&self, name: String);

    fn import(&self, path: String);
    fn export(&self, name: String, path: String);

    fn add_provider(&self, component_provider: Arc<dyn ComponentProvider>);
}
