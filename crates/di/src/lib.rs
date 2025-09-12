use log::error;
use rustc_hash::FxHashMap;
use springtime_di::component::Injectable;
use springtime_di::component_registry::ComponentAliasMetadata;
use springtime_di::component_registry::ComponentDefinition;
use springtime_di::component_registry::ComponentDefinitionRegistry;
use springtime_di::component_registry::ComponentDefinitionRegistryError;
use springtime_di::component_registry::ComponentMetadata;
use springtime_di::component_registry::StaticComponentDefinitionRegistry;
use springtime_di::component_registry::conditional::SimpleContextFactory;
use springtime_di::factory::ComponentFactory;
use springtime_di::factory::ScopeFactoryPtr;
use springtime_di::instance_provider::CastFunction;
use springtime_di::instance_provider::ComponentInstanceAnyPtr;
use springtime_di::instance_provider::ComponentInstanceProvider;
use springtime_di::instance_provider::ComponentInstanceProviderError;
use springtime_di::instance_provider::TypedComponentInstanceProvider;
use springtime_di::scope::PROTOTYPE;
use springtime_di::scope::PrototypeScopeFactory;
use springtime_di::scope::SINGLETON;
use springtime_di::scope::SingletonScopeFactory;
use std::any::TypeId;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;

pub static STATIC_COMPONENT_DEFINITION_REGISTRY_WRAPPER: LazyLock<Option<Arc<StaticComponentDefinitionRegistryWrapper>>> = LazyLock::new(|| {
    StaticComponentDefinitionRegistry::new(true, &SimpleContextFactory)
        .ok()
        .map(|definition_registry| Arc::new(StaticComponentDefinitionRegistryWrapper::new(definition_registry)))
});

pub static SHARED_COMPONENT_DEFINITION_REGISTRY: LazyLock<Option<Box<SharedComponentDefinitionRegistry>>> = LazyLock::new(|| {
    STATIC_COMPONENT_DEFINITION_REGISTRY_WRAPPER
        .clone()
        .map(|parent_definition_registry| Box::new(SharedComponentDefinitionRegistry::new(parent_definition_registry.clone())))
});

#[allow(clippy::let_and_return)]
pub fn get_shared_component_definition_registry() -> Option<Box<SharedComponentDefinitionRegistry>> {
    let shared_component_definition_registry = SHARED_COMPONENT_DEFINITION_REGISTRY.clone();
    shared_component_definition_registry
}

pub fn get_shared_component_factory() -> ComponentFactory {
    let Some(definition_registry) = get_shared_component_definition_registry() else {
        panic!("Failed to initialize the shared component definition registry!");
    };
    ComponentFactory::new(
        definition_registry,
        [
            (SINGLETON.to_string(), Box::<SingletonScopeFactory>::default() as ScopeFactoryPtr),
            (PROTOTYPE.to_string(), Box::<PrototypeScopeFactory>::default() as ScopeFactoryPtr),
        ]
        .into_iter()
        .collect(),
    )
}

pub fn get_container<T>() -> Arc<T>
where
    T: Injectable + ?Sized + Send + Sync,
{
    let mut component_factory = get_shared_component_factory();
    // match TypedComponentInstanceProvider::primary_instance_typed::<dyn Runtime + Send + Sync>(&mut component_factory) {
    match TypedComponentInstanceProvider::primary_instance_typed::<T>(&mut component_factory) {
        Ok(runtime) => runtime,
        // Err(e) => {
        //     panic!("{}", e);
        // }
        Err(ComponentInstanceProviderError::NoPrimaryInstance { type_id, type_name }) => {
            error!("Missing component {type_name:?}");
            let instances: Result<Vec<(ComponentInstanceAnyPtr, CastFunction)>, ComponentInstanceProviderError> = component_factory.instances(type_id);
            match instances {
                Ok(instances) => {
                    error!("Component factory has {} instances of type {:?}", instances.len(), type_id);
                    for (component_instance, _) in instances {
                        error!("  Type Id: {:?}", component_instance.type_id());
                    }
                }
                Err(e) => {
                    error!("None. {e:?}");
                }
            }
            panic!("Cannot find a primary instance for component '{type_id:?}/{type_name:?}' - either none or multiple exists without a primary marker.");
        }
        Err(ComponentInstanceProviderError::IncompatibleComponent { type_id, type_name }) => {
            panic!("Tried to downcast component to incompatible type: {type_id:?}/{type_name}");
        }
        Err(ComponentInstanceProviderError::NoNamedInstance(type_name)) => {
            panic!("Cannot find named component: {type_name}");
        }
        Err(ComponentInstanceProviderError::UnrecognizedScope(scope)) => {
            panic!("Unrecognized scope: {scope}");
        }
        Err(ComponentInstanceProviderError::DependencyCycle { type_id, type_name }) => {
            panic!("Detected dependency cycle for: {type_id:?}/{type_name:?}");
        }
        Err(ComponentInstanceProviderError::ConstructorError(constructor_error)) => {
            panic!("Error in component constructor: {constructor_error}");
        }
    }
}

#[derive(Debug)]
pub struct StaticComponentDefinitionRegistryWrapper {
    pub definition_registry: RwLock<StaticComponentDefinitionRegistry>,
}

impl StaticComponentDefinitionRegistryWrapper {
    pub fn new(definition_registry: StaticComponentDefinitionRegistry) -> Self {
        Self {
            definition_registry: RwLock::new(definition_registry),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SharedComponentDefinitionRegistry {
    definition_registry: Arc<StaticComponentDefinitionRegistryWrapper>,
}

impl SharedComponentDefinitionRegistry {
    pub fn new(definition_registry: Arc<StaticComponentDefinitionRegistryWrapper>) -> Self {
        Self { definition_registry }
    }
}

impl ComponentDefinitionRegistry for SharedComponentDefinitionRegistry {
    #[inline]
    fn register_component(&mut self, target: TypeId, target_name: &str, metadata: &ComponentMetadata) -> Result<(), ComponentDefinitionRegistryError> {
        {
            let mut parent_definition_registry = self.definition_registry.definition_registry.write().unwrap();
            parent_definition_registry.register_component(target, target_name, metadata)
        }
    }

    #[inline]
    fn register_alias(
        &mut self,
        alias_type: TypeId,
        target_type: TypeId,
        alias_name: &str,
        target_name: &str,
        metadata: &ComponentAliasMetadata,
    ) -> Result<(), ComponentDefinitionRegistryError> {
        {
            let mut parent_definition_registry = self.definition_registry.definition_registry.write().unwrap();
            parent_definition_registry.register_alias(alias_type, target_type, alias_name, target_name, metadata)
        }
    }

    #[inline]
    fn components_by_type(&self, type_id: TypeId) -> Vec<ComponentDefinition> {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.components_by_type(type_id)
        }
    }

    #[inline]
    fn component_by_name(&self, name: &str, type_id: TypeId) -> Option<ComponentDefinition> {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.component_by_name(name, type_id)
        }
    }

    #[inline]
    fn primary_component(&self, type_id: TypeId) -> Option<ComponentDefinition> {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.primary_component(type_id)
        }
    }

    #[inline]
    fn is_registered(&self, type_id: TypeId) -> bool {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.is_registered(type_id)
        }
    }

    #[inline]
    fn is_name_registered(&self, name: &str) -> bool {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.is_name_registered(name)
        }
    }

    #[inline]
    fn all_definitions(&self) -> FxHashMap<TypeId, Vec<ComponentDefinition>> {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.all_definitions()
        }
    }
}
