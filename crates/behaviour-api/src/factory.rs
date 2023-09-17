use std::ops::Deref;
use std::sync::Arc;

use dashmap::DashMap;

use inexor_rgf_reactive_api::prelude::*;

use crate::BehaviourCreationError;
use crate::BehaviourFsm;
use crate::BehaviourTypeId;

pub trait BehaviourFactory<ID: Clone, T: ReactiveInstance<ID>> {
    /// Creates a new behaviour in the given reactive instance and returns the created state machine.
    fn create(&self, reactive_instance: T) -> Result<Arc<dyn BehaviourFsm<ID, T> + Send + Sync>, BehaviourCreationError>;

    /// Returns the behaviour type of the behaviour factory.
    fn behaviour_ty(&self) -> &BehaviourTypeId;
}

pub struct BehaviourFactories<ID: Clone, T: ReactiveInstance<ID>>(DashMap<BehaviourTypeId, Arc<dyn BehaviourFactory<ID, T> + Send + Sync>>);

impl<ID: Clone, T: ReactiveInstance<ID>> BehaviourFactories<ID, T> {
    pub fn new() -> Self {
        Self(DashMap::new())
    }

    pub fn factory(self, factory: Arc<dyn BehaviourFactory<ID, T> + Send + Sync>) -> Self {
        self.0.insert(factory.behaviour_ty().clone(), factory);
        self
    }

    pub fn push(&self, factory: Arc<dyn BehaviourFactory<ID, T> + Send + Sync>) {
        self.0.insert(factory.behaviour_ty().clone(), factory);
    }
}

impl<ID: Clone, T: ReactiveInstance<ID>> Deref for BehaviourFactories<ID, T> {
    type Target = DashMap<BehaviourTypeId, Arc<dyn BehaviourFactory<ID, T> + Send + Sync>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<ID: Clone, T: ReactiveInstance<ID>> Clone for BehaviourFactories<ID, T> {
    fn clone(&self) -> Self {
        let factories = self.0.iter().map(|factory| (factory.key().clone(), factory.value().clone())).collect();
        Self(factories)
    }
}

// impl<ID: Clone, T: ReactiveInstance<ID>> IntoIterator for BehaviourFactories<ID, T> {
//     type Item = (BehaviourTypeId, Arc<dyn BehaviourFactory<ID, T> + Send + Sync>);
//     type IntoIter = OwningIter<BehaviourTypeId, Arc<dyn BehaviourFactory<ID, T> + Send + Sync>>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter().map(|factory| (factory.key().clone(), factory.value().clone())).collect();
//     }
// }

// impl<ID: Clone, T: ReactiveInstance<ID>> FromIterator<BehaviourFunctionsReadOnlyView<ID, T, FnType>> for BehaviourFactories<ID, T> {
//     fn from_iter<T: IntoIterator<Item = (BehaviourTypeId, FnType)>>(iter: T) -> Self {
//         let factories = BehaviourFactories::new();
//         factories
//     }
// }
//
// impl<ID: Clone, T: ReactiveInstance<ID>, F> From<BehaviourFunctionsReadOnlyView<F>> for BehaviourFactories<ID, T> {
//     fn from(fns: BehaviourFunctionsReadOnlyView<F>) -> Self {
//         let mut factories = BehaviourFactories::new();
//         // let x = F::new();
//         fns.iter().map(|(ty, f)| {
//             let a = Arc::new();
//             factories.push()
//             let x = Arc::new(T::new(behaviour_ty.clone(), *f));
//             BehaviourFactory<ID, T>
//         }).collect()
//         for f in fns.iter() {
//             factories.
//         }
//         factories
//     }
// }

#[macro_export]
macro_rules! behaviour_factory {
    (
        $factory: ident,
        $behaviour: ty,
        $id: ty,
        $reactive_instance: ty
        $(, $fn_name:ident, $fn_ident: ident)*
    ) => {
        use $crate::BehaviourTypesContainer as ModelBehaviourTypesContainer;

        pub struct $factory {
            pub ty: $crate::BehaviourTypeId,
            $(pub $fn_name: $fn_ident,)*
        }

        impl $factory {
            pub fn new(ty: $crate::BehaviourTypeId, $($fn_name: $fn_ident)*) -> Self {
                $factory {
                    ty,
                    $($fn_name,)*
                }
            }
        }

        impl $crate::BehaviourFactory<$id, $reactive_instance> for $factory {
            fn create(
                &self,
                reactive_instance: $reactive_instance,
            ) -> Result<std::sync::Arc<dyn $crate::BehaviourFsm<$id, $reactive_instance> + Send + Sync>, $crate::BehaviourCreationError> {
                // Prevent that the same behaviour can be applied twice / multiple times.
                if reactive_instance.behaves_as(&self.ty) {
                    return Err($crate::BehaviourCreationError::BehaviourAlreadyApplied(self.ty.clone()));
                }
                match <$behaviour>::new(reactive_instance, self.ty.clone() $(, self.$fn_name)*) {
                    Ok(state) => {
                        let state = state as std::sync::Arc<dyn $crate::BehaviourFsm<$id, $reactive_instance> + Send + Sync>;
                        Ok(state)
                    }
                    Err(e) => Err(e),
                }
            }

            fn behaviour_ty(&self) -> &$crate::BehaviourTypeId {
                &self.ty
            }
        }
    };
}
