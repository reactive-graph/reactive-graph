use std::any::Any;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use log::error;
use springtime_di::instance_provider::CastFunction;
use springtime_di::instance_provider::ComponentInstanceAnyPtr;
use springtime_di::instance_provider::ComponentInstanceProvider;
use springtime_di::instance_provider::ComponentInstanceProviderError;
use springtime_di::instance_provider::TypedComponentInstanceProvider;

use inexor_rgf_di::get_shared_component_factory;
use inexor_rgf_runtime_api::Runtime;

use crate::RuntimeImpl;

pub fn get_runtime() -> Arc<dyn Runtime + Send + Sync> {
    let mut component_factory = get_shared_component_factory();
    // match TypedComponentInstanceProvider::primary_instance_typed::<dyn Runtime + Send + Sync>(&mut component_factory) {
    match TypedComponentInstanceProvider::primary_instance_typed::<RuntimeImpl>(&mut component_factory) {
        Ok(runtime) => runtime,
        // Err(e) => {
        //     panic!("{}", e);
        // }
        Err(ComponentInstanceProviderError::NoPrimaryInstance { type_id, type_name }) => {
            error!("Missing component {type_name:?}");
            let instances: Result<Vec<(ComponentInstanceAnyPtr, CastFunction)>, ComponentInstanceProviderError> = component_factory.instances(type_id);
            match instances {
                Ok(instances) => {
                    for (component_instance, _) in instances {
                        error!("Type Id: {:?}", component_instance.type_id());
                    }
                }
                Err(e) => {
                    error!("{e:?}");
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
            panic!("Error in component constructor: {}", constructor_error);
        }
    }

    // Old 3
    // match ComponentFactoryBuilder::new() {
    //     Ok(component_factory) => {
    //         // Old 1
    //         // let mut component_factory = component_factory.build();
    //         // Try 2
    //         let Some(definition_registry) = get_type_system_cdr() else {
    //             panic!("No definition registry!");
    //         };
    //         let mut component_factory = component_factory.with_definition_registry(definition_registry).build();
    //         // TEST
    //
    //         match TypedComponentInstanceProvider::primary_instance_typed::<dyn Runtime + Send + Sync>(&mut component_factory) {
    //             Ok(runtime) => runtime,
    //             Err(ComponentInstanceProviderError::NoPrimaryInstance { type_id, type_name }) => {
    //                 error!("Missing component {type_name:?}");
    //                 let instances: Result<Vec<(ComponentInstanceAnyPtr, CastFunction)>, ComponentInstanceProviderError> = component_factory.instances(type_id);
    //                 match instances {
    //                     Ok(instances) => {
    //                         for (component_instance, _) in instances {
    //                             error!("Type Id: {:?}", component_instance.type_id());
    //                         }
    //                     }
    //                     Err(e) => {
    //                         error!("{e:?}");
    //                     }
    //                 }
    //                 panic!(
    //                     "Cannot find a primary instance for component '{type_id:?}/{type_name:?}' - either none or multiple exists without a primary marker."
    //                 );
    //             }
    //             Err(ComponentInstanceProviderError::IncompatibleComponent { type_id, type_name }) => {
    //                 panic!("Tried to downcast component to incompatible type: {type_id:?}/{type_name}");
    //             }
    //             Err(ComponentInstanceProviderError::NoNamedInstance(type_name)) => {
    //                 panic!("Cannot find named component: {type_name}");
    //             }
    //             Err(ComponentInstanceProviderError::UnrecognizedScope(scope)) => {
    //                 panic!("Unrecognized scope: {scope}");
    //             }
    //             Err(ComponentInstanceProviderError::DependencyCycle { type_id, type_name }) => {
    //                 panic!("Detected dependency cycle for: {type_id:?}/{type_name:?}");
    //             }
    //             Err(ComponentInstanceProviderError::ConstructorError(constructor_error)) => {
    //                 panic!("Error in component constructor: {}", constructor_error);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         panic!("{}", e);
    //     }
    // }
}

pub async fn main<F1, F2, C1, C2>(pre_config: C1, post_config: C2)
where
    F1: Future<Output = ()>,
    F2: Future<Output = ()>,
    C1: FnOnce(Arc<dyn Runtime>) -> F1,
    C2: FnOnce(Arc<dyn Runtime>) -> F2,
{
    {
        let runtime = get_runtime();
        // Runtime Configuration Phase
        pre_config(runtime.clone()).await;
        runtime.config().await;
        post_config(runtime.clone()).await;
        // Runtime Lifecycle
        runtime.init().await;
        runtime.post_init().await;
        runtime.run().await;
        runtime.pre_shutdown().await;
        runtime.shutdown().await;
    } // Destruct the whole runtime
    tokio::time::sleep(Duration::from_millis(2000)).await;
}
