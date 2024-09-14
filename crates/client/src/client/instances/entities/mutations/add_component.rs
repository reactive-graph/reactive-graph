#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::variables::id_and_component::queries::IdAndComponentVariables;
    use crate::client::instances::variables::id_and_component::queries::IdAndComponentVariablesFields;
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use crate::schema_graphql::types::component::ComponentTypeIds;
    use cynic::Operation;
    use cynic::QueryFragment;
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

    pub fn add_component(id: Uuid, component_ty: reactive_graph_graph::ComponentTypeId) -> Operation<AddComponent, IdAndComponentVariables> {
        use cynic::MutationBuilder;
        let component_ty = component_ty.into();
        let vars = IdAndComponentVariables::builder().id(id.into()).components(Some(vec![component_ty])).build();
        AddComponent::build(vars.into())
    }

    pub fn add_components(id: Uuid, component_tys: reactive_graph_graph::ComponentTypeIds) -> Operation<AddComponent, IdAndComponentVariables> {
        use cynic::MutationBuilder;
        let component_tys: ComponentTypeIds = component_tys.into();
        let vars = IdAndComponentVariables::builder().id(id.into()).components(Some(component_tys.0)).build();
        AddComponent::build(vars.into())
    }
}
