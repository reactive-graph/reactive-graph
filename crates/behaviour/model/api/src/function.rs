use std::ops::Deref;
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::ReadOnlyView;

use inexor_rgf_reactive_model_api::ReactiveInstance;

use crate::BehaviourFactories;
use crate::BehaviourFactory;
use crate::BehaviourTypeId;
use crate::ComponentBehaviourTypeId;
use crate::ComponentBehaviourTypeIds;
use crate::EntityBehaviourTypeId;
use crate::EntityBehaviourTypeIds;
use crate::RelationBehaviourTypeId;
use crate::RelationBehaviourTypeIds;

// pub type BehaviourFunction = fn(Value) -> Result<(), BehaviourFunctionError>;

pub type BehaviourFactoryCreator<ID, T, FnType> = fn(&BehaviourTypeId, FnType) -> Arc<dyn BehaviourFactory<ID, T> + Send + Sync + 'static>;

pub struct BehaviourFunctions<ID: Clone, T: ReactiveInstance<ID>, FnType: Clone>(DashMap<BehaviourTypeId, FnType>, BehaviourFactoryCreator<ID, T, FnType>);

impl<ID: Clone, T: ReactiveInstance<ID>, FnType: Clone> BehaviourFunctions<ID, T, FnType> {
    pub fn new(factory_creator: BehaviourFactoryCreator<ID, T, FnType>) -> Self {
        Self(DashMap::new(), factory_creator)
    }

    pub fn with_namespace<N: Into<String>>(
        namespace: N,
        factory_creator: BehaviourFactoryCreator<ID, T, FnType>,
    ) -> NamespacedBehaviourFunctions<ID, T, FnType> {
        NamespacedBehaviourFunctions::new(namespace, factory_creator)
    }

    pub fn behaviour_from_ty<B: Into<BehaviourTypeId>>(self, ty: B, f: FnType) -> Self {
        self.0.insert(ty.into(), f);
        self
    }
    pub fn behaviour<N: Into<String>, TN: Into<String>>(self, namespace: N, type_name: TN, f: FnType) -> Self {
        self.0.insert(BehaviourTypeId::new_from_type(namespace, type_name), f);
        self
    }

    pub fn get(&self) -> BehaviourFunctionsReadOnlyView<ID, T, FnType> {
        self.into()
    }
}

pub struct BehaviourFunctionsReadOnlyView<ID: Clone, T: ReactiveInstance<ID>, FnType: Clone>(
    ReadOnlyView<BehaviourTypeId, FnType>,
    BehaviourFactoryCreator<ID, T, FnType>,
);

impl<ID: Clone, T: ReactiveInstance<ID>, FnType: Clone> BehaviourFunctionsReadOnlyView<ID, T, FnType> {
    pub fn to_component_behaviour_tys(&self) -> ComponentBehaviourTypeIds {
        self.keys().map(ComponentBehaviourTypeId::from).collect()
    }

    pub fn to_entity_behaviour_tys(&self) -> EntityBehaviourTypeIds {
        self.keys().map(EntityBehaviourTypeId::from).collect()
    }

    pub fn to_relation_behaviour_tys(&self) -> RelationBehaviourTypeIds {
        self.keys().map(RelationBehaviourTypeId::from).collect()
    }

    pub fn get_factories(&self) -> BehaviourFactories<ID, T> {
        self.0
            .iter()
            .fold(BehaviourFactories::new(), |factories, (ty, f)| factories.factory(self.1(ty, f.clone())))
    }
}

impl<'a, ID: Clone, T: ReactiveInstance<ID>, FnType: Clone> Deref for BehaviourFunctionsReadOnlyView<ID, T, FnType> {
    type Target = ReadOnlyView<BehaviourTypeId, FnType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<ID: Clone, T: ReactiveInstance<ID>, FnType: Clone> From<&BehaviourFunctions<ID, T, FnType>> for BehaviourFunctionsReadOnlyView<ID, T, FnType> {
    fn from(bf: &BehaviourFunctions<ID, T, FnType>) -> Self {
        Self(bf.0.clone().into_read_only(), bf.1)
    }
}

pub struct NamespacedBehaviourFunctions<ID: Clone, T: ReactiveInstance<ID>, FnType: Clone>(String, BehaviourFunctions<ID, T, FnType>);

impl<ID: Clone, T: ReactiveInstance<ID>, FnType: Clone> NamespacedBehaviourFunctions<ID, T, FnType> {
    pub fn new<N: Into<String>>(namespace: N, factory_creator: BehaviourFactoryCreator<ID, T, FnType>) -> Self {
        Self(namespace.into(), BehaviourFunctions::new(factory_creator))
    }

    pub fn behaviour<TN: Into<String>>(self, type_name: TN, f: FnType) -> Self {
        self.1 .0.insert(BehaviourTypeId::new_from_type(self.0.clone(), type_name), f);
        self
    }

    pub fn get(&self) -> BehaviourFunctionsReadOnlyView<ID, T, FnType> {
        self.1.get()
    }
}

impl<ID: Clone, T: ReactiveInstance<ID>, FnType: Clone> From<NamespacedBehaviourFunctions<ID, T, FnType>> for BehaviourFunctions<ID, T, FnType> {
    fn from(f: NamespacedBehaviourFunctions<ID, T, FnType>) -> Self {
        f.1
    }
}

#[macro_export]
macro_rules! entity_behaviour_functions {
    ($ident: ident, $fn_type: ty, $constructor: expr) => {
        pub static $ident: std::sync::LazyLock<$crate::BehaviourFunctionsReadOnlyView<Uuid, inexor_rgf_reactive_model_impl::ReactiveEntity, $fn_type>> =
            std::sync::LazyLock::new($constructor);
    };
}

#[macro_export]
macro_rules! relation_behaviour_functions {
    ($ident: ident, $fn_type: ty, $constructor: expr) => {
        pub static $ident: std::sync::LazyLock<
            $crate::BehaviourFunctionsReadOnlyView<inexor_rgf_graph::RelationTypeId, inexor_rgf_reactive_model_impl::ReactiveRelation, $fn_type>,
        > = std::sync::LazyLock::new($constructor);
    };
}
