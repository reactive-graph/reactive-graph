use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;

use crate::mutation::GraphQLExtensionDefinition;
use crate::mutation::GraphQLRelationInstanceId;
use crate::query::GraphQLExtension;
use crate::query::GraphQLPropertyInstance;
use reactive_graph_graph::Extension;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_graph::RelationInstances;

/// Relation instances are edges from an outbound entity instance to an
/// inbound entity instance.
///
/// The relation instance is of a relation type. The relation type defines
/// the entity types of the outbound entity instance and the inbound entity
/// instance. Furthermore, the relation type defines which properties
/// (name, data type, socket type) a relation instance have to have.
///
/// In contrast to the relation type, the relation instance stores values/
/// documents in its properties.
#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "RelationInstanceDefinition")]
pub struct GraphQLRelationInstanceDefinition {
    /// The fully qualified namespace, outbound id, inbound id and instance id of the relation instance
    pub relation_instance_id: GraphQLRelationInstanceId,

    /// The name of the relation instance.
    pub name: String,

    /// Textual description of the relation instance.
    pub description: String,

    /// The properties of then relation instance.
    ///
    /// Each property is represented by its name (String) and it's value. The value is
    /// a representation of a JSON. Therefore, the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    pub properties: Vec<GraphQLPropertyInstance>,

    /// Relation instance specific extensions.
    pub extensions: Vec<GraphQLExtensionDefinition>,
}

// impl From<GraphQLRelationInstanceDefinition> for RelationInstance {
//     fn from(relation_instance: GraphQLRelationInstanceDefinition) -> Self {
//         let ty = RelationInstanceTypeId::new_from_type_unique_for_instance_id(
//             relation_instance.namespace,
//             relation_instance.type_name,
//             relation_instance.instance_id,
//         );
//         let properties: PropertyInstances = relation_instance
//             .properties
//             .iter()
//             .map(|property_instance| (property_instance.name.clone(), property_instance.value.clone()))
//             .collect();
//         // let components; relation_instance.components.iter().map(|e| ComponentTypeId::from(e.clone())).collect();
//         let extensions: Extensions = relation_instance.extensions.iter().map(|e| Extension::from(e.clone())).collect();
//         RelationInstance::builder()
//             .outbound_id(relation_instance.outbound_id)
//             .ty(ty)
//             .inbound_id(relation_instance.inbound_id)
//             .description(relation_instance.description)
//             .properties(properties)
//             // .components(components) ???
//             .extensions(extensions)
//             .build()
//     }
// }

#[derive(Default)]
pub struct GraphQLRelationInstanceDefinitions(pub Vec<GraphQLRelationInstanceDefinition>);

impl GraphQLRelationInstanceDefinitions {
    pub fn new(relation_instances: Vec<GraphQLRelationInstanceDefinition>) -> Self {
        Self(relation_instances)
    }
}

impl From<GraphQLRelationInstanceDefinitions> for RelationInstances {
    fn from(relation_instances: GraphQLRelationInstanceDefinitions) -> Self {
        relation_instances.0.into_iter().map(|entity_instance| entity_instance.into()).collect()
    }
}
