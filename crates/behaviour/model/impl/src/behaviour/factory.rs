// use dashmap::DashSet;
// use dashmap::ReadOnlyView;
// use std::ops::Deref;
// use std::sync::Arc;
//
// use reactive_graph_behaviour_model_api::prelude::*;
// use reactive_graph_reactive_service_api::prelude::*;

// pub trait BehaviourFactory<ID: Clone, T: ReactiveInstance<ID>> {
//     /// Creates a new behaviour in the given reactive instance and returns the created state machine.
//     fn create(&self, reactive_instance: T) -> Result<Arc<dyn BehaviourFsm<ID, T> + Send + Sync>, BehaviourCreationError>;
//
//     /// Returns the behaviour type of the behaviour factory.
//     fn behaviour_ty(&self) -> &BehaviourTypeId;
// }

// pub struct BehaviourFactories<ID: Clone, T: ReactiveInstance<ID>>(DashSet<Arc<dyn BehaviourFactory<ID, T>>>);
//
// impl<ID: Clone, T: ReactiveInstance<ID>> BehaviourFactories<ID, T> {
//     pub fn new() -> Self {
//         Self(DashSet::new())
//     }
//
//     pub fn factory(self, factory: Arc<dyn BehaviourFactory<ID, T>>) -> Self {
//         self.0.insert(factory);
//         self
//     }
//
//     fn push(&self, factory: Arc<dyn BehaviourFactory<ID, T>>) {
//         self.0.insert(factory);
//     }
// }
//
// impl<ID: Clone, T: ReactiveInstance<ID>> Deref for BehaviourFactories<ID, T> {
//     type Target = DashSet<Box<dyn BehaviourFactory<ID, T>>>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
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

// impl<ID: Clone, T: ReactiveInstance<ID>, F> FromIterator<BehaviourFunctionsReadOnlyView<F>> for BehaviourFactories<ID, T> {
//     fn from_iter<T: IntoIterator<Item = BehaviourFunctionsReadOnlyView<F>>>(iter: T) -> Self {
//         for x in iter {
//
//         }
//     }
// }
