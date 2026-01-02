use crate::client::types::entities::args::add_extension::EntityTypeAddExtensionArgs;
use crate::client::types::entities::args::add_property::EntityTypeAddPropertyArgs;
use crate::client::types::entities::args::create::CreateEntityTypeArgs;
use crate::client::types::entities::args::entity_component_type::EntityComponentTypeIdArgs;
use crate::client::types::entities::args::entity_extension_type::EntityExtensionTypeIdArgs;
use crate::client::types::entities::args::entity_type_property::EntityTypePropertyArgs;
use crate::client::types::entities::args::parse_entity_ty;
use crate::client::types::entities::args::update_description::EntityTypeUpdateDescriptionArgs;
use clap::Subcommand;
use reactive_graph_graph::EntityTypeId;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum EntityTypesCommands {
    /// List all entity types.
    #[non_exhaustive]
    List,
    /// Prints a single entity type.
    #[non_exhaustive]
    Get {
        #[arg(name = "entity_type", value_parser = parse_entity_ty)]
        entity_ty: EntityTypeId,
    },
    /// List the properties of an entity type.
    #[non_exhaustive]
    ListProperties {
        #[arg(name = "entity_type", value_parser = parse_entity_ty)]
        entity_ty: EntityTypeId,
    },
    /// List the extensions of an entity type.
    #[non_exhaustive]
    ListExtensions {
        #[arg(name = "entity_type", value_parser = parse_entity_ty)]
        entity_ty: EntityTypeId,
    },
    /// List the components of an entity type.
    #[non_exhaustive]
    ListComponents {
        #[arg(name = "entity_type", value_parser = parse_entity_ty)]
        entity_ty: EntityTypeId,
    },
    /// Prints the JSON Schema of an entity type.
    #[non_exhaustive]
    GetJsonSchema {
        #[arg(name = "entity_type", value_parser = parse_entity_ty)]
        entity_ty: EntityTypeId,
    },
    /// Creates a new entity type.
    #[non_exhaustive]
    Create(CreateEntityTypeArgs),
    /// Deletes a entity type.
    #[non_exhaustive]
    Delete {
        #[arg(name = "entity_type", value_parser = parse_entity_ty)]
        entity_ty: EntityTypeId,
    },
    /// Adds a property to an entity type.
    #[non_exhaustive]
    AddProperty(EntityTypeAddPropertyArgs),
    /// Removes a property from an entity type.
    #[non_exhaustive]
    RemoveProperty(EntityTypePropertyArgs),
    /// Adds an extension to an entity type.
    #[non_exhaustive]
    AddExtension(EntityTypeAddExtensionArgs),
    /// Removes an extension from an entity type.
    #[non_exhaustive]
    RemoveExtension(EntityExtensionTypeIdArgs),
    /// Adds a component to an entity type.
    #[non_exhaustive]
    AddComponent(EntityComponentTypeIdArgs),
    /// Removes a component from an entity type.
    #[non_exhaustive]
    RemoveComponent(EntityComponentTypeIdArgs),
    /// Updates the description of an entity type.
    #[non_exhaustive]
    UpdateDescription(EntityTypeUpdateDescriptionArgs),
    /// Prints the JSON Schema of entity types.
    #[non_exhaustive]
    JsonSchema,
}
