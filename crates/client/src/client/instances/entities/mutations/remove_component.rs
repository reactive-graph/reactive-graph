#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::variables::id_and_component::variables::IdAndComponentVariables;
    use crate::client::instances::variables::id_and_component::variables::IdAndComponentVariablesFields;
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::ComponentTypeId;
    use reactive_graph_graph::ComponentTypeIds;
    use uuid::Uuid;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "IdAndComponentVariables")]
    pub struct RemoveComponent {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "IdAndComponentVariables")]
    pub struct MutationInstances {
        pub entities: MutationEntityInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "IdAndComponentVariables")]
    pub struct MutationEntityInstances {
        #[arguments(id: $id, removeComponents: $components)]
        pub update: EntityInstance,
    }

    pub fn remove_component(id: Uuid, component_ty: ComponentTypeId) -> Operation<RemoveComponent, IdAndComponentVariables> {
        use cynic::MutationBuilder;
        RemoveComponent::build(IdAndComponentVariables::new(id, Some(vec![component_ty])))
    }

    pub fn remove_components(id: Uuid, component_tys: ComponentTypeIds) -> Operation<RemoveComponent, IdAndComponentVariables> {
        use cynic::MutationBuilder;
        RemoveComponent::build(IdAndComponentVariables::new(id, Some(component_tys)))
    }
}
