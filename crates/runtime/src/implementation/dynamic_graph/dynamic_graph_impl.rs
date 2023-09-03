use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;

use apollo_compiler::values::DirectiveDefinition;
use apollo_compiler::values::EnumTypeDefinition;
use apollo_compiler::values::Field;
use apollo_compiler::values::FieldDefinition;
use apollo_compiler::values::FragmentDefinition;
use apollo_compiler::values::InputObjectTypeDefinition;
use apollo_compiler::values::ObjectTypeDefinition;
use apollo_compiler::values::OperationDefinition;
use apollo_compiler::values::OperationType;
use apollo_compiler::values::RootOperationTypeDefinition;
use apollo_compiler::values::ScalarTypeDefinition;
use apollo_compiler::values::SelectionSet;
use apollo_compiler::values::UnionTypeDefinition;
use apollo_compiler::ApolloCompiler;
use apollo_compiler::ApolloDiagnostic;
use apollo_encoder::Document;
use apollo_encoder::ObjectDefinition;
use apollo_encoder::SchemaDefinition;
use apollo_encoder::Type_::List;
use apollo_encoder::Type_::NamedType;
use apollo_parser::ast::Definition;
use async_graphql::indexmap::IndexMap;
use async_graphql::Name;
use async_graphql::Pos;
use async_graphql::Response;
use async_graphql::ServerError;
use async_graphql_actix_web::GraphQLRequest;
use async_graphql_actix_web::GraphQLResponse;
use async_trait::async_trait;
use convert_case::Case;
use convert_case::Casing;
use log::debug;
use log::error;
use log::info;
use log::trace;
use log::warn;
use uuid::Uuid;

use crate::api::ComponentManager;
use crate::api::DynamicGraph;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityManager;
use crate::api::ReactiveRelationManager;
use crate::api::RelationTypeManager;
use crate::api::SystemEventManager;
use crate::di::*;
use crate::graphql::dynamic_graph::introspection::IntrospectionType;
use crate::graphql::dynamic_graph::introspection::ResolverType;
use crate::graphql::dynamic_graph::introspection::BUILTIN_INTROSPECTION_SCHEMA;
use crate::graphql::dynamic_graph::introspection::INTROSPECTION_SCHEMA;
use crate::graphql::dynamic_graph::schema_generation::enum_definition;
use crate::graphql::dynamic_graph::schema_generation::scalar_definition;
use crate::implementation::PROPERTY_EVENT;
use crate::reactive::ComponentContainer;
use crate::model::EntityType;
use crate::model::PropertyInstanceGetter;
use crate::reactive::ReactiveEntity;
use crate::reactive::ReactivePropertyContainer;
use crate::reactive::ReactiveRelation;
use crate::model::RelationType;
use crate::plugins::SystemEventTypes;

static UUID_TYPE_SYSTEM_CHANGED_EVENT: Uuid = Uuid::from_u128(0x6ba7b8109e1511d150b900c04fe530c7);

const RELATION_TYPES_BLACKLIST: &[&str] = &[
    "debug_connector",
    "default_connector",
    "parse_float_connector",
    "parse_int_connector",
    "to_string_connector",
    "trace_connector",
    "buffered_fifo_connector",
    "debounce_connector",
    "decrement_by_connector",
    "delay_connector",
    "increment_by_connector",
    "numeric_interpolation_connector",
    "threaded_connector",
    //
    // "categorized_as",
    // "tagged_with",
];

#[wrapper]
pub struct DynamicSchemaContainer(RwLock<Option<Arc<Document>>>);

#[provides]
fn create_dynamic_schema() -> DynamicSchemaContainer {
    DynamicSchemaContainer(RwLock::new(None))
}

#[wrapper]
pub struct TypeSystemModifiedStateContainer(Arc<RwLock<bool>>);

#[provides]
fn create_dynamic_schema_modified() -> TypeSystemModifiedStateContainer {
    TypeSystemModifiedStateContainer(Arc::new(RwLock::new(true)))
}

#[component]
pub struct DynamicGraphImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    entity_instance_manager: Wrc<dyn ReactiveEntityManager>,

    relation_instance_manager: Wrc<dyn ReactiveRelationManager>,

    dynamic_schema: DynamicSchemaContainer,

    type_system_modified_state: TypeSystemModifiedStateContainer,
}

#[async_trait]
#[provides]
impl DynamicGraph for DynamicGraphImpl {
    fn is_type_system_modified(&self) -> bool {
        *self.type_system_modified_state.0.read().unwrap().deref()
    }

    fn create_sdl(&self) -> Document {
        let mut document = Document::new();
        document.scalar(scalar_definition("UUID", None));
        document.scalar(scalar_definition("JSON", None));
        document.enum_(enum_definition("TypeType", vec!["COMPONENT", "ENTITY_TYPE", "RELATION_TYPE"]));

        let mut object_definition_query = ObjectDefinition::new("Query".to_owned());

        object_definition_query.field(apollo_encoder::FieldDefinition::new(
            "__schema".to_owned(),
            NamedType {
                name: INTROSPECTION_SCHEMA.to_owned(),
            },
        ));

        // let mut object_definition_entities = ObjectDefinition::new("Entities".to_owned());
        // // object_definition_entities.field(apollo_encoder::FieldDefinition::new(
        // //     "entities".to_owned(),
        // //     NamedType {
        // //         name: INTROSPECTION_SCHEMA.to_owned(),
        // //     },
        // // ));
        // let mut field_definition_entities = apollo_encoder::FieldDefinition::new("entities".to_owned(), NamedType { name: "Entities".to_owned() });
        // field_definition_entities.description("The entities".to_owned());
        // object_definition_query.field(field_definition_entities);

        for component in self.component_manager.get_all() {
            let type_name = component_type_name(&component);
            // let mut interface_definition = InterfaceDefinition::new(format!("component__{}", component.name.clone()));
            let mut object_definition = ObjectDefinition::new(type_name);
            object_definition.description(component.description.clone());
            // _type = component
            let mut field_definition_type = apollo_encoder::FieldDefinition::new("_type".to_owned(), NamedType { name: "TypeType".to_owned() });
            field_definition_type.description("The type is always component".to_owned());
            object_definition.field(field_definition_type);
            for property_type in &component.properties {
                // interface_definition.field(FieldDefinition::new(property_type.name.clone(), NamedType { name: "JSON".to_owned() }));
                let mut field_definition_property = apollo_encoder::FieldDefinition::new(property_type.name.clone(), NamedType { name: "JSON".to_owned() });
                field_definition_property.description(property_type.description.clone());
                object_definition.field(field_definition_property);
            }
            // interface_definition.field(FieldDefinition::new(
            //     format!("component__{}", component.name.clone()),
            //     NamedType { name: "String".to_owned() },
            // ));
            // document.interface(interface_definition);
            // let mut field_definition = FieldDefinition::new(component_type_name(&component), NamedType { name: "String".to_owned() });
            // field_definition.description(component.description.clone());
            // object_definition.field(field_definition);
            document.object(object_definition);
        }

        for entity_type in self.entity_type_manager.get_entity_types() {
            let type_name = entity_type_name(&entity_type);

            let mut object_definition = ObjectDefinition::new(type_name.clone());
            object_definition.description(entity_type.description.clone());
            let mut field_definition_type = apollo_encoder::FieldDefinition::new("_type".to_owned(), NamedType { name: "TypeType".to_owned() });
            field_definition_type.description("The meta type is always entity_type".to_owned());
            object_definition.field(field_definition_type);

            let mut field_definition_id = apollo_encoder::FieldDefinition::new("_id".to_owned(), NamedType { name: "UUID".to_owned() });
            field_definition_id.description("The unique identifier of the entity instance".to_owned());
            object_definition.field(field_definition_id);

            let mut field_definition_label = apollo_encoder::FieldDefinition::new("_label".to_owned(), NamedType { name: "String".to_owned() });
            field_definition_label.description("The label of the entity instance".to_owned());
            object_definition.field(field_definition_label);

            for component_name in &entity_type.components {
                if let Some(component) = self.component_manager.get(component_name) {
                    let component_type_name = component_type_name(&component);
                    // object_definition.interface(format!("component__{}", component_name.clone()));
                    let mut field_definition = apollo_encoder::FieldDefinition::new(
                        component_name.clone(),
                        // format!("component__{}", component_name.clone()),
                        NamedType { name: component_type_name },
                    );
                    field_definition.description(component.description.clone());
                    object_definition.field(field_definition);
                }
            }

            for property_type in &entity_type.properties {
                let property_name = property_type.name.clone();
                if entity_type.components.contains(&property_name) {
                    continue;
                }
                if entity_type
                    .components
                    .iter()
                    .flat_map(|component_name| self.component_manager.get(component_name))
                    .any(|component| component.has_property(&property_name))
                {
                    continue;
                }
                let mut field_definition_property = apollo_encoder::FieldDefinition::new(property_name.clone(), NamedType { name: "JSON".to_owned() });
                field_definition_property.description(property_type.description.clone());
                object_definition.field(field_definition_property);
            }

            let outbound_relation_types = self.relation_type_manager.get_outbound_relation_types(&entity_type.name, true);
            let inbound_relation_types = self.relation_type_manager.get_inbound_relation_types(&entity_type.name, true);

            // let outbound_relation_types = self.relation_type_manager.get_outbound_relation_types(&entity_type.name, false);
            // if !outbound_relation_types.is_empty() {
            //     let outbound_type_name = format!("{}Outbound", type_name.clone());
            //     let mut object_definition_outbound = ObjectDefinition::new(outbound_type_name.clone());
            //     for outbound_relation_type in outbound_relation_types {
            //         let relation_type_name = relation_type_name(&outbound_relation_type);
            //         // info!("{} --[{}]--> {}", entity_type.name, outbound_relation_type.type_name, outbound_relation_type.inbound_type);
            //         let mut field_definition_property = apollo_encoder::FieldDefinition::new(
            //             outbound_relation_type.type_name.clone(),
            //             List {
            //                 ty: Box::from(NamedType { name: relation_type_name }),
            //             },
            //         );
            //         field_definition_property.description(outbound_relation_type.description.clone());
            //         object_definition_outbound.field(field_definition_property);
            //     }
            //     document.object(object_definition_outbound);
            //     let mut field_definition_outbound = apollo_encoder::FieldDefinition::new("outbound".to_owned(), NamedType { name: outbound_type_name });
            //     field_definition_outbound.description("The outbound relations".to_owned());
            //     object_definition.field(field_definition_outbound);
            // }

            for outbound_relation_type in outbound_relation_types.iter().filter(|relation_type| {
                !(RELATION_TYPES_BLACKLIST.contains(&relation_type.type_name.as_str())
                    || relation_type.outbound_type == "*" && relation_type.inbound_type == entity_type.name)
            }) {
                let field_name = match outbound_relation_type
                    .extensions
                    .iter()
                    .find(|extension| extension.name == "outbound_collection_field_name")
                    .map(|extension| {
                        extension
                            .extension
                            .as_str()
                            .map_or(format!("outbound_{}", outbound_relation_type.type_name), |field_name| field_name.to_string())
                    }) {
                    Some(field_name) => field_name,
                    None => {
                        if inbound_relation_types
                            .iter()
                            .any(|inbound_relation_type| outbound_relation_type.type_name == inbound_relation_type.type_name)
                        {
                            format!("outbound_{}", outbound_relation_type.type_name.clone())
                        } else {
                            outbound_relation_type.type_name.clone()
                        }
                    }
                };

                let mut field_definition_outbound = apollo_encoder::FieldDefinition::new(
                    field_name,
                    List {
                        ty: Box::from(NamedType {
                            name: relation_type_name(outbound_relation_type),
                        }),
                    },
                );
                field_definition_outbound.description(outbound_relation_type.description.clone());
                object_definition.field(field_definition_outbound);
            }

            // let inbound_relation_types = self.relation_type_manager.get_inbound_relation_types(&entity_type.name, false);
            // if !inbound_relation_types.is_empty() {
            //     let inbound_type_name = format!("{}Inbound", type_name.clone());
            //     let mut object_definition_inbound = ObjectDefinition::new(inbound_type_name.clone());
            //     for inbound_relation_type in inbound_relation_types {
            //         let relation_type_name = relation_type_name(&inbound_relation_type);
            //         // info!("{} --[{}]--> {}", inbound_relation_type.inbound_type, inbound_relation_type.type_name, entity_type.name);
            //         let mut field_definition_property = apollo_encoder::FieldDefinition::new(
            //             inbound_relation_type.type_name.clone(),
            //             List {
            //                 ty: Box::from(NamedType { name: relation_type_name }),
            //             },
            //         );
            //         field_definition_property.description(inbound_relation_type.description.clone());
            //         object_definition_inbound.field(field_definition_property);
            //     }
            //     document.object(object_definition_inbound);
            //     let mut field_definition_inbound = apollo_encoder::FieldDefinition::new("inbound".to_owned(), NamedType { name: inbound_type_name });
            //     field_definition_inbound.description("The inbound relations".to_owned());
            //     object_definition.field(field_definition_inbound);
            // }

            for inbound_relation_type in inbound_relation_types.iter().filter(|relation_type| {
                !(RELATION_TYPES_BLACKLIST.contains(&relation_type.type_name.as_str())
                    || relation_type.inbound_type == "*" && relation_type.outbound_type == entity_type.name)
            }) {
                let field_name = match inbound_relation_type
                    .extensions
                    .iter()
                    .find(|extension| extension.name == "inbound_collection_field_name")
                    .map(|extension| {
                        extension
                            .extension
                            .as_str()
                            .map_or(format!("inbound_{}", inbound_relation_type.type_name), |field_name| field_name.to_string())
                    }) {
                    Some(field_name) => field_name,
                    None => {
                        if outbound_relation_types
                            .iter()
                            .any(|outbound_relation_type| inbound_relation_type.type_name == outbound_relation_type.type_name)
                        {
                            format!("inbound_{}", inbound_relation_type.type_name.clone())
                        } else {
                            inbound_relation_type.type_name.clone()
                        }
                    }
                };

                let mut field_definition_inbound = apollo_encoder::FieldDefinition::new(
                    field_name,
                    List {
                        ty: Box::from(NamedType {
                            name: relation_type_name(inbound_relation_type),
                        }),
                    },
                );
                field_definition_inbound.description(inbound_relation_type.description.clone());
                object_definition.field(field_definition_inbound);
            }

            document.object(object_definition);

            let mut field_definition_query = apollo_encoder::FieldDefinition::new(
                entity_type.name.clone(),
                List {
                    ty: Box::from(NamedType { name: type_name.clone() }),
                },
            );
            field_definition_query.description(entity_type.description.clone());
            object_definition_query.field(field_definition_query)
        }

        for relation_type in self.relation_type_manager.get_relation_types() {
            let type_name = relation_type_name(&relation_type);

            let mut object_definition = ObjectDefinition::new(type_name.clone());
            object_definition.description(relation_type.description.clone());
            let mut field_definition_type = apollo_encoder::FieldDefinition::new("_type".to_owned(), NamedType { name: "TypeType".to_owned() });
            field_definition_type.description("The type is always relation_type".to_owned());
            object_definition.field(field_definition_type);

            // TODO: Make EdgeKey a Type
            let mut field_definition_edge_key = apollo_encoder::FieldDefinition::new("_edge_key".to_owned(), NamedType { name: "String".to_owned() });
            field_definition_edge_key.description("The edge key of the relation instance".to_owned());
            object_definition.field(field_definition_edge_key);

            let mut field_definition_label = apollo_encoder::FieldDefinition::new("_label".to_owned(), NamedType { name: "String".to_owned() });
            field_definition_label.description("The label of the relation instance".to_owned());
            object_definition.field(field_definition_label);

            for component_name in &relation_type.components {
                if let Some(component) = self.component_manager.get(component_name) {
                    let component_type_name = component_type_name(&component);
                    // object_definition.interface(format!("component__{}", component_name.clone()));
                    let mut field_definition = apollo_encoder::FieldDefinition::new(
                        component_name.clone(),
                        // format!("component__{}", component_name.clone()),
                        NamedType { name: component_type_name },
                    );
                    field_definition.description(component.description.clone());
                    object_definition.field(field_definition);
                }
            }

            for property_type in &relation_type.properties {
                let property_name = property_type.name.clone();
                if relation_type.components.contains(&property_name) {
                    continue;
                }
                if relation_type
                    .components
                    .iter()
                    .flat_map(|component_name| self.component_manager.get(component_name))
                    .any(|component| component.has_property(&property_name))
                {
                    continue;
                }
                let mut field_definition_property = apollo_encoder::FieldDefinition::new(property_name.clone(), NamedType { name: "JSON".to_owned() });
                field_definition_property.description(property_type.description.clone());
                object_definition.field(field_definition_property);
            }

            if let Some(outbound_type) = self.entity_type_manager.get(&relation_type.outbound_type) {
                let outbound_field_name = match relation_type
                    .extensions
                    .iter()
                    .find(|extension| extension.name == "outbound_field_name")
                    .map(|extension| extension.extension.as_str().map_or(format!("outbound_{}", relation_type.outbound_type), |field_name| field_name.to_string()))
                    // .map(|extension| extension.extension.to_string())
                {
                    Some(outbound_field_name) => outbound_field_name,
                    None => {
                        if relation_type.outbound_type == relation_type.inbound_type {
                            format!("outbound_{}", &relation_type.outbound_type)
                        } else {
                            relation_type.outbound_type.clone()
                        }
                    }
                };
                let field_description = match relation_type
                    .extensions
                    .iter()
                    .find(|extension| extension.name == "outbound_field_description")
                    .map(|extension| {
                        extension
                            .extension
                            .as_str()
                            .map_or(outbound_type.description.clone(), |field_name| field_name.to_string())
                    }) {
                    Some(outbound_field_description) => outbound_field_description,
                    None => outbound_type.description.clone(),
                };
                let mut field_definition_outbound = apollo_encoder::FieldDefinition::new(
                    outbound_field_name,
                    NamedType {
                        name: entity_type_name(&outbound_type),
                    },
                );
                field_definition_outbound.description(field_description);
                object_definition.field(field_definition_outbound);
            }

            if let Some(inbound_type) = self.entity_type_manager.get(&relation_type.inbound_type) {
                let inbound_field_name = match relation_type
                    .extensions
                    .iter()
                    .find(|extension| extension.name == "inbound_field_name")
                    .map(|extension| extension.extension.as_str().map_or(format!("outbound_{}", relation_type.inbound_type), |field_name| field_name.to_string()))
                    // .map(|extension| extension.extension.to_string())
                {
                    Some(inbound_field_name) => inbound_field_name,
                    None => {
                        if relation_type.inbound_type == relation_type.outbound_type {
                            format!("inbound_{}", &relation_type.inbound_type)
                        } else {
                            relation_type.inbound_type.clone()
                        }
                    }
                };
                let field_description = match relation_type
                    .extensions
                    .iter()
                    .find(|extension| extension.name == "inbound_field_description")
                    .map(|extension| {
                        extension
                            .extension
                            .as_str()
                            .map_or(inbound_type.description.clone(), |field_name| field_name.to_string())
                    }) {
                    Some(field_description) => field_description,
                    None => inbound_type.description.clone(),
                };
                let mut field_definition_inbound = apollo_encoder::FieldDefinition::new(
                    inbound_field_name,
                    NamedType {
                        name: entity_type_name(&inbound_type),
                    },
                );
                field_definition_inbound.description(field_description);
                object_definition.field(field_definition_inbound);
            }

            document.object(object_definition);

            let mut field_definition_query = apollo_encoder::FieldDefinition::new(
                relation_type.type_name.clone(),
                List {
                    ty: Box::from(NamedType { name: type_name.clone() }),
                },
            );
            field_definition_query.description(relation_type.description.clone());
            object_definition_query.field(field_definition_query)
        }

        document.object(object_definition_query);

        let mut schema_definition = SchemaDefinition::new();
        schema_definition.query("Query".to_string());
        // schema_definition.mutation("Mutation".to_string());
        // schema_definition.subscription("Subscription".to_string());

        document.schema(schema_definition);

        document
    }

    fn regenerate_schema(&self) {
        debug!("Regenerate dynamic schema");
        let mut guard = self.dynamic_schema.0.write().unwrap();
        *guard = Some(Arc::new(self.create_sdl()));
        let mut guard = self.type_system_modified_state.0.write().unwrap();
        *guard = false;
    }

    fn regenerate_schema_if_modified(&self) {
        if self.is_type_system_modified() {
            trace!("The type system has been modified. Regenerating the dynamic schema");
            self.regenerate_schema();
        }
    }

    fn execute_request(&self, request: GraphQLRequest) -> GraphQLResponse {
        debug!("Dynamic Graph: Executing query:\n{}", request.0.query.as_str());
        self.regenerate_schema_if_modified();
        let guard = self.dynamic_schema.0.read().unwrap();
        match guard.clone() {
            Some(dynamic_schema) => {
                let merged = format!("{}\n{}\n{}", dynamic_schema.to_string(), BUILTIN_INTROSPECTION_SCHEMA, request.0.query.as_str());
                let compiler = ApolloCompiler::new(merged.as_str());
                let diagnostics = compiler.validate();
                if !diagnostics.is_empty() {
                    for diagnostic in &diagnostics {
                        if diagnostic.is_advice() {
                            info!("{}", diagnostic.to_string());
                        }
                        if diagnostic.is_warning() {
                            warn!("{}", diagnostic.to_string());
                        }
                        if diagnostic.is_error() {
                            error!("{}", diagnostic.to_string());
                        }
                    }
                    if diagnostics.iter().any(|diagnostic| diagnostic.is_error()) {
                        let diagnostics_errors = diagnostics
                            .iter()
                            .map(|diagnostic| {
                                match diagnostic {
                                    // ApolloDiagnostic::MissingIdent(_) => {}
                                    // ApolloDiagnostic::MissingField(_) => {}
                                    // ApolloDiagnostic::UniqueDefinition(_) => {}
                                    // ApolloDiagnostic::SingleRootField(_) => {}
                                    // ApolloDiagnostic::UnsupportedOperation(_) => {}
                                    // ApolloDiagnostic::SyntaxError(_) => {}
                                    // ApolloDiagnostic::UniqueField(_) => {}
                                    // ApolloDiagnostic::UndefinedDefinition(_) => {}
                                    ApolloDiagnostic::UndefinedField(undefined_field) => {
                                        let message = format!("{}: {}", undefined_field.field.as_str(), undefined_field.help.as_str());
                                        let pos = Pos::from((undefined_field.definition.offset(), undefined_field.definition.len()));
                                        ServerError::new(message, Some(pos))
                                        // let mut e = ServerError::new(message, Some(pos));
                                        // e.source.replace(Arc::from(String::from(undefined_field.src.as_str())));
                                        // e
                                    }
                                    // ApolloDiagnostic::RecursiveDefinition(_) => {}
                                    // ApolloDiagnostic::TransitiveImplementedInterfaces(_) => {}
                                    // ApolloDiagnostic::QueryRootOperationType(query_root_operation_) => {}
                                    // ApolloDiagnostic::BuiltInScalarDefinition(_) => {}
                                    // ApolloDiagnostic::ScalarSpecificationURL(_) => {}
                                    // ApolloDiagnostic::CapitalizedValue(_) => {}
                                    // ApolloDiagnostic::UnusedVariable(_) => {}
                                    // ApolloDiagnostic::OutputType(_) => {}
                                    // ApolloDiagnostic::ObjectType(_) => {}
                                    _ => ServerError::new(diagnostic.to_string(), None),
                                }
                            })
                            .collect();
                        return Response::from_errors(diagnostics_errors).into();
                    }
                }
                for operation in compiler.operations().iter() {
                    let _ = self.execute_operation(&compiler, operation);
                }
                match request.0.operation_name {
                    Some(operation_name) => {
                        if let Some(operation_definition) = compiler
                            .operations()
                            .iter()
                            .filter(|o| {
                                if let Some(op_name) = o.name() {
                                    if op_name == operation_name.as_str() {
                                        return true;
                                    }
                                }
                                false
                            })
                            .collect::<Vec<&OperationDefinition>>()
                            .first()
                        {
                            let data = self.execute_operation(&compiler, operation_definition);
                            return Response::new(data).into();
                        }
                    }
                    None => {
                        // Execute first query operation if possible
                        if let Some(operation_definition) = compiler.operations().iter().filter(is_query).collect::<Vec<&OperationDefinition>>().first() {
                            let data = self.execute_operation(&compiler, operation_definition);
                            return Response::new(data).into();
                        }
                    }
                }
                Response::from_errors(vec![ServerError::new("No operation found", None)]).into()
            }
            None => Response::from_errors(vec![ServerError::new("Dynamic schema not available", None)]).into(),
        }
    }

    fn execute_operation(&self, compiler: &ApolloCompiler, operation: &OperationDefinition) -> async_graphql::Value {
        debug!("Executing operation {:?}", operation.name().map(|n| n.to_string()).unwrap_or_else(|| "---".to_owned()));
        let mut fields = self.resolve_fields_index_map(compiler, operation.fields(&compiler.db));
        fields.extend(self.resolve_fields_index_map(compiler, operation.fields_in_inline_fragments(&compiler.db)));
        fields.extend(self.resolve_fields_index_map(compiler, operation.fields_in_fragment_spread(&compiler.db)));
        async_graphql::Value::Object(fields)
    }

    fn resolve_fields(&self, compiler: &ApolloCompiler, fields: Arc<Vec<Field>>) -> async_graphql::Value {
        async_graphql::Value::Object(self.resolve_fields_index_map(compiler, fields))
    }

    fn resolve_fields_index_map(&self, compiler: &ApolloCompiler, fields: Arc<Vec<Field>>) -> IndexMap<Name, async_graphql::Value> {
        let mut result = IndexMap::new();
        for field in fields.iter() {
            // info!("  field {}: {:?}", field.name(), field.ty(&compiler.db).map(|ty| ty.name()).unwrap_or("---".to_owned()));
            let field_data = match &field.ty(&compiler.db) {
                Some(ty) => match ty.into() {
                    ResolverType::IntrospectionType(introspection_type) => match introspection_type {
                        IntrospectionType::Schema => self.resolve_schema(compiler, field),
                        IntrospectionType::Type => self.resolve_types(compiler, field),
                        // IntrospectionType::InputValue => self.resolve_types(compiler, field),
                        // IntrospectionType:: => self.resolve_types(compiler, field),
                        IntrospectionType::Directive => self.resolve_directives(compiler, field),
                        _ => async_graphql::Value::Null,
                    },
                    ResolverType::Other => self.resolve_instances(compiler, field),
                },
                None => async_graphql::Value::Null,
            };
            result.insert(Name::new(field.name()), field_data);
        }
        result
    }

    fn resolve_instances(&self, compiler: &ApolloCompiler, field: &Field) -> async_graphql::Value {
        let mut resolved_instances = Vec::new();
        resolved_instances.extend(self.resolve_entity_instances(compiler, field));
        resolved_instances.extend(self.resolve_relation_instances(compiler, field));
        async_graphql::Value::List(resolved_instances)
    }

    fn resolve_entity_instances(&self, compiler: &ApolloCompiler, field: &Field) -> Vec<async_graphql::Value> {
        let ty = field.ty(&compiler.db).unwrap();
        let entity_type_name = reverse_entity_type_name(&ty.name());
        trace!(
            "Resolve field {}: {:?} entity_type_name {}",
            field.name(),
            field
                .ty(&compiler.db)
                .map(|ty| format!("{} named: {} list: {} non-null: {}", ty.name(), ty.is_named(), ty.is_list(), ty.is_non_null()))
                .unwrap_or_else(|| "---".to_owned()),
            entity_type_name
        );
        let mut resolved_entity_instances = Vec::new();
        // Filtering:
        // field.arguments()
        if self.entity_type_manager.has(&entity_type_name) {
            let entity_instances: Vec<ReactiveEntity> = self
                .entity_instance_manager
                .get_entity_instances()
                .iter()
                .filter(|entity_instance| entity_type_name == entity_instance.type_name.clone())
                .cloned()
                .collect();
            for entity_instance in entity_instances {
                resolved_entity_instances.push(self.resolve_entity_instance(compiler, field, &entity_instance, false));
            }
        }
        resolved_entity_instances
    }

    fn resolve_entity_instance(
        &self,
        compiler: &ApolloCompiler,
        field: &Field,
        entity_instance: &ReactiveEntity,
        in_component: bool,
    ) -> async_graphql::Value {
        let selection_set = field.selection_set();
        let mut resolved_fields = self.resolve_entity_instance_index_map(compiler, selection_set.fields(), entity_instance, in_component);
        for fragment_spread in selection_set.fragment_spreads().iter() {
            if let Some(fragment_definition) = fragment_spread.fragment(&compiler.db) {
                resolved_fields.extend(self.resolve_entity_instance_index_map(
                    compiler,
                    fragment_definition.selection_set().fields(),
                    entity_instance,
                    in_component,
                ));
            }
        }
        for inline_fragment in selection_set.inline_fragments().iter() {
            resolved_fields.extend(self.resolve_entity_instance_index_map(compiler, inline_fragment.selection_set().fields(), entity_instance, in_component));
        }
        async_graphql::Value::Object(resolved_fields)
    }

    fn resolve_entity_instance_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        entity_instance: &ReactiveEntity,
        in_component: bool,
    ) -> IndexMap<Name, async_graphql::Value> {
        let mut resolved_fields = IndexMap::new();
        for field in fields.iter() {
            match field.name() {
                "_type" => {
                    resolved_fields.insert(Name::new(field.name()), async_graphql::Value::String(entity_instance.type_name.clone()));
                }
                "_id" => {
                    resolved_fields.insert(Name::new(field.name()), async_graphql::Value::String(entity_instance.id.to_string()));
                }
                "_label" => {
                    resolved_fields.insert(Name::new(field.name()), async_graphql::Value::String("TODO".to_owned()));
                }
                _ => {
                    if !in_component && entity_instance.is_a(field.name()) {
                        resolved_fields.insert(Name::new(field.name()), self.resolve_entity_instance(compiler, field, entity_instance, true));
                    } else {
                        match entity_instance.get(field.name()) {
                            Some(property_value) => match async_graphql::Value::from_json(property_value) {
                                Ok(value) => {
                                    resolved_fields.insert(Name::new(field.name()), value);
                                }
                                Err(_) => {
                                    resolved_fields.insert(Name::new(field.name()), async_graphql::Value::Null);
                                }
                            },
                            None => {
                                // XYZ
                                let mut resolved_relation_instances = Vec::new();
                                let relation_type = match field.ty(&compiler.db).map(|field_type| reverse_relation_type_name(&field_type.name())) {
                                    Some(relation_type_name) => self.relation_type_manager.get_starts_with(&relation_type_name),
                                    None => None,
                                };
                                if relation_type.is_none() {
                                    error!("Missing relation type for field {}", field.name());
                                }
                                if let Some(relation_type) = relation_type {
                                    info!(
                                        "Relation type found {} ---[{}]---> {}",
                                        relation_type.type_name, relation_type.outbound_type, relation_type.inbound_type
                                    );
                                    let outbound_field_name = relation_type
                                        .extensions
                                        .iter()
                                        .find(|extension| extension.name == "outbound_collection_field_name")
                                        .map(|extension| {
                                            extension
                                                .extension
                                                .as_str()
                                                .map_or(format!("outbound_{}", relation_type.outbound_type), |field_name| field_name.to_string())
                                        })
                                        .unwrap_or(format!("outbound_{}", relation_type.outbound_type));
                                    info!("Field {} outbound_field_name {}", field.name(), outbound_field_name);
                                    if field.name() == outbound_field_name {
                                        info!("  is_outbound");
                                        for relation_instance in self
                                            .relation_instance_manager
                                            .get_by_outbound_entity(entity_instance.id)
                                            .iter()
                                            .filter(|relation_instance| relation_instance.type_name.starts_with(relation_type.type_name.as_str()))
                                        {
                                            info!("    relation_instance found");
                                            resolved_relation_instances.push(self.resolve_relation_instance(compiler, field, relation_instance, false));
                                        }
                                    }

                                    let inbound_field_name = relation_type
                                        .extensions
                                        .iter()
                                        .find(|extension| extension.name == "inbound_collection_field_name")
                                        .map(|extension| {
                                            extension
                                                .extension
                                                .as_str()
                                                .map_or(format!("inbound_{}", relation_type.inbound_type), |field_name| field_name.to_string())
                                        })
                                        .unwrap_or(format!("inbound_{}", relation_type.inbound_type));
                                    info!("Field {} inbound_field_name {}", field.name(), inbound_field_name);
                                    if field.name() == inbound_field_name {
                                        info!("  is_inbound");
                                        for relation_instance in self
                                            .relation_instance_manager
                                            .get_by_inbound_entity(entity_instance.id)
                                            .iter()
                                            .filter(|relation_instance| relation_instance.type_name.starts_with(relation_type.type_name.as_str()))
                                        {
                                            info!("    relation_instance found");
                                            resolved_relation_instances.push(self.resolve_relation_instance(compiler, field, relation_instance, false));
                                        }
                                    }
                                }
                                resolved_fields.insert(Name::new(field.name()), async_graphql::Value::List(resolved_relation_instances));

                                // let outbound_field_name = match field.ty(&compiler.db).map(|field_type| reverse_relation_type_name(&field_type.name())) {
                                //     Some(relation_type_name) => match self.relation_type_manager.get_starts_with(relation_type_name) {
                                //         Some(relation_type) => {
                                //             match relation_type
                                //                 .extensions
                                //                 .iter()
                                //                 .find(|extension| extension.name == "outbound_collection_name")
                                //                 .map(|extension| {
                                //                     extension
                                //                         .extension
                                //                         .as_str()
                                //                         .map_or(format!("outbound_{}", relation_type.outbound_type), |field_name| field_name.to_string())
                                //                 }) {
                                //                 Some(field_name) => field_name,
                                //                 None => format!("outbound_{}", relation_type.outbound_type),
                                //             }
                                //         }
                                //         None => format!("outbound_{}", relation_type_name),
                                //     }
                                //     None => format!("outbound_{}", field.name()),
                                // };
                                // let outbound_field_name =
                                //     match relation_type.clone() {
                                //         Some(relation_type) => match relation_type
                                //             .extensions
                                //             .iter()
                                //             .find(|extension| extension.name == "outbound_field_name")
                                //             .map(|extension| {
                                //                 extension
                                //                     .extension
                                //                     .as_str()
                                //                     .map_or(format!("outbound_{}", relation_type.outbound_type), |field_name| field_name.to_string())
                                //             }) {
                                //             Some(field_name) => field_name,
                                //             None => format!("outbound_{}", relation_type.outbound_type),
                                //         },
                                //         None => format!("outbound_{}", relation_instance.type_name.clone()),
                                //     };

                                // BEFORE

                                // let is_outbound = field.name().starts_with("outbound_");
                                // let is_inbound = field.name().starts_with("inbound_");
                                // let field_name = field.name().replace("outbound_", "").replace("inbound_", "");
                                // let mut resolved_relation_instances = Vec::new();
                                // if let Some(relation_type) = self.relation_type_manager.get(field_name.clone()) {
                                //     if is_outbound {
                                //         for relation_instance in self
                                //             .relation_instance_manager
                                //             .get_by_outbound_entity(entity_instance.id)
                                //             .iter()
                                //             .filter(|relation_instance| relation_instance.type_name.starts_with(relation_type.type_name.as_str()))
                                //         {
                                //             resolved_relation_instances.push(self.resolve_relation_instance(compiler, field, relation_instance, false));
                                //         }
                                //     }
                                //     if is_inbound {
                                //         for relation_instance in self
                                //             .relation_instance_manager
                                //             .get_by_inbound_entity(entity_instance.id)
                                //             .iter()
                                //             .filter(|relation_instance| relation_instance.type_name.starts_with(relation_type.type_name.as_str()))
                                //         {
                                //             resolved_relation_instances.push(self.resolve_relation_instance(compiler, field, relation_instance, false));
                                //         }
                                //     }
                                // }
                                // resolved_fields.insert(Name::new(field.name()), async_graphql::Value::List(resolved_relation_instances));
                            }
                        }
                    }
                }
            }
        }
        resolved_fields
    }

    fn resolve_relation_instances(&self, compiler: &ApolloCompiler, field: &Field) -> Vec<async_graphql::Value> {
        let ty = field.ty(&compiler.db).unwrap();
        let relation_type_name = reverse_relation_type_name(&ty.name());
        trace!(
            "Resolve field {}: {:?} relation_type_name {}",
            field.name(),
            field
                .ty(&compiler.db)
                .map(|ty| format!("{} named: {} list: {} non-null: {}", ty.name(), ty.is_named(), ty.is_list(), ty.is_non_null()))
                .unwrap_or_else(|| "---".to_owned()),
            relation_type_name
        );
        let mut resolved_relation_instances = Vec::new();
        // Filtering:
        // field.arguments()
        if self.relation_type_manager.has(&relation_type_name) {
            let relation_instances: Vec<ReactiveRelation> = self
                .relation_instance_manager
                .get_all()
                .iter()
                .filter(|relation_instance| relation_instance.type_name.starts_with(relation_type_name.as_str()))
                .cloned()
                .collect();
            for relation_instance in relation_instances {
                resolved_relation_instances.push(self.resolve_relation_instance(compiler, field, &relation_instance, false));
            }
        }
        resolved_relation_instances
    }

    fn resolve_relation_instance(
        &self,
        compiler: &ApolloCompiler,
        field: &Field,
        relation_instance: &ReactiveRelation,
        in_component: bool,
    ) -> async_graphql::Value {
        let selection_set = field.selection_set();
        let mut resolved_fields = self.resolve_relation_instance_index_map(compiler, selection_set.fields(), relation_instance, in_component);
        for fragment_spread in selection_set.fragment_spreads().iter() {
            if let Some(fragment_definition) = fragment_spread.fragment(&compiler.db) {
                resolved_fields.extend(self.resolve_relation_instance_index_map(
                    compiler,
                    fragment_definition.selection_set().fields(),
                    relation_instance,
                    in_component,
                ));
            }
        }
        for inline_fragment in selection_set.inline_fragments().iter() {
            resolved_fields.extend(self.resolve_relation_instance_index_map(
                compiler,
                inline_fragment.selection_set().fields(),
                relation_instance,
                in_component,
            ));
        }
        async_graphql::Value::Object(resolved_fields)
    }

    fn resolve_relation_instance_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        relation_instance: &ReactiveRelation,
        in_component: bool,
    ) -> IndexMap<Name, async_graphql::Value> {
        let mut resolved_fields = IndexMap::new();
        for field in fields.iter() {
            match field.name() {
                "_type" => {
                    resolved_fields.insert(Name::new(field.name()), async_graphql::Value::String(relation_instance.type_name.clone()));
                }
                "_edge_key" => {
                    let edge_key = relation_instance.get_key();
                    resolved_fields.insert(
                        Name::new(field.name()),
                        async_graphql::Value::String(format!("{}-{}-{}", edge_key.outbound_id, edge_key.t.to_string(), edge_key.inbound_id)),
                    );
                }
                "_label" => {
                    resolved_fields.insert(Name::new(field.name()), async_graphql::Value::String("TODO".to_owned()));
                }
                _ => {
                    if !in_component && relation_instance.is_a(field.name()) {
                        resolved_fields.insert(Name::new(field.name()), self.resolve_relation_instance(compiler, field, relation_instance, true));
                    } else {
                        match relation_instance.get(field.name()) {
                            Some(property_value) => match async_graphql::Value::from_json(property_value) {
                                Ok(value) => {
                                    resolved_fields.insert(Name::new(field.name()), value);
                                }
                                Err(_) => {
                                    resolved_fields.insert(Name::new(field.name()), async_graphql::Value::Null);
                                }
                            },
                            None => {
                                let relation_type = self.relation_type_manager.get_starts_with(&relation_instance.type_name);
                                let outbound_field_name =
                                    match relation_type.clone() {
                                        Some(relation_type) => match relation_type
                                            .extensions
                                            .iter()
                                            .find(|extension| extension.name == "outbound_field_name")
                                            .map(|extension| {
                                                extension
                                                    .extension
                                                    .as_str()
                                                    .map_or(format!("outbound_{}", relation_type.outbound_type), |field_name| field_name.to_string())
                                            }) {
                                            Some(field_name) => field_name,
                                            None => format!("outbound_{}", relation_type.outbound_type),
                                        },
                                        None => format!("outbound_{}", relation_instance.type_name.clone()),
                                    };
                                let inbound_field_name =
                                    match relation_type {
                                        Some(relation_type) => match relation_type
                                            .extensions
                                            .iter()
                                            .find(|extension| extension.name == "inbound_field_name")
                                            .map(|extension| {
                                                extension
                                                    .extension
                                                    .as_str()
                                                    .map_or(format!("outbound_{}", relation_type.outbound_type), |field_name| field_name.to_string())
                                            }) {
                                            Some(field_name) => field_name,
                                            None => format!("inbound_{}", relation_type.inbound_type),
                                        },
                                        None => format!("inbound_{}", relation_instance.type_name.clone()),
                                    };
                                // let is_outbound = field.name() == outbound_field_name;
                                // let is_inbound = field.name() == outbound_field_name;
                                // let is_outbound = field.name().starts_with("outbound_");
                                // let is_inbound = field.name().starts_with("inbound_");
                                // let field_name = field.name().replace("outbound_", "").replace("inbound_", "");
                                if field.name() == outbound_field_name {
                                    resolved_fields
                                        .insert(Name::new(field.name()), self.resolve_entity_instance(compiler, field, &relation_instance.outbound, false));
                                }
                                if field.name() == inbound_field_name {
                                    resolved_fields
                                        .insert(Name::new(field.name()), self.resolve_entity_instance(compiler, field, &relation_instance.inbound, false));
                                }
                            }
                        }
                    }
                }
            }
        }
        resolved_fields
    }

    fn resolve_schema(&self, compiler: &ApolloCompiler, field: &Field) -> async_graphql::Value {
        // info!("Resolving schema {}", field.name());
        async_graphql::Value::Object(self.resolve_selection_set_index_map(compiler, field.selection_set()))
    }

    fn resolve_selection_set_index_map(&self, compiler: &ApolloCompiler, selection_set: &SelectionSet) -> IndexMap<Name, async_graphql::Value> {
        let mut resolved_fields = self.resolve_fields_index_map(compiler, Arc::from(selection_set.fields()));
        for fragment_spread in selection_set.fragment_spreads().iter() {
            if let Some(fragment_definition) = fragment_spread.fragment(&compiler.db) {
                resolved_fields.extend(self.resolve_fields_index_map(compiler, Arc::from(fragment_definition.selection_set().fields())));
            }
        }
        for inline_fragment in selection_set.inline_fragments().iter() {
            resolved_fields.extend(self.resolve_fields_index_map(compiler, Arc::from(inline_fragment.selection_set().fields())));
        }
        resolved_fields
    }

    fn resolve_types(&self, compiler: &ApolloCompiler, field: &Field) -> async_graphql::Value {
        let ty = field.ty(&compiler.db).unwrap();
        // info!("Resolving introspection type {} of type {}", field.name(), ty.name().as_str());
        match field.name() {
            "queryType" | "mutationType" | "subscriptionType" => {
                return self.resolve_root_operation_type(compiler, field);
            }
            _ => {}
        }
        if !ty.is_list() {
            async_graphql::Value::List(Vec::new());
        }
        let mut types = Vec::new();
        types.extend(
            compiler
                .scalars()
                .iter()
                .map(|scalar_type_definition| self.resolve_scalar_type_definition(compiler, field, scalar_type_definition)),
        );
        types.extend(compiler.enums().iter().map(|enum_type| self.resolve_enum_type(compiler, field, enum_type)));
        types.extend(compiler.unions().iter().map(|union_type| self.resolve_union_type(compiler, field, union_type)));
        types.extend(
            compiler
                .object_types()
                .iter()
                .filter(|object_type_definition| !object_type_definition.name().starts_with("__"))
                // .filter(|object_type_definition| object_type_definition.name() != "__Schema")
                .map(|object_type_definition| self.resolve_object_type(compiler, field, object_type_definition)),
        );
        types.extend(
            compiler
                .input_objects()
                .iter()
                .map(|input_object_type_definition| self.resolve_input_object_type_definition(compiler, field, input_object_type_definition)),
        );
        // types.extend(
        //     compiler
        //         .fragments()
        //         .iter()
        //         .map(|fragment_definition| self.resolve_fragment_definition(compiler, field, fragment_definition)),
        // );
        async_graphql::Value::List(types)
    }

    fn resolve_type(&self, compiler: &ApolloCompiler, ty: apollo_compiler::values::Type) -> async_graphql::Value {
        match ty {
            apollo_compiler::values::Type::NonNull { ty, ast_ptr: _ } => {
                let mut result = IndexMap::new();
                result.insert(Name::new("kind"), async_graphql::Value::String("NON_NULL".to_owned()));
                result.insert(Name::new("name"), async_graphql::Value::Null);
                result.insert(Name::new("ofType"), self.resolve_type(compiler, *ty));
                async_graphql::Value::Object(result)
            }
            apollo_compiler::values::Type::List { ty, ast_ptr: _ } => {
                let mut result = IndexMap::new();
                result.insert(Name::new("kind"), async_graphql::Value::String("LIST".to_owned()));
                result.insert(Name::new("name"), async_graphql::Value::Null);
                result.insert(Name::new("ofType"), self.resolve_type(compiler, *ty));
                async_graphql::Value::Object(result)
            }
            apollo_compiler::values::Type::Named { name, ast_ptr } => {
                let kind = find_type_name(compiler, name.as_str());
                if kind.is_none() {
                    error!("Failed to resolve type name: {:?} {:?}", name, ast_ptr.map(|ast_ptr| ast_ptr.kind()));
                }

                let mut result = IndexMap::new();
                result.insert(
                    Name::new("kind"),
                    match kind {
                        Some(kind) => async_graphql::Value::String(kind),
                        None => async_graphql::Value::Null,
                    },
                );
                result.insert(Name::new("name"), async_graphql::Value::String(name.to_owned()));
                result.insert(Name::new("ofType"), async_graphql::Value::Null);
                async_graphql::Value::Object(result)
            }
        }
    }

    fn resolve_root_operation_type(&self, compiler: &ApolloCompiler, field: &Field) -> async_graphql::Value {
        // let mut resolved_type = IndexMap::new();
        // info!("Resolving root operation type {}", field.name());
        if let Some(field_operation_type) = match field.name() {
            "queryType" => Some(OperationType::Query),
            "mutationType" => Some(OperationType::Mutation),
            "subscriptionType" => Some(OperationType::Subscription),
            _ => None,
        } {
            // info!("  field_operation_type: {}", field_operation_type);
            if let Some(root_operation_type_definition) = compiler
                .schema()
                .root_operation_type_definition()
                .iter()
                .filter(|root_operation_type_definition| root_operation_type_definition.operation_type() == field_operation_type)
                .map(|root_operation_type_definition| root_operation_type_definition.to_owned())
                .collect::<Vec<RootOperationTypeDefinition>>()
                .first()
            {
                // info!("  root_operation_type_definition type: {}", root_operation_type_definition.named_type().name());
                if let Some(object_type) = compiler
                    .object_types()
                    .iter()
                    .cloned()
                    .filter(|object_type_definition| object_type_definition.name() == root_operation_type_definition.named_type().name().as_str())
                    .collect::<Vec<ObjectTypeDefinition>>()
                    .first()
                {
                    // info!("  object_type: {}", object_type.name());
                    return self.resolve_object_type(compiler, field, object_type);
                }
            }
        }
        async_graphql::Value::Null
    }

    fn resolve_scalar_type_definition(&self, compiler: &ApolloCompiler, field: &Field, scalar_type_definition: &ScalarTypeDefinition) -> async_graphql::Value {
        let selection_set = field.selection_set();
        let mut resolved_fields = self.resolve_scalar_type_definition_index_map(selection_set.fields(), scalar_type_definition);
        for fragment_spread in selection_set.fragment_spreads().iter() {
            if let Some(fragment_definition) = fragment_spread.fragment(&compiler.db) {
                resolved_fields.extend(self.resolve_scalar_type_definition_index_map(fragment_definition.selection_set().fields(), scalar_type_definition));
            }
        }
        for inline_fragment in selection_set.inline_fragments().iter() {
            resolved_fields.extend(self.resolve_scalar_type_definition_index_map(inline_fragment.selection_set().fields(), scalar_type_definition));
        }
        async_graphql::Value::Object(resolved_fields)
    }

    fn resolve_scalar_type_definition_index_map(
        &self,
        fields: Vec<Field>,
        scalar_type_definition: &ScalarTypeDefinition,
    ) -> IndexMap<Name, async_graphql::Value> {
        let mut resolved_fields = IndexMap::new();
        for field in fields.iter() {
            resolved_fields.insert(
                Name::new(field.name()),
                match field.name() {
                    "kind" => async_graphql::Value::String("SCALAR".to_owned()),
                    "name" => async_graphql::Value::String(scalar_type_definition.name().to_string()),
                    "description" => match scalar_type_definition.description() {
                        Some(description) => async_graphql::Value::String(description.to_string()),
                        None => async_graphql::Value::Null,
                    },
                    "fields" => async_graphql::Value::Null,
                    _ => async_graphql::Value::Null,
                },
            );
        }
        resolved_fields
    }

    fn resolve_enum_type(&self, compiler: &ApolloCompiler, field: &Field, enum_type_definition: &EnumTypeDefinition) -> async_graphql::Value {
        let selection_set = field.selection_set();
        let mut resolved_fields = self.resolve_enum_type_index_map(selection_set.fields(), enum_type_definition);
        for fragment_spread in selection_set.fragment_spreads().iter() {
            if let Some(fragment_definition) = fragment_spread.fragment(&compiler.db) {
                resolved_fields.extend(self.resolve_enum_type_index_map(fragment_definition.selection_set().fields(), enum_type_definition));
            }
        }
        for inline_fragment in selection_set.inline_fragments().iter() {
            resolved_fields.extend(self.resolve_enum_type_index_map(inline_fragment.selection_set().fields(), enum_type_definition));
        }
        async_graphql::Value::Object(resolved_fields)
    }

    fn resolve_enum_type_index_map(&self, fields: Vec<Field>, enum_type_definition: &EnumTypeDefinition) -> IndexMap<Name, async_graphql::Value> {
        let mut resolved_fields = IndexMap::new();
        for enum_field in fields.iter() {
            resolved_fields.insert(
                Name::new(enum_field.name()),
                match enum_field.name() {
                    "kind" => async_graphql::Value::String("ENUM".to_owned()),
                    "name" => async_graphql::Value::String(enum_type_definition.name().to_string()),
                    "description" => match enum_type_definition.description() {
                        Some(description) => async_graphql::Value::String(description.to_string()),
                        None => async_graphql::Value::Null,
                    },
                    "fields" => async_graphql::Value::Null,
                    "inputFields" => async_graphql::Value::Null,
                    "interfaces" => async_graphql::Value::Null,
                    "enumValues" => self.resolve_enum_values(enum_type_definition),
                    _ => async_graphql::Value::Null,
                },
            );
        }
        resolved_fields
    }

    fn resolve_enum_values(&self, enum_type: &EnumTypeDefinition) -> async_graphql::Value {
        let mut enum_values = Vec::new();
        for enum_value_definition in enum_type.enum_values_definition().iter() {
            let mut enum_value = IndexMap::new();
            enum_value.insert(Name::new("name"), async_graphql::Value::String(enum_value_definition.enum_value().to_string()));
            enum_value.insert(
                Name::new("description"),
                match enum_value_definition.description() {
                    Some(description) => async_graphql::Value::String(description.to_string()),
                    None => async_graphql::Value::Null,
                },
            );
            enum_value.insert(Name::new("isDeprecated"), async_graphql::Value::Boolean(false));
            enum_value.insert(Name::new("deprecationReason"), async_graphql::Value::Null);
            enum_values.push(async_graphql::Value::Object(enum_value));
        }
        async_graphql::Value::List(enum_values)
    }

    fn resolve_union_type(&self, compiler: &ApolloCompiler, field: &Field, union_type_definition: &UnionTypeDefinition) -> async_graphql::Value {
        let selection_set = field.selection_set();
        let mut resolved_fields = self.resolve_union_type_index_map(compiler, selection_set.fields(), union_type_definition);
        for fragment_spread in selection_set.fragment_spreads().iter() {
            if let Some(fragment_definition) = fragment_spread.fragment(&compiler.db) {
                resolved_fields.extend(self.resolve_union_type_index_map(compiler, fragment_definition.selection_set().fields(), union_type_definition));
            }
        }
        for inline_fragment in selection_set.inline_fragments().iter() {
            resolved_fields.extend(self.resolve_union_type_index_map(compiler, inline_fragment.selection_set().fields(), union_type_definition));
        }
        async_graphql::Value::Object(resolved_fields)
    }

    fn resolve_union_type_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        union_type_definition: &UnionTypeDefinition,
    ) -> IndexMap<Name, async_graphql::Value> {
        let mut resolved_fields = IndexMap::new();
        for union_field in fields.iter() {
            resolved_fields.insert(
                Name::new(union_field.name()),
                match union_field.name() {
                    "kind" => async_graphql::Value::String("UNION".to_owned()),
                    "name" => async_graphql::Value::String(union_type_definition.name().to_string()),
                    "description" => match union_type_definition.description() {
                        Some(description) => async_graphql::Value::String(description.to_string()),
                        None => async_graphql::Value::Null,
                    },
                    "possibleFields" => self.resolve_union_members(compiler, union_type_definition),
                    _ => async_graphql::Value::Null,
                },
            );
        }
        resolved_fields
    }

    fn resolve_union_members(&self, compiler: &ApolloCompiler, union_type_definition: &UnionTypeDefinition) -> async_graphql::Value {
        let mut resolved_union_members = Vec::new();
        for union_member in union_type_definition.union_members() {
            if let Some(object_type_definition) = union_member.object(&compiler.db) {
                let mut type_ref = IndexMap::new();
                type_ref.insert(Name::new("kind"), async_graphql::Value::String("OBJECT".to_owned()));
                type_ref.insert(Name::new("name"), async_graphql::Value::String(object_type_definition.name().to_string()));
                type_ref.insert(Name::new("ofType"), async_graphql::Value::Null);
                resolved_union_members.push(async_graphql::Value::Object(type_ref));
            }
        }
        async_graphql::Value::List(resolved_union_members)
    }

    fn resolve_object_type(&self, compiler: &ApolloCompiler, field: &Field, object_type_definition: &ObjectTypeDefinition) -> async_graphql::Value {
        let selection_set = field.selection_set();
        let mut resolved_fields = self.resolve_object_type_index_map(compiler, selection_set.fields(), object_type_definition);
        for fragment_spread in selection_set.fragment_spreads().iter() {
            if let Some(fragment_definition) = fragment_spread.fragment(&compiler.db) {
                resolved_fields.extend(self.resolve_object_type_index_map(compiler, fragment_definition.selection_set().fields(), object_type_definition));
            }
        }
        for inline_fragment in selection_set.inline_fragments().iter() {
            resolved_fields.extend(self.resolve_object_type_index_map(compiler, inline_fragment.selection_set().fields(), object_type_definition));
        }
        async_graphql::Value::Object(resolved_fields)
    }

    fn resolve_object_type_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        object_type_definition: &ObjectTypeDefinition,
    ) -> IndexMap<Name, async_graphql::Value> {
        let mut resolved_fields = IndexMap::new();
        for object_type_field in fields.iter() {
            let value = match object_type_field.name() {
                "kind" => async_graphql::Value::String("OBJECT".to_owned()),
                "name" => async_graphql::Value::String(object_type_definition.name().to_string()),
                "description" => match object_type_definition.description() {
                    None => async_graphql::Value::Null,
                    Some(description) => async_graphql::Value::String(description.to_string()),
                },
                "fields" => async_graphql::Value::List(
                    object_type_definition
                        .fields_definition()
                        .iter()
                        .filter(|field_definition| !field_definition.name().starts_with("__"))
                        .map(|field_definition| self.resolve_field(compiler, field_definition, Arc::new(object_type_field.selection_set().fields())))
                        .collect(),
                ),
                "interfaces" => {
                    // TODO: !!! object_type_definition.implements_interfaces()
                    async_graphql::Value::List(Vec::new())
                }
                _ => async_graphql::Value::Null,
            };
            resolved_fields.insert(Name::new(object_type_field.name()), value);
        }
        resolved_fields
    }

    fn resolve_input_object_type_definition(
        &self,
        compiler: &ApolloCompiler,
        field: &Field,
        input_object_type_definition: &InputObjectTypeDefinition,
    ) -> async_graphql::Value {
        let selection_set = field.selection_set();
        let mut resolved_fields = self.resolve_input_object_type_definition_index_map(compiler, selection_set.fields(), input_object_type_definition);
        for fragment_spread in selection_set.fragment_spreads().iter() {
            if let Some(fragment_definition) = fragment_spread.fragment(&compiler.db) {
                resolved_fields.extend(self.resolve_input_object_type_definition_index_map(
                    compiler,
                    fragment_definition.selection_set().fields(),
                    input_object_type_definition,
                ));
            }
        }
        for inline_fragment in selection_set.inline_fragments().iter() {
            resolved_fields.extend(self.resolve_input_object_type_definition_index_map(
                compiler,
                inline_fragment.selection_set().fields(),
                input_object_type_definition,
            ));
        }
        async_graphql::Value::Object(resolved_fields)
    }

    fn resolve_input_object_type_definition_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        input_object_type_definition: &InputObjectTypeDefinition,
    ) -> IndexMap<Name, async_graphql::Value> {
        let mut resolved_fields = IndexMap::new();
        for input_object_field in fields.iter() {
            let value = match input_object_field.name() {
                "kind" => async_graphql::Value::String("INPUT_OBJECT".to_owned()),
                "name" => async_graphql::Value::String(input_object_type_definition.name().to_string()),
                "description" => match input_object_type_definition.description() {
                    None => async_graphql::Value::Null,
                    Some(description) => async_graphql::Value::String(description.to_string()),
                },
                "input_fields" => async_graphql::Value::List(Vec::new()), // TODO: !!!
                // "fields" => async_graphql::Value::List(
                //     input_object_type_definition
                //         .
                //         .fields_definition()
                //         .iter()
                //         .map(|field_definition| self.resolve_field(compiler, field_definition, Arc::new(type_field.selection_set().fields())))
                //         .collect(),
                // ),
                _ => async_graphql::Value::Null,
            };
            resolved_fields.insert(Name::new(input_object_field.name()), value);
        }
        resolved_fields
    }

    fn resolve_fragment_definition(&self, compiler: &ApolloCompiler, field: &Field, fragment_definition: &FragmentDefinition) -> async_graphql::Value {
        let selection_set = field.selection_set();
        let mut resolved_fields = self.resolve_fragment_definition_index_map(compiler, selection_set.fields(), fragment_definition);
        for fragment_spread in selection_set.fragment_spreads().iter() {
            if let Some(fragment_definition_2) = fragment_spread.fragment(&compiler.db) {
                resolved_fields.extend(self.resolve_fragment_definition_index_map(
                    compiler,
                    fragment_definition_2.selection_set().fields(),
                    fragment_definition,
                ));
            }
        }
        for inline_fragment in selection_set.inline_fragments().iter() {
            resolved_fields.extend(self.resolve_fragment_definition_index_map(compiler, inline_fragment.selection_set().fields(), fragment_definition));
        }
        async_graphql::Value::Object(resolved_fields)
    }

    fn resolve_fragment_definition_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        fragment_definition: &FragmentDefinition,
    ) -> IndexMap<Name, async_graphql::Value> {
        let mut resolved_fields = IndexMap::new();
        for fragment_definition_field in fields.iter() {
            resolved_fields.insert(
                Name::new(fragment_definition_field.name()),
                match fragment_definition_field.name() {
                    "kind" => async_graphql::Value::String("FRAGMENT_DEFINITION".to_owned()),
                    "name" => async_graphql::Value::String(fragment_definition.name().to_string()),
                    "fields" => async_graphql::Value::List(
                        fragment_definition
                            .selection_set()
                            // .fields_definition()
                            .fields()
                            .iter()
                            .map(|fragment_definition_field| fragment_definition_field.field_definition(&compiler.db))
                            .map(|fragment_definition| match fragment_definition {
                                Some(fragment_definition) => {
                                    self.resolve_field(compiler, &fragment_definition, Arc::new(fragment_definition_field.selection_set().fields()))
                                } // TODO <-- selection_set
                                None => async_graphql::Value::Null,
                            })
                            .collect(),
                    ),
                    _ => async_graphql::Value::Null,
                },
            );
        }
        resolved_fields
    }

    fn resolve_directives(&self, compiler: &ApolloCompiler, field: &Field) -> async_graphql::Value {
        let ty = field.ty(&compiler.db).unwrap();
        info!("Resolving introspection type {} of type {}", field.name(), ty.name().as_str());
        if !ty.is_list() {
            return async_graphql::Value::List(Vec::new());
        }
        let mut directives = Vec::new();
        directives.extend(
            compiler
                .directive_definitions()
                .iter()
                .map(|directive_definition| self.resolve_directive_definition(compiler, field, directive_definition)),
        );
        async_graphql::Value::List(directives)
    }

    fn resolve_directive_definition(&self, compiler: &ApolloCompiler, field: &Field, directive_definition: &DirectiveDefinition) -> async_graphql::Value {
        let selection_set = field.selection_set();
        let mut resolved_fields = self.resolve_directive_definition_index_map(compiler, selection_set.fields(), directive_definition);
        for fragment_spread in selection_set.fragment_spreads().iter() {
            if let Some(fragment_definition) = fragment_spread.fragment(&compiler.db) {
                resolved_fields.extend(self.resolve_directive_definition_index_map(
                    compiler,
                    fragment_definition.selection_set().fields(),
                    directive_definition,
                ));
            }
        }
        for inline_fragment in selection_set.inline_fragments().iter() {
            resolved_fields.extend(self.resolve_directive_definition_index_map(compiler, inline_fragment.selection_set().fields(), directive_definition));
        }
        async_graphql::Value::Object(resolved_fields)
    }

    fn resolve_directive_definition_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        directive_definition: &DirectiveDefinition,
    ) -> IndexMap<Name, async_graphql::Value> {
        let mut resolved_fields = IndexMap::new();
        for directive_definition_field in fields.iter() {
            resolved_fields.insert(
                Name::new(directive_definition_field.name()),
                match directive_definition_field.name() {
                    "name" => async_graphql::Value::String(directive_definition.name().to_string()),
                    "description" => match directive_definition.description() {
                        None => async_graphql::Value::Null,
                        Some(description) => async_graphql::Value::String(description.to_string()),
                    },
                    "locations" => async_graphql::Value::List(
                        directive_definition
                            .directive_locations()
                            .iter()
                            .map(|location| async_graphql::Value::String(location.to_owned().into()))
                            .collect(),
                    ),
                    "args" => async_graphql::Value::List(Vec::new()), // TODO !!!
                    _ => async_graphql::Value::Null,
                },
            );
        }
        resolved_fields
    }

    fn resolve_field(&self, compiler: &ApolloCompiler, field: &FieldDefinition, fields: Arc<Vec<Field>>) -> async_graphql::Value {
        trace!("Resolving field {} fields", field.name());
        let mut resolved_field = IndexMap::new();
        for selection_set_field in fields.iter() {
            resolved_field.insert(
                Name::new(selection_set_field.name()),
                match selection_set_field.name() {
                    "name" => async_graphql::Value::String(field.name().to_string()),
                    "description" => match field.description() {
                        None => async_graphql::Value::Null,
                        Some(description) => async_graphql::Value::String(description.to_string()),
                    },
                    "args" => async_graphql::Value::List(Vec::new()), // TODO !!!
                    "type" => match selection_set_field.ty(&compiler.db) {
                        Some(ty) => self.resolve_type(compiler, field.ty().to_owned()),
                        None => async_graphql::Value::Null,
                    },
                    "isDeprecated" => async_graphql::Value::Boolean(false),
                    "deprecationReason" => async_graphql::Value::Null,
                    _ => async_graphql::Value::Null,
                },
            );
        }
        async_graphql::Value::Object(resolved_field)
    }
}

fn is_query(operation: &&OperationDefinition) -> bool {
    operation.operation_ty().is_query()
}

#[async_trait]
impl Lifecycle for DynamicGraphImpl {
    async fn init(&self) {}

    async fn post_init(&self) {
        if let Some(event_type_system_changed) = self.event_manager.get_system_event_instance(SystemEventTypes::TypeSystemChanged) {
            let type_system_modified_state = self.type_system_modified_state.0.clone();
            event_type_system_changed.observe_with_handle(
                PROPERTY_EVENT,
                move |v| {
                    if v.is_boolean() && v.as_bool().unwrap() {
                        // The type system has changed -> regenerate the dynamic schema
                        let mut guard = type_system_modified_state.write().unwrap();
                        *guard = true;
                    }
                },
                UUID_TYPE_SYSTEM_CHANGED_EVENT.as_u128(),
            );
        }
    }

    async fn pre_shutdown(&self) {
        if let Some(event_type_system_changed) = self.event_manager.get_system_event_instance(SystemEventTypes::TypeSystemChanged) {
            event_type_system_changed.remove_observer(PROPERTY_EVENT, UUID_TYPE_SYSTEM_CHANGED_EVENT.as_u128());
            // event_type_system_changed
            //     .properties
            //     .get(PROPERTY_EVENT)
            //     .unwrap()
            //     .stream
            //     .read()
            //     .unwrap()
            //     .remove(UUID_TYPE_SYSTEM_CHANGED_EVENT.as_u128());
        }
    }

    async fn shutdown(&self) {}
}

fn component_type_name(component: &crate::model::Component) -> String {
    format!("c{}", component.name.to_case(Case::Pascal))
}

fn entity_type_name(entity_type: &EntityType) -> String {
    entity_type.name.to_case(Case::Pascal)
}

fn entity_type_name2(entity_type_name: &String) -> String {
    entity_type_name.to_case(Case::Pascal)
}

fn relation_type_name(relation_type: &RelationType) -> String {
    relation_type.type_name.to_case(Case::Pascal)
}

fn reverse_entity_type_name(entity_type_name: &String) -> String {
    entity_type_name.to_case(Case::Snake)
}

fn reverse_relation_type_name(relation_type_name: &String) -> String {
    relation_type_name.to_case(Case::Snake)
}

fn find_type_name(compiler: &ApolloCompiler, ty_name: &str) -> Option<String> {
    match ty_name {
        "String" | "Boolean" | "Int" | "Float" => return Some("SCALAR".to_owned()),
        _ => {}
    }
    for definition in compiler.definitions().iter() {
        match definition {
            Definition::EnumTypeDefinition(d) => {
                if let Some(d_name) = d.name().map(|name| name.text()) {
                    if ty_name == d_name.as_str() {
                        return Some("ENUM".to_owned());
                    }
                }
            }
            Definition::ScalarTypeDefinition(d) => {
                if let Some(d_name) = d.name().map(|name| name.text()) {
                    if ty_name == d_name.as_str() {
                        return Some("SCALAR".to_owned());
                    }
                }
            }
            Definition::ObjectTypeDefinition(d) => {
                if let Some(d_name) = d.name().map(|name| name.text()) {
                    if ty_name == d_name.as_str() {
                        return Some("OBJECT".to_owned());
                    }
                }
            }
            _ => {}
        }
    }
    None
}

// let x = d.name().map(|name| name.text()).filter(|d_name| d_name.to_string() == name).map(|| "OBJECT".to_owned());
// x
// compiler.definitions().iter().map(|d| match d {
//     Definition::ObjectTypeDefinition(d) => {
//         let x = d.name().map(|name| name.text()).filter(|d_name| d_name.to_string() == name).map(|| "OBJECT".to_owned());
//         x
//     },
//     _ => None
// }).collect().iter().first();
