use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaBuilder;
use async_graphql::dynamic::SchemaError;
use async_graphql::dynamic::Type;
use async_trait::async_trait;
use log::trace;
use reactive_graph_dynamic_graph_api::DynamicGraphSchemaBuilder;
use reactive_graph_dynamic_graph_api::InterfaceManager;
use reactive_graph_dynamic_graph_api::MutationNamespaceObjectTreeFactory;
use reactive_graph_dynamic_graph_api::MutationObjectManager;
use reactive_graph_dynamic_graph_api::QueryNamespaceObjectTreeFactory;
use reactive_graph_dynamic_graph_api::QueryObjectManager;
use reactive_graph_dynamic_graph_api::ScalarManager;
use reactive_graph_dynamic_graph_api::SchemaBuilderManager;
use reactive_graph_dynamic_graph_api::UnionManager;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::Component;
use springtime_di::component_alias;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Component)]
pub struct DynamicGraphSchemaBuilderImpl {
    interface_manager: Arc<dyn InterfaceManager + Send + Sync>,
    query_object_manager: Arc<dyn QueryObjectManager + Send + Sync>,
    query_namespace_object_tree_factory: Arc<dyn QueryNamespaceObjectTreeFactory + Send + Sync>,
    mutation_object_manager: Arc<dyn MutationObjectManager + Send + Sync>,
    mutation_namespace_object_tree_factory: Arc<dyn MutationNamespaceObjectTreeFactory + Send + Sync>,
    scalar_manager: Arc<dyn ScalarManager + Send + Sync>,
    schema_builder_manager: Arc<dyn SchemaBuilderManager + Send + Sync>,
    union_manager: Arc<dyn UnionManager + Send + Sync>,
}

impl DynamicGraphSchemaBuilderImpl {}

#[async_trait]
#[component_alias]
impl DynamicGraphSchemaBuilder for DynamicGraphSchemaBuilderImpl {
    fn build_dynamic_schema(&self) -> Result<Schema, SchemaError> {
        let mut schema = self.schema_builder_manager.get_schema_builder();
        schema = register_all(schema, self.scalar_manager.get_scalars());
        schema = register_all(schema, self.interface_manager.get_interfaces());
        // query
        schema = register_all(schema, self.query_object_manager.get_query_objects());
        schema = register_all(schema, self.query_namespace_object_tree_factory.get_namespace_objects());
        // schema = schema.register(self.query_namespace_object_tree_factory.create_root_object());
        // mutation
        schema = register_all(schema, self.mutation_object_manager.get_mutation_objects());
        schema = register_all(schema, self.mutation_namespace_object_tree_factory.get_namespace_objects());
        // schema = schema.register(self.mutation_namespace_object_tree_factory.create_root_object());
        schema = register_all(schema, self.union_manager.get_unions());
        schema.finish()
    }
}

#[async_trait]
impl Lifecycle for DynamicGraphSchemaBuilderImpl {}

fn register_all(mut schema: SchemaBuilder, tys: Vec<impl Into<Type> + Debug>) -> SchemaBuilder {
    for ty in tys {
        let ty = ty.into();
        trace!("Registering dynamic graph type {:?}", ty);
        schema = schema.register(ty);
    }
    schema
}
