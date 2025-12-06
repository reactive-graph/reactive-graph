use crate::field::json::to_field_value;
use crate::type_ref::TYPE_REF_JSON;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_trait::async_trait;
use convert_case::Case::Pascal;
use convert_case::Casing;
use reactive_graph_dynamic_graph_api::FIELD_JSON_SCHEMA_APPENDIX;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::NamespacedType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionJsonSchemaGetter;
use reactive_graph_graph::TypeIdType;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct JsonSchemaFieldFactoryImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
}

impl JsonSchemaFieldFactoryImpl {}

#[async_trait]
#[component_alias]
impl JsonSchemaFieldFactory for JsonSchemaFieldFactoryImpl {
    fn get_json_schema_field(&self, field_name: &str, type_definition: &TypeDefinition) -> Option<Field> {
        let description = format!("JSON schema of {} {}", type_definition.type_id_type.full_name(), type_definition.namespace());
        let schema = match type_definition.type_id_type {
            TypeIdType::Component => {
                let ty = ComponentTypeId::try_from(type_definition).ok()?;
                self.component_manager.get(&ty)?.json_schema()
            }
            TypeIdType::EntityType => {
                let ty = EntityTypeId::try_from(type_definition).ok()?;
                self.entity_type_manager.get(&ty)?.json_schema()
            }
            TypeIdType::RelationType => {
                let ty = RelationTypeId::try_from(type_definition).ok()?;
                self.relation_type_manager.get(&ty)?.json_schema()
            }
            TypeIdType::FlowType => {
                let ty = FlowTypeId::try_from(type_definition).ok()?;
                let flow_type = self.flow_type_manager.get(&ty)?;
                let wrapper_type = self.entity_type_manager.get(&flow_type.wrapper_type())?;
                flow_type.json_schema(&wrapper_type).ok()?
            }
            _ => {
                return None;
            }
        };
        Some(
            Field::new(field_name, TYPE_REF_JSON.clone(), move |_ctx| {
                let schema = schema.clone();
                FieldFuture::new(async move { Ok(to_field_value(schema.as_value().clone())) })
            })
            .description(description),
        )
    }
}

#[async_trait]
impl Lifecycle for JsonSchemaFieldFactoryImpl {}

pub fn json_schema_field_name<T: Into<NamespacedType>>(ty: T) -> String {
    format!("_{}{}", ty.into().type_name.to_string().to_case(Pascal), FIELD_JSON_SCHEMA_APPENDIX)
}
