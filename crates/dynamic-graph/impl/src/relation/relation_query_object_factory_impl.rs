use crate::extension::field_description::get_dynamic_graph_field_descriptions;
use crate::extension::field_name::get_dynamic_graph_field_names;
use crate::field::namespace_path_field;
use crate::field::property::property_container_property_fields;
use crate::field::relation::relation_id_field;
use crate::field::relation::relation_instance_id_field;
use crate::interface_manager_impl::component_type_id_container_component_fields;
use crate::object_type_name::object_type_name;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use async_trait::async_trait;
use log::trace;
use reactive_graph_dynamic_graph_api::FIELD_NAME_JSON_SCHEMA;
use reactive_graph_dynamic_graph_api::INTERFACE_RELATION;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RelationQueryObjectFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_dynamic_graph_api::UNION_ALL_ENTITIES;
use reactive_graph_graph::InboundOutboundDirection;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::MatchingInboundOutboundType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::RelationTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct RelationQueryObjectFactoryImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

impl RelationQueryObjectFactoryImpl {
    fn inbound_outbound_fields(&self, relation_type: &RelationType, mut object: Object) -> Object {
        // Outbound fields
        for field in self.inbound_outbound_field(&relation_type, InboundOutboundDirection::Outbound, &relation_type.outbound_type) {
            object = object.field(field);
        }
        // Inbound fields
        for field in self.inbound_outbound_field(&relation_type, InboundOutboundDirection::Inbound, &relation_type.inbound_type) {
            object = object.field(field);
        }
        object
    }

    fn inbound_outbound_field(&self, relation_type: &RelationType, dir: InboundOutboundDirection, ty: &InboundOutboundType) -> Vec<Field> {
        // Look up field names and descriptions in extensions
        let field_names = get_dynamic_graph_field_names(relation_type);
        let field_descriptions = get_dynamic_graph_field_descriptions(relation_type);
        let (field_name, field_description) = match dir {
            InboundOutboundDirection::Outbound => (field_names.from_relation_to_outbound_entity, field_descriptions.from_relation_to_outbound_entity),
            InboundOutboundDirection::Inbound => (field_names.from_relation_to_inbound_entity, field_descriptions.from_relation_to_inbound_entity),
        };

        match ty {
            InboundOutboundType::EntityType(ty) => match ty {
                MatchingInboundOutboundType::NamespacedType(ty) => {
                    // If no field name was given via extensions, the field name is generated using
                    // the direction and the fully qualified type name. The direction is necessary
                    // because the outbound and inbound fields could be of the same type and every
                    // field name must be unique.
                    let field_name = field_name.unwrap_or_else(|| format!("{dir}_{}", ty.fully_qualified_type_name()));
                    let type_name = object_type_name(ty, RootObjectType::Query);
                    vec![create_inbound_outbound_field(dir, &type_name, &field_name, field_description)]
                }
                MatchingInboundOutboundType::Any => {
                    // Returns the union which represents all entities. If no field name was given
                    // via extensions, the field name is the direction (inbound or outbound).
                    let field_name = field_name.unwrap_or(format!("{dir}"));
                    vec![create_inbound_outbound_field(dir, UNION_ALL_ENTITIES, &field_name, field_description)]
                }
            },
            InboundOutboundType::Component(ty) => match ty {
                MatchingInboundOutboundType::NamespacedType(ty) => {
                    // If no field name was given via extensions, the field name is generated using
                    // the direction and the fully qualified type name. The direction is necessary
                    // because the outbound and inbound fields could be of the same type and every
                    // field name must be unique.
                    let field_name = field_name.unwrap_or_else(|| format!("{dir}_{}", ty.fully_qualified_type_name()));
                    let type_name = object_type_name(ty, RootObjectType::Interface);
                    vec![create_inbound_outbound_field(dir, &type_name, &field_name, field_description)]
                }
                MatchingInboundOutboundType::Any => {
                    self.component_manager
                        .get_type_ids()
                        .into_iter()
                        .map(|ty| {
                            // As we return multiple fields and every field name must be unique,
                            // we have to prefix the direction AND use the type name
                            let field_name = format!("{dir}_{}", ty.fully_qualified_type_name());
                            let type_name = object_type_name(&ty, RootObjectType::Interface);
                            create_inbound_outbound_field(dir.clone(), &type_name, &field_name, None)
                        })
                        .collect()
                }
            },
        }
    }
}

#[async_trait]
#[component_alias]
impl RelationQueryObjectFactory for RelationQueryObjectFactoryImpl {
    fn create_query_objects(&self) -> Vec<Object> {
        let mut query_objects = Vec::<Object>::new();
        for (_, relation_type) in self.relation_type_manager.get_all() {
            query_objects.push(self.create_query_object(relation_type));
        }
        query_objects
    }

    fn create_query_object(&self, relation_type: RelationType) -> Object {
        let object_type_name = object_type_name(&relation_type.ty, RootObjectType::Query);
        trace!("Create query object {object_type_name} for {}", &relation_type.ty);
        let mut object = Object::new(object_type_name)
            .description(&relation_type.description)
            .implement(INTERFACE_RELATION)
            // Namespace path field
            .field(namespace_path_field(relation_type.namespace()))
            // Relation ID field
            .field(relation_id_field())
            // Instance ID field
            .field(relation_instance_id_field());
        if let Some(field) = self
            .json_schema_field_factory
            .get_json_schema_field(FIELD_NAME_JSON_SCHEMA, &relation_type.type_definition())
        {
            object = object.field(field);
        }
        // `ComponentTypeIdContainer`s implements the interfaces of all components
        // and add a component id field for each component
        object = component_type_id_container_component_fields(&relation_type, object);
        // PropertyTypeContainer adds property fields
        object = property_container_property_fields::<RelationType, ReactiveRelation>(&relation_type, object);
        // Inbound and outbound Fields
        object = self.inbound_outbound_fields(&relation_type, object);
        object
    }
}

#[async_trait]
impl Lifecycle for RelationQueryObjectFactoryImpl {}

pub fn create_inbound_outbound_field(dir: InboundOutboundDirection, type_name: &str, field_name: &str, field_description: Option<String>) -> Field {
    let mut field = Field::new(field_name, TypeRef::named_nn(type_name), move |ctx| {
        FieldFuture::new({
            let inner_dir = dir.clone();
            async move {
                let dir = inner_dir.clone();
                let reactive_relation = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
                let reactive_entity = match dir {
                    InboundOutboundDirection::Outbound => reactive_relation.outbound.clone(),
                    InboundOutboundDirection::Inbound => reactive_relation.inbound.clone(),
                };
                Ok(Some(FieldValue::owned_any(reactive_entity)))
            }
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    field
}

// pub fn create_inbound_outbound_field_from_type<T: Into<NamespacedType>>(relation_type: &RelationType, dir: &InboundOutboundDirection, ty: MatchingInboundOutboundType<T>) -> Field {
//     let ty = ty.into();
//     let type_name = ty.fully_qualified_type_name();
//     let field_names = get_dynamic_graph_field_names(relation_type);
//     let field_descriptions = get_dynamic_graph_field_descriptions(relation_type);
//     let (field_name, field_description) = match dir {
//         InboundOutboundDirection::Outbound => (field_names.from_relation_to_outbound_entity, field_descriptions.from_relation_to_outbound_entity),
//         InboundOutboundDirection::Inbound => (field_names.from_relation_to_inbound_entity, field_descriptions.from_relation_to_inbound_entity),
//     };
//     let field_name = field_name.unwrap_or_else(|| format!("{dir}_{}", ty.fully_qualified_type_name()));
//     create_inbound_outbound_field(dir, type_name, field_name, field_description)
// }

// pub fn relation_inbound_field(
//     ty: &InboundOutboundType,
//     field_name: Option<String>,
//     field_description: Option<String>,
//     context: &SchemaBuilderContext,
// ) -> Vec<Field> {
//     match ty {
//         InboundOutboundType::EntityType(inbound_ty) => match inbound_ty {
//             MatchingInboundOutboundType::NamespacedType(entity_ty) => {
//                 vec![relation_inbound_entity_field(entity_ty, field_name, field_description)]
//             }
//             MatchingInboundOutboundType::Any => {
//                 vec![relation_inbound_entity_union_field(UNION_ALL_ENTITIES, field_name, field_description)]
//             }
//         },
//         InboundOutboundType::Component(inbound_ty) => match inbound_ty {
//             MatchingInboundOutboundType::NamespacedType(component_ty) => {
//                 vec![relation_inbound_component_field(component_ty, field_name, field_description)]
//             }
//             MatchingInboundOutboundType::Any => context
//                 .component_manager
//                 .get_type_ids()
//                 .iter()
//                 .map(|ty| relation_inbound_component_field(ty.key(), None, None))
//                 .collect(),
//         },
//     }
// }

// pub fn inbound_outbound__field(dir: &InboundOutboundDirection, ty: &NamespacedType, field_name: Option<String>, field_description: Option<String>) -> Field {
//     // Field name must be unique. Outbound and inbound type could be the same, therefore prefix with direction.
//     let field_name = field_name.unwrap_or_else(|| format!("{dir}_{}", ty.fully_qualified_type_name()));
//     create_inbound_outbound_field(dir, ty, &field_name, field_description)
// }
