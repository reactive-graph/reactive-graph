use std::ops::Deref;
use std::sync::Arc;

use async_graphql::*;

use crate::api::RelationTypeManager;
use crate::graphql::query::{GraphQLEntityInstance, GraphQLPropertyInstance, GraphQLRelationType};
use crate::model::ReactiveRelationInstance;

/// Relation instances are edges from an outbound entity instance to an
/// inbound entity instance.
///
/// The relation instance is of a relation type. The relation type defines
/// the entity types of the outbound entity instance and the inbound entity
/// instance. Furthermore the relation type defines which properties
/// (name, data type, socket type) a relation instance have to have.
///
/// In constrast to the relation type, the relation instance stores values/
/// documents in it's properties.
pub struct GraphQLRelationInstance {
    relation_instance: Arc<ReactiveRelationInstance>,
}

#[Object(name = "RelationInstance")]
impl GraphQLRelationInstance {
    /// The outbound entity instance.
    ///
    /// You can use this in order to navigate from the outbound entity instance to the inbound
    /// entity instance or vice versa.
    async fn outbound(&self) -> GraphQLEntityInstance {
        self.relation_instance.outbound.clone().into()
    }

    /// The relation type.
    #[graphql(name = "type")]
    async fn relation_type(&self, context: &Context<'_>) -> Option<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>();
        if relation_type_manager.is_ok() {
            let relation_type_manager = relation_type_manager.unwrap();
            let type_name = self.relation_instance.type_name.clone();
            // starts_with because the relation type name of the default_connector contains extra
            // information (outbound+inbound property names) in order to allow multiple connectors
            // between the two entity instances
            if let Some(relation_type) = relation_type_manager.get_starts_with(type_name.clone()) {
                let mut relation_type = relation_type;
                relation_type.full_name = type_name;
                return Some(relation_type.into());
            }
        }
        None
    }

    /// The inbound entity instance.
    ///
    /// You can use this in order to navigate from the inbound entity instance to the outbound
    /// entity instance or vice versa.
    async fn inbound(&self) -> GraphQLEntityInstance {
        self.relation_instance.inbound.clone().into()
    }

    /// Textual description of the relation instance.
    async fn description(&self) -> String {
        self.relation_instance.description.clone()
    }

    /// The properties of then relation instance.
    ///
    /// Each property is represented by it's name (String) and it's value. The value is
    /// a representation of a JSON. Therefore the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    async fn properties(
        &self,
        #[graphql(desc = "Filters by property name.")] name: Option<String>,
        #[graphql(desc = "Filters by property names")] names: Option<Vec<String>>,
    ) -> Vec<GraphQLPropertyInstance> {
        self.relation_instance
            .properties
            .iter()
            .filter(|(property_name, _property_instance)| name.is_none() || name.clone().unwrap() == property_name.deref().clone())
            .filter(|(property_name, _property_instance)| names.is_none() || names.clone().unwrap().contains(&property_name))
            .map(|(name, property_instance)| {
                let value = property_instance.value.read().unwrap().deref().clone();
                GraphQLPropertyInstance { name: name.clone(), value }
            })
            .collect()
    }
}

impl From<Arc<ReactiveRelationInstance>> for GraphQLRelationInstance {
    fn from(relation_instance: Arc<ReactiveRelationInstance>) -> Self {
        GraphQLRelationInstance { relation_instance }
    }
}
