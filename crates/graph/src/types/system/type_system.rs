use crate::Components;
use crate::EntityTypes;
use crate::FlowTypes;
use crate::NamespacedTypeComponentPropertiesContainer;
use crate::RelationTypes;
use crate::TypeSystemMergeComponentPropertiesError;
use typed_builder::TypedBuilder;

#[cfg(any(test, feature = "test"))]
use crate::Component;
#[cfg(any(test, feature = "test"))]
use crate::EntityInstance;
#[cfg(any(test, feature = "test"))]
use crate::EntityInstances;
#[cfg(any(test, feature = "test"))]
use crate::EntityType;
#[cfg(any(test, feature = "test"))]
use crate::Extensions;
#[cfg(any(test, feature = "test"))]
use crate::FlowType;
#[cfg(any(test, feature = "test"))]
use crate::NamespacedType;
#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeContainer;
#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::Namespaces;
#[cfg(any(test, feature = "test"))]
use crate::PropertyTypes;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildType;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypes;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypes;
#[cfg(any(test, feature = "test"))]
use crate::RelationInstances;
#[cfg(any(test, feature = "test"))]
use crate::RelationType;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::DefaultTryFrom;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;
#[cfg(any(test, feature = "test"))]
use std::ops::Range;

#[derive(Clone, Default, TypedBuilder)]
pub struct TypeSystem {
    #[builder(default, setter(into))]
    components: Components,
    #[builder(default, setter(into))]
    entity_types: EntityTypes,
    #[builder(default, setter(into))]
    relation_types: RelationTypes,
    #[builder(default, setter(into))]
    flow_types: FlowTypes,
}

impl TypeSystem {
    pub fn components(&self) -> &Components {
        &self.components
    }

    pub fn entity_types(&self) -> &EntityTypes {
        &self.entity_types
    }

    pub fn relation_types(&self) -> &RelationTypes {
        &self.relation_types
    }

    pub fn flow_types(&self) -> &FlowTypes {
        &self.flow_types
    }

    pub fn merge_own_component_properties(&self) -> Result<(), TypeSystemMergeComponentPropertiesError> {
        self.entity_types.merge_component_properties(self.components.clone())?;
        self.relation_types.merge_component_properties(self.components.clone())?;
        Ok(())
    }

    pub fn merge_component_properties<C: Into<Components>>(&self, components: C) -> Result<(), TypeSystemMergeComponentPropertiesError> {
        let components = components.into();
        self.entity_types.merge_component_properties(components.clone())?;
        self.relation_types.merge_component_properties(components)?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.components.len() + self.entity_types.len() + self.relation_types.len() + self.flow_types.len()
    }
}

// impl AsRef<Components> for TypeSystem {
//     fn as_ref(&self) -> &Components {
//         &self.components
//     }
// }
//
// impl AsRef<EntityTypes> for TypeSystem {
//     fn as_ref(&self) -> &EntityTypes {
//         &self.entity_types
//     }
// }
//
// impl AsRef<RelationTypes> for TypeSystem {
//     fn as_ref(&self) -> &RelationTypes {
//         &self.relation_types
//     }
// }
//
// impl AsRef<FlowTypes> for TypeSystem {
//     fn as_ref(&self) -> &FlowTypes {
//         &self.flow_types
//     }
// }

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedTypes for TypeSystem {
    type Error = NamespacedTypeError;

    fn random_types(range: Range<usize>) -> Result<Self, Self::Error> {
        Ok(TypeSystem {
            components: Components::random_types(range.clone())?,
            entity_types: EntityTypes::random_types(range.clone())?,
            relation_types: RelationTypes::random_types(range.clone())?,
            flow_types: FlowTypes::random_types(range)?,
        })
    }
}

#[cfg(any(test, feature = "test"))]
impl TypeSystem {
    pub fn random_tree(
        range_components: Range<usize>,
        range_entity_types: Range<usize>,
        range_relation_types: Range<usize>,
        range_flow_types: Range<usize>,
    ) -> Result<Self, NamespacedTypeError> {
        let mut rng = rand::rng();
        let namespaces = Namespaces::random_path_tree()?;
        println!("#namespaces: {}", namespaces.len());
        println!("namespaces: {:?}", namespaces);
        let components = Components::new();
        for namespace in namespaces.iter() {
            components.push(Component::random_child_type(namespace)?);
            components.push_all(Components::random_child_types(namespace, range_components.clone())?);
        }
        let entity_types = EntityTypes::new();
        for namespace in namespaces.iter() {
            for _ in 0..rng.random_range(range_entity_types.clone()) {
                entity_types.push(
                    EntityType::builder()
                        .ty(NamespacedType::random_child_type_id(namespace)?)
                        .description(r_string())
                        .components(components.pick_random_types(2..3).type_ids())
                        .properties(PropertyTypes::random_types(0..5)?)
                        .extensions(Extensions::random_types(0..5)?)
                        .build(),
                );
            }
        }
        let relation_types = RelationTypes::new();
        for namespace in namespaces.iter() {
            for _ in 0..rng.random_range(range_relation_types.clone()) {
                if let (Some(outbound_type), Some(inbound_type)) = (entity_types.pick_random_type(), entity_types.pick_random_type()) {
                    relation_types.push(
                        RelationType::builder()
                            .outbound_type(outbound_type.ty)
                            .ty(NamespacedType::random_child_type_id(namespace)?)
                            .inbound_type(inbound_type.ty)
                            .description(r_string())
                            .components(components.pick_random_types(2..3).type_ids())
                            .properties(PropertyTypes::random_types(0..5)?)
                            .extensions(Extensions::random_types(0..5)?)
                            .build(),
                    );
                }
            }
        }
        let flow_types = FlowTypes::new();
        for namespace in namespaces.iter() {
            for _ in 0..rng.random_range(range_flow_types.clone()) {
                if let Some(wrapper_entity_type) = entity_types.pick_random_type() {
                    if let Ok(wrapper_entity_instance) = EntityInstance::default_try_from(&wrapper_entity_type) {
                        let entity_instances = EntityInstances::new();
                        // for _ in 0..rng.random_range(0..3) {
                        //     if let Some(entity_type) = entity_types.pick_random_type() {
                        //         if let Ok(entity_instance) = EntityInstance::default_try_from(&entity_type) {
                        //             entity_instances.push(entity_instance);
                        //         }
                        //     }
                        // }
                        let relation_instances = RelationInstances::new();
                        // for _ in 0..rng.random_range(0..3) {
                        //     if let Some(relation_type) = relation_types.pick_random_type() {
                        //         if let InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(entity_type)) = relation_type.outbound_type {
                        //             entity_types.get_by_namespace()
                        //             if let Ok(entity_instance) = EntityInstance::default_try_from(&entity_type) {
                        //                 entity_instances.push(entity_instance);
                        //             }
                        //         }
                        //         let outbound_id = match relation_type.outbound_type {
                        //             InboundOutboundType::Component(_) => {}
                        //             InboundOutboundType::EntityType(entity_type) => {
                        //                 if let Ok(entity_instance) = EntityInstance::default_try_from(&entity_type) {
                        //                     entity_instances.push(entity_instance);
                        //                 }
                        //             }
                        //         }
                        //         let relation_instance = RelationInstance::builder()
                        //             .outbound_id()
                        //             .build();
                        //         relation_instances.push(relation_instance);
                        //         if let Ok(relation_instance) = RelationInstance::default_try_from(&relation_type) {
                        //             relation_instances.push(relation_instance);
                        //         }
                        //     }
                        // }
                        flow_types.push(
                            FlowType::builder()
                                .wrapper_entity_instance(wrapper_entity_instance)
                                .ty(NamespacedType::random_child_type_id(namespace)?)
                                .description(r_string())
                                .entity_instances(entity_instances)
                                .relation_instances(relation_instances)
                                .extensions(Extensions::random_types(0..10)?)
                                .build(),
                        );
                    }
                }
            }
        }

        Ok(TypeSystem {
            components,
            entity_types,
            relation_types,
            flow_types,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::RandomNamespacedTypes;
    use crate::types::TypeSystem;

    #[test]
    fn test_typesystem_empty() {
        let type_system = TypeSystem::default();
        assert_eq!(0, type_system.components.len());
        assert_eq!(0, type_system.entity_types.len());
        assert_eq!(0, type_system.relation_types.len());
        assert_eq!(0, type_system.flow_types.len());
    }

    #[test]
    fn test_typesystem_random_types() {
        let type_system = TypeSystem::random_types(0..10).expect("Failed to create a type system with random types");
        println!("{}", type_system.components.len());
        println!("{}", type_system.entity_types.len());
        println!("{}", type_system.relation_types.len());
        println!("{}", type_system.flow_types.len());
    }

    #[test]
    fn test_typesystem_random_tree() {
        let type_system = TypeSystem::random_tree(2..3, 1..5, 1..3, 1..2).expect("Failed to create a random type system tree");
        println!("{}", type_system.components.len());
        println!("{}", type_system.entity_types.len());
        println!("{}", type_system.relation_types.len());
        println!("{}", type_system.flow_types.len());
    }
}
