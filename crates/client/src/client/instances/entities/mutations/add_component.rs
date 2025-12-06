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
    pub struct AddComponent {
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
        #[arguments(id: $id, addComponents: $components)]
        pub update: EntityInstance,
    }

    pub fn add_component<C: Into<ComponentTypeId>>(id: Uuid, component_ty: C) -> Operation<AddComponent, IdAndComponentVariables> {
        add_components(id, ComponentTypeIds::new().component(component_ty.into()))
    }

    pub fn add_components(id: Uuid, component_tys: ComponentTypeIds) -> Operation<AddComponent, IdAndComponentVariables> {
        use cynic::MutationBuilder;
        AddComponent::build(IdAndComponentVariables::new(id, Some(component_tys)))
    }
}
