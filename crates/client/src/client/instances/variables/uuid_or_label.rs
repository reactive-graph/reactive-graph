#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use serde::Serialize;
    use serde::Serializer;
    use uuid::Uuid;

    #[derive(Debug)]
    pub enum UuidOrLabel {
        Uuid(UUID),
        Label(String),
    }

    impl Serialize for UuidOrLabel {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Ok(S::Ok())
        }
    }

    #[derive(QueryVariables, Debug)]
    pub struct UuidOrLabelVariables {
        pub id_or_label: UuidOrLabel,
    }

    impl From<Uuid> for UuidOrLabelVariables {
        fn from(id: Uuid) -> Self {
            Self {
                id_or_label: UuidOrLabel::Uuid(UUID(id)),
            }
        }
    }

    impl From<String> for UuidOrLabelVariables {
        fn from(label: String) -> Self {
            Self {
                id_or_label: UuidOrLabel::Label(label),
            }
        }
    }
}
