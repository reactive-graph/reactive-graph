#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::relations::variables::id_and_components::variables::RelationInstanceIdAndComponentsVariables;
    use crate::client::instances::relations::variables::id_and_components::variables::RelationInstanceIdAndComponentsVariablesFields;
    use crate::schema_graphql::instances::relation_instance::RelationInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::ComponentTypeId;
    use reactive_graph_graph::ComponentTypeIds;
    use reactive_graph_graph::RelationInstanceId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "RelationInstanceIdAndComponentsVariables")]
    pub struct RemoveComponent {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RelationInstanceIdAndComponentsVariables")]
    pub struct MutationInstances {
        pub relations: MutationRelationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RelationInstanceIdAndComponentsVariables")]
    pub struct MutationRelationInstances {
        #[arguments(relationInstanceId: { outboundId: $outbound_id, namespace: $namespace, typeName: $name, instanceId: $instance_id, inboundId: $inbound_id}, removeComponents: $components)]
        pub update: RelationInstance,
    }

    pub fn remove_component<TY: Into<ComponentTypeId>>(
        id: &RelationInstanceId,
        component_ty: TY,
    ) -> Operation<RemoveComponent, RelationInstanceIdAndComponentsVariables> {
        use cynic::MutationBuilder;
        RemoveComponent::build(RelationInstanceIdAndComponentsVariables::new_from_component_type(id, component_ty))
    }

    pub fn remove_components<TY: Into<ComponentTypeIds>>(
        id: &RelationInstanceId,
        component_tys: TY,
    ) -> Operation<RemoveComponent, RelationInstanceIdAndComponentsVariables> {
        use cynic::MutationBuilder;
        RemoveComponent::build(RelationInstanceIdAndComponentsVariables::new(id, component_tys))
    }
}
