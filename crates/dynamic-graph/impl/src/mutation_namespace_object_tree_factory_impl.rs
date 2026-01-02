use crate::field::namespace_field;
use crate::field::namespace_field_name;
use crate::field::namespace_path_field;
use crate::field::root_object_namespace_path_field;
use crate::object_type_name::namespace_type_name;
use async_graphql::dynamic::Object;
use async_trait::async_trait;
use convert_case::Case::Pascal;
use convert_case::Casing;
use log::trace;
use log::warn;
use reactive_graph_dynamic_graph_api::ComponentMutationFieldFactory;
use reactive_graph_dynamic_graph_api::EntityMutationFieldFactory;
use reactive_graph_dynamic_graph_api::FlowMutationFieldFactory;
use reactive_graph_dynamic_graph_api::MutationNamespaceObjectTreeFactory;
use reactive_graph_dynamic_graph_api::ROOT_OBJECT_NAME_MUTATION;
use reactive_graph_dynamic_graph_api::RelationMutationFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType::Mutation;
use reactive_graph_graph::Namespace;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::NamespacedTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Component)]
pub struct MutationNamespaceObjectTreeFactoryImpl {
    namespaced_type_manager: Arc<dyn NamespacedTypeManager + Send + Sync>,
    component_mutation_field_factory: Arc<dyn ComponentMutationFieldFactory + Send + Sync>,
    entity_mutation_field_factory: Arc<dyn EntityMutationFieldFactory + Send + Sync>,
    flow_mutation_field_factory: Arc<dyn FlowMutationFieldFactory + Send + Sync>,
    relation_mutation_field_factory: Arc<dyn RelationMutationFieldFactory + Send + Sync>,
}

impl MutationNamespaceObjectTreeFactoryImpl {}

#[async_trait]
#[component_alias]
impl MutationNamespaceObjectTreeFactory for MutationNamespaceObjectTreeFactoryImpl {
    fn get_namespace_objects(&self) -> Vec<Object> {
        let namespaced_types = self.namespaced_type_manager.get_all();
        let namespaces = namespaced_types.get_all_parent_paths_recursively();
        let mut root_object = self.create_root_object();
        let mut object_tree = BTreeMap::<Namespace, Object>::new();
        for namespace in namespaces {
            // Create object
            let mut object = self.create_namespace_object(&namespace);
            // Create fields
            for field in self.component_mutation_field_factory.create_mutation_fields(&namespace) {
                object = object.field(field);
            }
            for field in self.entity_mutation_field_factory.create_mutation_fields(&namespace) {
                object = object.field(field);
            }
            for field in self.relation_mutation_field_factory.get_mutation_fields(&namespace) {
                object = object.field(field);
            }
            for field in self.flow_mutation_field_factory.create_mutation_fields(&namespace) {
                object = object.field(field);
            }
            object_tree.insert(namespace.clone(), object);
            // Create field in parent object or root object if there is no parent object
            let field = namespace_field(namespace.clone(), Mutation);
            let namespace_field_name = namespace_field_name(&namespace);
            match namespace.parent() {
                None => {
                    // Create field in root object
                    root_object = root_object.field(field);
                    trace!("Create field {namespace_field_name} for Namespace({namespace}) in root object {ROOT_OBJECT_NAME_MUTATION}");
                }
                Some(parent_namespace) => {
                    // Create field in parent namespace
                    match object_tree.remove(&parent_namespace) {
                        None => {
                            // Should never happen because of the BTreeMap
                            warn!("Missing object for parent Namespace({parent_namespace})");
                        }
                        Some(parent_object) => {
                            let parent_object = parent_object.field(field);
                            object_tree.insert(parent_namespace.clone(), parent_object);
                            trace!("Created field {namespace_field_name} for Namespace({namespace}) in parent Namespace({parent_namespace})");
                        }
                    }
                }
            }
        }
        let mut objects = vec![root_object];
        for (_, object) in object_tree.into_iter() {
            objects.push(object);
        }
        objects
    }

    fn create_namespace_object(&self, namespace: &Namespace) -> Object {
        let object_type_name = namespace_type_name(namespace, Mutation);
        trace!("Create namespace {Mutation} object {object_type_name} for Namespace({namespace})");
        Object::new(namespace_type_name(namespace, Mutation))
            .description(format!(
                "Mutations for components, entities and relations on the namespace {}",
                &namespace.to_string().to_case(Pascal)
            ))
            .field(namespace_path_field(namespace.clone()))
    }

    fn create_root_object(&self) -> Object {
        trace!("Create root object {ROOT_OBJECT_NAME_MUTATION}");
        Object::new(ROOT_OBJECT_NAME_MUTATION)
            .description("Mutations")
            .field(root_object_namespace_path_field())
    }
}

#[async_trait]
impl Lifecycle for MutationNamespaceObjectTreeFactoryImpl {}
