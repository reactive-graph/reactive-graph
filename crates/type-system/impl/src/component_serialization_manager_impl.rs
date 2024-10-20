use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_serde::error::SerializationError;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::ComponentSerializationManager;
use reactive_graph_type_system_api::ComponentSerializeError;

#[derive(Component)]
pub struct ComponentSerializationManagerImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl ComponentSerializationManager for ComponentSerializationManagerImpl {
    async fn serialize(&self, ty: &ComponentTypeId) -> Result<String, ComponentSerializeError> {
        let Some(component) = self.component_manager.get(ty) else {
            return Err(ComponentSerializeError::ComponentNotFound(ty.clone()));
        };
        serde_json::to_string_pretty(&component).map_err(|e| SerializationError::Json(e).into())
    }
}

#[async_trait]
impl Lifecycle for ComponentSerializationManagerImpl {}
