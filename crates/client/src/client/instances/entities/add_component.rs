#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use crate::schema_graphql::scalar::UUID;
    use crate::schema_graphql::types::component::ComponentTypeId;
    use crate::schema_graphql::types::component::ComponentTypeIds;
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct AddComponentVariables {
        pub id: UUID,
        pub components: Option<Vec<ComponentTypeId>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddComponentVariables")]
    pub struct AddComponent {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddComponentVariables")]
    pub struct MutationInstances {
        pub entities: MutationEntityInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddComponentVariables")]
    pub struct MutationEntityInstances {
        #[arguments(id: $id, addComponents: $components)]
        pub update: EntityInstance,
    }

    pub fn add_component(id: Uuid, component_ty: reactive_graph_graph::ComponentTypeId) -> Operation<AddComponent, AddComponentVariables> {
        use cynic::MutationBuilder;
        let component_ty = component_ty.into();
        let vars = AddComponentVariables::builder().id(id.into()).components(Some(vec![component_ty])).build();
        AddComponent::build(vars.into())
    }

    pub fn add_components(id: Uuid, component_tys: reactive_graph_graph::ComponentTypeIds) -> Operation<AddComponent, AddComponentVariables> {
        use cynic::MutationBuilder;
        let component_tys: ComponentTypeIds = component_tys.into();
        let vars = AddComponentVariables::builder().id(id.into()).components(Some(component_tys.0)).build();
        AddComponent::build(vars.into())
    }
}
