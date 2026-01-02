use std::sync::Arc;

use crate::extension::field_description::get_dynamic_graph_field_descriptions;
use crate::extension::field_name::get_dynamic_graph_field_names;
use crate::field::entity::entity_id_field;
use crate::field::namespace_path_field;
use crate::field::optional_field_to_vec;
use crate::field::property::property_container_property_fields;
use crate::interface_manager_impl::component_type_id_container_component_fields;
use crate::object_type_name::object_type_name;
use crate::object_type_name::object_type_ref_list;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use async_trait::async_trait;
use log::trace;
use reactive_graph_dynamic_graph_api::EntityQueryObjectFactory;
use reactive_graph_dynamic_graph_api::FIELD_NAME_JSON_SCHEMA;
use reactive_graph_dynamic_graph_api::INTERFACE_ENTITY;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_dynamic_graph_api::UNION_ALL_ENTITIES;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::InboundOutboundDirection;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::MatchingInboundOutboundType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;

#[derive(Component)]
pub struct EntityQueryObjectFactoryImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

impl EntityQueryObjectFactoryImpl {
    /// Creates fields from the entity to the relations and other entities
    fn inbound_outbound_fields(&self, entity_type: &EntityType, mut object: Object) -> Object {
        let inbound_outbound_type = InboundOutboundType::from(entity_type);
        for outbound_relation_type in self.relation_type_manager.get_outbound_relation_types(&inbound_outbound_type, false).iter() {
            // From outbound entity to relation
            if let Some(outbound_entity_to_relation_field) =
                self.inbound_outbound_to_relation_field(outbound_relation_type.value(), InboundOutboundDirection::Outbound)
            {
                object = object.field(outbound_entity_to_relation_field);
            }
            // From outbound entity to inbound entities (skip the relation)
            for field in self.inbound_outbound_to_inbound_outbound_field(&outbound_relation_type, InboundOutboundDirection::Outbound) {
                object = object.field(field);
            }
        }
        for inbound_relation_type in self.relation_type_manager.get_inbound_relation_types(&inbound_outbound_type, false).iter() {
            // From inbound entity to relation
            if let Some(inbound_entity_to_relation_field) =
                self.inbound_outbound_to_relation_field(inbound_relation_type.value(), InboundOutboundDirection::Inbound)
            {
                object = object.field(inbound_entity_to_relation_field);
            }
            // From inbound entity to outbound entities (skip the relation)
            for field in self.inbound_outbound_to_inbound_outbound_field(&inbound_relation_type, InboundOutboundDirection::Inbound) {
                object = object.field(field);
            }
        }
        object
    }

    fn inbound_outbound_to_relation_field(&self, relation_type: &RelationType, dir: InboundOutboundDirection) -> Option<Field> {
        let Some(relation_type) = self.relation_type_manager.get(&relation_type.ty) else {
            return None;
        };

        let ty = relation_type.ty.clone();

        // Look up field names and descriptions in extensions
        let field_names = get_dynamic_graph_field_names(&relation_type);
        let field_descriptions = get_dynamic_graph_field_descriptions(&relation_type);
        let (field_name, field_description) = match dir {
            InboundOutboundDirection::Outbound => (field_names.from_outbound_entity_to_relation, field_descriptions.from_outbound_entity_to_relation),
            InboundOutboundDirection::Inbound => (field_names.from_inbound_entity_to_relation, field_descriptions.from_inbound_entity_to_relation),
        };
        let field_name = field_name.unwrap_or_else(|| format!("{dir}_{}", ty.fully_qualified_type_name()));
        if field_name.is_empty() {
            return None;
        }
        trace!("Create field {field_name} on {dir} entity to {}", &ty);
        let field = Field::new(field_name, object_type_ref_list(&ty, RootObjectType::Query), move |ctx| {
            let inbound_outbound_ty = ty.clone();
            FieldFuture::new({
                let inner_dir = dir.clone();
                async move {
                    let dir = inner_dir.clone();
                    let reactive_entity = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
                    let reactive_relation_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
                    let reactive_relations = match dir {
                        InboundOutboundDirection::Outbound => reactive_relation_manager.get_by_outbound_entity(reactive_entity.id),
                        InboundOutboundDirection::Inbound => reactive_relation_manager.get_by_inbound_entity(reactive_entity.id),
                    };
                    let reactive_relations: Vec<FieldValue> = reactive_relations
                        .iter()
                        .filter(|relation_instance| inbound_outbound_ty.clone() == relation_instance.relation_type_id())
                        .map(|relation_instance| FieldValue::owned_any(relation_instance.clone()))
                        .collect();
                    Ok(Some(FieldValue::list(reactive_relations)))
                }
            })
        })
        .description(field_description.unwrap_or(relation_type.description.clone()));
        Some(field)
    }

    fn inbound_outbound_to_inbound_outbound_field(&self, relation_type: &RelationType, dir: InboundOutboundDirection) -> Vec<Field> {
        let Some(relation_type) = self.relation_type_manager.get(&relation_type.ty) else {
            return Vec::new();
        };
        // Look up field names and descriptions in extensions
        let field_names = get_dynamic_graph_field_names(&relation_type);
        let field_descriptions = get_dynamic_graph_field_descriptions(&relation_type);
        let (field_name, field_description, inbound_outbound_type) = match dir {
            InboundOutboundDirection::Outbound => (
                field_names.from_outbound_entity_to_inbound_entity,
                field_descriptions.from_outbound_entity_to_inbound_entity,
                &relation_type.inbound_type,
            ),
            InboundOutboundDirection::Inbound => (
                field_names.from_inbound_entity_to_outbound_entity,
                field_descriptions.from_inbound_entity_to_outbound_entity,
                &relation_type.outbound_type,
            ),
        };
        match inbound_outbound_type {
            InboundOutboundType::EntityType(ty) => match ty {
                MatchingInboundOutboundType::NamespacedType(ty) => {
                    let field_name = field_name.unwrap_or_else(|| format!("{dir}_{}", ty.fully_qualified_type_name()));
                    let type_name = object_type_name(ty, RootObjectType::Query);
                    optional_field_to_vec(create_inbound_outbound_entity_to_inbound_outbound_field(
                        dir,
                        &relation_type.ty,
                        &type_name,
                        &field_name,
                        field_description,
                    ))
                }
                MatchingInboundOutboundType::Any => {
                    let field_name = field_name.unwrap_or(format!("{dir}"));
                    optional_field_to_vec(create_inbound_outbound_entity_to_inbound_outbound_field(
                        dir,
                        &relation_type.ty,
                        UNION_ALL_ENTITIES,
                        &field_name,
                        field_description,
                    ))
                }
            },
            InboundOutboundType::Component(ty) => match ty {
                MatchingInboundOutboundType::NamespacedType(ty) => {
                    let field_name = field_name.unwrap_or_else(|| format!("{dir}_{}", ty.fully_qualified_type_name()));
                    let type_name = object_type_name(ty, RootObjectType::Interface);
                    optional_field_to_vec(create_inbound_outbound_entity_to_inbound_outbound_field(
                        dir,
                        &relation_type.ty,
                        &type_name,
                        &field_name,
                        field_description,
                    ))
                }
                MatchingInboundOutboundType::Any => self
                    .component_manager
                    .get_type_ids()
                    .into_iter()
                    .filter_map(|ty| {
                        let field_name = format!("{dir}_{}", ty.fully_qualified_type_name());
                        let type_name = object_type_name(&ty, RootObjectType::Interface);
                        create_inbound_outbound_entity_to_inbound_outbound_field(dir.clone(), &relation_type.ty, &type_name, &field_name, None)
                    })
                    .collect(),
            },
        }
    }
}

#[async_trait]
#[component_alias]
impl EntityQueryObjectFactory for EntityQueryObjectFactoryImpl {
    fn create_query_objects(&self) -> Vec<Object> {
        let mut query_objects = Vec::new();
        for (_, entity_type) in self.entity_type_manager.get_all() {
            query_objects.push(self.create_query_object(entity_type));
        }
        query_objects
    }

    fn create_query_object(&self, entity_type: EntityType) -> Object {
        let object_type_name = object_type_name(&entity_type.ty, RootObjectType::Query);
        trace!("Create query object {object_type_name} for {}", &entity_type.ty);
        let mut object = Object::new(object_type_name)
            .description(&entity_type.description)
            .implement(INTERFACE_ENTITY)
            // Namespace path field
            .field(namespace_path_field(entity_type.namespace()))
            // ID field
            .field(entity_id_field());
        if let Some(field) = self
            .json_schema_field_factory
            .get_json_schema_field(FIELD_NAME_JSON_SCHEMA, &entity_type.type_definition())
        {
            object = object.field(field);
        }
        // `ComponentTypeIdContainer`s implements the interfaces of all components
        // and add a component id field for each component
        object = component_type_id_container_component_fields(&entity_type, object);
        // PropertyTypeContainer adds property fields
        object = property_container_property_fields::<EntityType, ReactiveEntity>(&entity_type, object);
        // Inbound and outbound fields
        object = self.inbound_outbound_fields(&entity_type, object);
        object
    }
}

#[async_trait]
impl Lifecycle for EntityQueryObjectFactoryImpl {}

pub fn create_inbound_outbound_entity_to_inbound_outbound_field(
    dir: InboundOutboundDirection,
    relation_ty: &RelationTypeId,
    type_name: &str,
    field_name: &str,
    field_description: Option<String>,
) -> Option<Field> {
    if field_name.is_empty() {
        return None;
    }
    let relation_ty_inner = relation_ty.clone();
    trace!("Create field {field_name} via {} to {dir} entity {type_name}", &relation_ty);
    let mut field = Field::new(field_name, TypeRef::named_nn_list_nn(type_name), move |ctx| {
        let relation_ty = relation_ty_inner.clone();
        FieldFuture::new({
            let inner_dir = dir.clone();
            async move {
                let dir = inner_dir.clone();
                let reactive_relation_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
                let reactive_entity = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
                Ok(Some(FieldValue::list(
                    reactive_relation_manager
                        .get_by_inbound_entity(reactive_entity.id)
                        .iter()
                        .filter(|reactive_relation| reactive_relation.relation_type_id() == relation_ty)
                        .map(|relation_instance| {
                            let other_reactive_entity = match dir {
                                InboundOutboundDirection::Outbound => relation_instance.inbound.clone(),
                                InboundOutboundDirection::Inbound => relation_instance.outbound.clone(),
                            };
                            // In case of a union, we have to specify the actual type name
                            let actual_type_name = object_type_name(&other_reactive_entity.ty, RootObjectType::Query);
                            FieldValue::owned_any(other_reactive_entity).with_type(actual_type_name)
                        }),
                )))
            }
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    Some(field)
}
