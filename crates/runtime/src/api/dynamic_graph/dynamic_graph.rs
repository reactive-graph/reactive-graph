use apollo_compiler::ApolloCompiler;
use std::sync::Arc;

use crate::reactive::ReactiveEntity;
use apollo_compiler::values::DirectiveDefinition;
use apollo_compiler::values::EnumTypeDefinition;
use apollo_compiler::values::Field;
use apollo_compiler::values::FieldDefinition;
use apollo_compiler::values::FragmentDefinition;
use apollo_compiler::values::InputObjectTypeDefinition;
use apollo_compiler::values::ObjectTypeDefinition;
use apollo_compiler::values::OperationDefinition;
use apollo_compiler::values::ScalarTypeDefinition;
use apollo_compiler::values::SelectionSet;
use apollo_compiler::values::Type;
use apollo_compiler::values::UnionTypeDefinition;
use apollo_encoder::Document;
use async_graphql::indexmap::IndexMap;
use async_graphql::Name;
use async_graphql::Value;
use async_graphql_actix_web::GraphQLRequest;
use async_graphql_actix_web::GraphQLResponse;
use async_trait::async_trait;
use inexor_rgf_core_model::ReactiveRelation;

use crate::api::Lifecycle;

#[async_trait]
pub trait DynamicGraph: Send + Sync + Lifecycle {
    /// Returns true, if the type system has been modified
    fn is_type_system_modified(&self) -> bool;

    /// Creates the dynamic GraphQL schema.
    fn create_sdl(&self) -> Document;

    /// Regenerates the dynamic GraphQL schema.
    fn regenerate_schema(&self);

    /// Regenerates the dynamic GraphQL schema if and only if the type system has been modified.
    fn regenerate_schema_if_modified(&self);

    /// Executes a GraphQL Request.
    fn execute_request(&self, request: GraphQLRequest) -> GraphQLResponse;

    fn execute_operation(&self, compiler: &ApolloCompiler, operation: &OperationDefinition) -> Value;

    fn resolve_fields(&self, compiler: &ApolloCompiler, fields: Arc<Vec<Field>>) -> Value;

    fn resolve_fields_index_map(&self, compiler: &ApolloCompiler, fields: Arc<Vec<Field>>) -> IndexMap<Name, Value>;

    fn resolve_instances(&self, compiler: &ApolloCompiler, field: &Field) -> Value;

    fn resolve_entity_instances(&self, compiler: &ApolloCompiler, field: &Field) -> Vec<Value>;

    fn resolve_entity_instance(&self, compiler: &ApolloCompiler, field: &Field, entity_instance: &ReactiveEntity, in_component: bool) -> Value;

    fn resolve_entity_instance_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        entity_instance: &ReactiveEntity,
        in_component: bool,
    ) -> IndexMap<Name, Value>;

    fn resolve_relation_instances(&self, compiler: &ApolloCompiler, field: &Field) -> Vec<Value>;

    fn resolve_relation_instance(
        &self,
        compiler: &ApolloCompiler,
        field: &Field,
        relation_instance: &ReactiveRelation,
        in_component: bool,
    ) -> Value;

    fn resolve_relation_instance_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        relation_instance: &ReactiveRelation,
        in_component: bool,
    ) -> IndexMap<Name, Value>;

    fn resolve_schema(&self, compiler: &ApolloCompiler, field: &Field) -> Value;

    fn resolve_selection_set_index_map(&self, compiler: &ApolloCompiler, selection_set: &SelectionSet) -> IndexMap<Name, Value>;

    fn resolve_types(&self, compiler: &ApolloCompiler, field: &Field) -> Value;

    fn resolve_type(&self, compiler: &ApolloCompiler, ty: Type) -> Value;

    fn resolve_root_operation_type(&self, compiler: &ApolloCompiler, field: &Field) -> Value;

    fn resolve_scalar_type_definition(&self, compiler: &ApolloCompiler, field: &Field, scalar_type_definition: &ScalarTypeDefinition) -> Value;

    fn resolve_scalar_type_definition_index_map(&self, fields: Vec<Field>, scalar_type_definition: &ScalarTypeDefinition) -> IndexMap<Name, Value>;

    fn resolve_enum_type(&self, compiler: &ApolloCompiler, field: &Field, enum_type_definition: &EnumTypeDefinition) -> Value;

    fn resolve_enum_type_index_map(&self, fields: Vec<Field>, enum_type_definition: &EnumTypeDefinition) -> IndexMap<Name, Value>;

    fn resolve_enum_values(&self, enum_type: &EnumTypeDefinition) -> Value;

    fn resolve_union_type(&self, compiler: &ApolloCompiler, field: &Field, union_type_definition: &UnionTypeDefinition) -> Value;

    fn resolve_union_type_index_map(&self, compiler: &ApolloCompiler, fields: Vec<Field>, union_type_definition: &UnionTypeDefinition)
        -> IndexMap<Name, Value>;

    fn resolve_union_members(&self, compiler: &ApolloCompiler, union_type_definition: &UnionTypeDefinition) -> Value;

    fn resolve_object_type(&self, compiler: &ApolloCompiler, field: &Field, object_type_definition: &ObjectTypeDefinition) -> Value;

    fn resolve_object_type_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        object_type_definition: &ObjectTypeDefinition,
    ) -> IndexMap<Name, Value>;

    fn resolve_input_object_type_definition(&self, compiler: &ApolloCompiler, field: &Field, input_object_type_definition: &InputObjectTypeDefinition)
        -> Value;

    fn resolve_input_object_type_definition_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        input_object_type_definition: &InputObjectTypeDefinition,
    ) -> IndexMap<Name, Value>;

    fn resolve_fragment_definition(&self, compiler: &ApolloCompiler, field: &Field, fragment_definition: &FragmentDefinition) -> Value;

    fn resolve_fragment_definition_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        fragment_definition: &FragmentDefinition,
    ) -> IndexMap<Name, Value>;

    fn resolve_directives(&self, compiler: &ApolloCompiler, field: &Field) -> Value;

    fn resolve_directive_definition(&self, compiler: &ApolloCompiler, field: &Field, directive_definition: &DirectiveDefinition) -> Value;

    fn resolve_directive_definition_index_map(
        &self,
        compiler: &ApolloCompiler,
        fields: Vec<Field>,
        directive_definition: &DirectiveDefinition,
    ) -> IndexMap<Name, Value>;

    fn resolve_field(&self, compiler: &ApolloCompiler, field: &FieldDefinition, fields: Arc<Vec<Field>>) -> Value;

    // TODO: compiler: compile schema and query separately
}
