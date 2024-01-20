use fxhash::FxHashMap;
use log::info;
use springtime_di::component::Injectable;
use springtime_di::component_registry::conditional::SimpleContextFactory;
use springtime_di::component_registry::ComponentAliasMetadata;
use springtime_di::component_registry::ComponentDefinition;
use springtime_di::component_registry::ComponentDefinitionRegistry;
use springtime_di::component_registry::ComponentDefinitionRegistryError;
use springtime_di::component_registry::ComponentMetadata;
use springtime_di::component_registry::StaticComponentDefinitionRegistry;
use springtime_di::factory::ComponentFactory;
use springtime_di::factory::ScopeFactoryPtr;
use springtime_di::instance_provider::CastFunction;
use springtime_di::instance_provider::ComponentInstanceAnyPtr;
use springtime_di::instance_provider::ComponentInstanceProviderError;
use springtime_di::instance_provider::ComponentInstancePtr;
use springtime_di::instance_provider::TypedComponentInstanceProvider;
use springtime_di::scope::PrototypeScopeFactory;
use springtime_di::scope::SingletonScopeFactory;
use springtime_di::scope::PROTOTYPE;
use springtime_di::scope::SINGLETON;
use std::any::type_name;
use std::any::TypeId;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;

pub static TYPE_SYSTEM_CDR: LazyLock<Option<Arc<StaticComponentDefinitionRegistryWrapper>>> = LazyLock::new(|| {
    let definition_registry = StaticComponentDefinitionRegistry::new(true, &SimpleContextFactory::default())
        .ok()
        .map(|definition_registry| Arc::new(StaticComponentDefinitionRegistryWrapper::new(definition_registry)));
    // .map(|f| Box::new(f));
    // Arc::new(registry)
    // let b = Box::new(x);
    // b
    definition_registry
});

pub static TYPE_SYSTEM_SHARED_CDR: LazyLock<Option<Box<SharedComponentDefinitionRegistry>>> = LazyLock::new(|| {
    let child_definition_registry = TYPE_SYSTEM_CDR
        .clone()
        .map(|parent_definition_registry| Box::new(SharedComponentDefinitionRegistry::new(parent_definition_registry.clone())));
    child_definition_registry
    // if let Some(registry) =  {
    //
    // };
    // None
    // HierarchicStaticComponentDefinitionRegistry::new(TYPE_SYSTEM_CDR)
});

pub fn get_type_system_cdr() -> Option<Box<SharedComponentDefinitionRegistry>> {
    let x = TYPE_SYSTEM_SHARED_CDR.clone();
    x
    // match TYPE_SYSTEM_SHARED_CDR.clone() {
    //     None => {}
    //     Some(_) => {}
    // }
    // let Some(definition_registry) = TYPE_SYSTEM_SHARED_CDR.clone() else {
    //     panic!("No definition registry!");
    // };
    // let mut component_factory = component_factory.with_definition_registry(definition_registry).build();
    //
    // let Some(definition_registry) = TYPE_SYSTEM_SHARED_CDR.clone() else {
    //     panic!("No definition registry!");
    // };
}

pub fn get_shared_component_factory() -> ComponentFactory {
    let Some(definition_registry) = get_type_system_cdr() else {
        panic!("No definition registry!");
    };
    let component_factory = ComponentFactory::new(
        definition_registry,
        [
            (SINGLETON.to_string(), Box::<SingletonScopeFactory>::default() as ScopeFactoryPtr),
            (PROTOTYPE.to_string(), Box::<PrototypeScopeFactory>::default() as ScopeFactoryPtr),
        ]
        .into_iter()
        .collect(),
    );
    component_factory
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

// impl Deref for SharedComponentDefinitionRegistry {
//     type Target = Arc<StaticComponentDefinitionRegistryWrapper>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.definition_registry
//     }
// }

impl ComponentDefinitionRegistry for SharedComponentDefinitionRegistry {
    #[inline]
    fn register_component(&mut self, target: TypeId, target_name: &str, metadata: &ComponentMetadata) -> Result<(), ComponentDefinitionRegistryError> {
        {
            // let x = self.definition_registry;
            let mut parent_definition_registry = self.definition_registry.definition_registry.write().unwrap();
            info!("register_component: {target:?} {target_name}");
            parent_definition_registry.register_component(target, target_name, metadata)
        }
        // let mut p = self.parent.clone();
        // p.register_component(target, target_name, metadata)
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
        // self.parent.register_alias(alias_type, target_type, alias_name, target_name, metadata)
    }

    #[inline]
    fn components_by_type(&self, type_id: TypeId) -> Vec<ComponentDefinition> {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.components_by_type(type_id)
        }
        // self.parent.components_by_type(type_id)
    }

    #[inline]
    fn component_by_name(&self, name: &str, type_id: TypeId) -> Option<ComponentDefinition> {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.component_by_name(name, type_id)
        }
        // self.parent.component_by_name(name, type_id)
    }

    #[inline]
    fn primary_component(&self, type_id: TypeId) -> Option<ComponentDefinition> {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.primary_component(type_id)
        }
        // self.parent.primary_component(type_id)
    }

    #[inline]
    fn is_registered(&self, type_id: TypeId) -> bool {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.is_registered(type_id)
        }
        // self.parent.is_registered(type_id)
    }

    #[inline]
    fn is_name_registered(&self, name: &str) -> bool {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.is_name_registered(name)
        }
        // self.parent.is_name_registered(name)
    }

    #[inline]
    fn all_definitions(&self) -> FxHashMap<TypeId, Vec<ComponentDefinition>> {
        {
            let parent_definition_registry = self.definition_registry.definition_registry.read().unwrap();
            parent_definition_registry.all_definitions()
        }
        // self.parent.all_definitions()
    }
}
//
// impl TypedComponentInstanceProvider for SharedComponentDefinitionRegistry {
//     fn primary_instance_typed<T: Injectable + ?Sized>(&mut self) -> Result<ComponentInstancePtr<T>, ComponentInstanceProviderError> {
//         let type_id = TypeId::of::<T>();
//         self.primary_instance(type_id)
//             .and_then(move |(p, cast)| cast_instance(p, cast, type_id))
//             .map_err(|error| enrich_error::<T>(error))
//     }
//
//     fn primary_instance_option<T: Injectable + ?Sized>(&mut self) -> Result<Option<ComponentInstancePtr<T>>, ComponentInstanceProviderError> {
//         todo!()
//     }
//
//     fn instances_typed<T: Injectable + ?Sized>(&mut self) -> Result<Vec<ComponentInstancePtr<T>>, ComponentInstanceProviderError> {
//         todo!()
//     }
//
//     fn instance_by_name_typed<T: Injectable + ?Sized>(&mut self, name: &str) -> Result<ComponentInstancePtr<T>, ComponentInstanceProviderError> {
//         todo!()
//     }
//
//     fn instance_by_name_option<T: Injectable + ?Sized>(&mut self, name: &str) -> Result<Option<ComponentInstancePtr<T>>, ComponentInstanceProviderError> {
//         todo!()
//     }
// }
//
// fn enrich_error<T: ?Sized>(error: ComponentInstanceProviderError) -> ComponentInstanceProviderError {
//     match error {
//         ComponentInstanceProviderError::NoPrimaryInstance { type_id, type_name: None } => ComponentInstanceProviderError::NoPrimaryInstance {
//             type_id,
//             type_name: Some(type_name::<T>().to_string()),
//         },
//         ComponentInstanceProviderError::DependencyCycle { type_id, type_name: None } => ComponentInstanceProviderError::DependencyCycle {
//             type_id,
//             type_name: Some(type_name::<T>().to_string()),
//         },
//         _ => error,
//     }
// }
//
// fn cast_instance<T: Injectable + ?Sized>(
//     instance: ComponentInstanceAnyPtr,
//     cast: CastFunction,
//     type_id: TypeId,
// ) -> Result<ComponentInstancePtr<T>, ComponentInstanceProviderError> {
//     debug_assert_eq!(type_id, TypeId::of::<T>());
//     cast(instance)
//         .map_err(|_| ComponentInstanceProviderError::IncompatibleComponent {
//             type_id,
//             type_name: type_name::<T>().to_string(),
//         })
//         .and_then(|p| {
//             p.downcast::<ComponentInstancePtr<T>>()
//                 .map(|p| (*p).clone())
//                 .map_err(|_| ComponentInstanceProviderError::IncompatibleComponent {
//                     type_id,
//                     type_name: type_name::<T>().to_string(),
//                 })
//         })
// }
