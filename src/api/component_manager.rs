use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::Component;
use crate::model::PropertyType;
use crate::plugins::ComponentProvider;

#[async_trait]
pub trait ComponentManager: Send + Sync + Lifecycle {
    fn register(&self, component: Component);
    // fn load_static_components(&self);
    fn get_components(&self) -> Vec<Component>;

    fn has(&self, name: String) -> bool;
    fn get(&self, name: String) -> Option<Component>;

    fn create(&self, name: String, properties: Vec<PropertyType>);
    fn delete(&self, name: String);

    fn import(&self, path: String);
    fn export(&self, name: String, path: String);

    fn add_provider(&self, component_provider: Arc<dyn ComponentProvider>);
}
