#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use crate::schema_graphql::instances::property_instance::PropertyInstanceDefinition;
    use crate::schema_graphql::scalar::UUID;
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use serde_json::Value;
    use typed_builder::TypedBuilder;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct SetPropertyVariables {
        pub id: UUID,
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "SetPropertyVariables")]
    pub struct SetProperty {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "SetPropertyVariables")]
    pub struct MutationInstances {
        pub entities: MutationEntityInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "SetPropertyVariables")]
    pub struct MutationEntityInstances {
        #[arguments(id: $id, properties: $properties)]
        pub update: EntityInstance,
    }

    pub fn set_property(id: Uuid, name: String, value: Value) -> Operation<SetProperty, SetPropertyVariables> {
        use cynic::MutationBuilder;

        let vars = SetPropertyVariables::builder()
            .id(id.into())
            .properties(Some(vec![PropertyInstanceDefinition { name, value }]))
            .build();

        SetProperty::build(vars.into())
    }
}
