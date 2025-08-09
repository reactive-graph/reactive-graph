use async_trait::async_trait;
use log::debug;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::NamespacedType;
use reactive_graph_graph::NamespacedTypes;
use reactive_graph_type_system_api::NamespaceManager;
use reactive_graph_type_system_api::error::namespace::NamespacedTypeRegistrationError;

#[derive(Component)]
pub struct NamespaceManagerImpl {
    #[component(default = "NamespacedTypes::new")]
    namespaced_types: NamespacedTypes,
}

#[async_trait]
#[component_alias]
impl NamespaceManager for NamespaceManagerImpl {
    fn register(&self, ty: NamespacedType) -> Result<NamespacedType, NamespacedTypeRegistrationError> {
        if self.namespaced_types.contains(&ty) {
            return Err(NamespacedTypeRegistrationError::NamespacedTypeAlreadyExists(ty));
        }
        self.namespaced_types.insert(ty.clone());
        debug!("Registered namespaced type {ty}");
        Ok(ty)
    }

    fn get_all(&self) -> NamespacedTypes {
        self.namespaced_types.clone()
    }

    fn has(&self, ty: &NamespacedType) -> bool {
        self.namespaced_types.contains(ty)
    }

    fn count(&self) -> usize {
        self.namespaced_types.len()
    }

    fn delete(&self, ty: &NamespacedType) -> bool {
        self.namespaced_types.remove(ty).is_some()
    }
}
