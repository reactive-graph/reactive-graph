use schemars::JsonSchema;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::EntityInstance;
use crate::EntityInstances;
use crate::EntityTypeId;
use crate::NamespacedTypeGetter;
use crate::RelationInstances;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;

#[derive(Debug)]
pub struct FlowInstanceCreationError;

/// A flow instance is a container for entity instances and relation instances.
///
/// A flow instance is strictly associated with a wrapper entity instance. The properties
/// of the wrapper entity instance are the properties of the flow.
///
/// Additionally, flows can be nested -  from the perspective of the outer flow
/// the inner flow acts like an entity instance. The wrapper entity instance of
/// the inner flow is the interface which can be accessed by the outer flow.
///
/// Entity instances and relation instances can be shared with multiple flows.
///
/// It's even possible to connect entity instances from different flows with relation
/// instances.
///
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[schemars(deny_unknown_fields)]
pub struct FlowInstance {
    /// The id of the flow corresponds to the id of the wrapper entity instance
    ///
    /// This means the vector of entity instances must contain an instance with
    /// the id of the flow.
    pub id: Uuid,

    /// The type definition of the entity type of the wrapper entity instance.
    #[serde(flatten)]
    #[builder(setter(into))]
    pub ty: EntityTypeId,

    /// TODO: Rename: flow_instance_name
    /// The name of the flow instance.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub name: String,

    /// Textual description of the flow instance.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub description: String,

    /// The entity instances which are contained in this flow instance.
    ///
    /// It can't have a default because the wrapper entity instance must be
    /// present in the list of entities.
    #[serde(default = "EntityInstances::new", alias = "entities")]
    #[builder(default, setter(into))]
    pub entity_instances: EntityInstances,

    /// The relation instances which are contained in this flow instance.
    ///
    /// By default, no relation instances are contained in this flow instance.
    #[serde(default = "RelationInstances::new", alias = "relations")]
    #[builder(default, setter(into))]
    pub relation_instances: RelationInstances,
}

impl FlowInstance {
    /// Constructs a new flow instance from the wrapper entity instance.
    pub fn from_instance_with_name<S: Into<String>>(wrapper_entity_instance: EntityInstance, name: S) -> FlowInstance {
        FlowInstance {
            id: wrapper_entity_instance.id,
            ty: wrapper_entity_instance.ty.clone(),
            name: name.into(),
            description: String::new(),
            entity_instances: EntityInstances::new_with_instance(wrapper_entity_instance),
            relation_instances: RelationInstances::new(),
        }
    }
}

impl From<EntityInstance> for FlowInstance {
    fn from(wrapper_entity_instance: EntityInstance) -> FlowInstance {
        FlowInstance {
            id: wrapper_entity_instance.id,
            ty: wrapper_entity_instance.ty.clone(),
            name: String::new(),
            description: String::new(),
            entity_instances: EntityInstances::new_with_instance(wrapper_entity_instance),
            relation_instances: RelationInstances::new(),
        }
    }
}

impl NamespacedTypeGetter for FlowInstance {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for FlowInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}
