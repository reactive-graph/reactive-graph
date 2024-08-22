use crate::cli::instances::entities::args::id::IdArgs;
use crate::cli::instances::entities::args::label::LabelArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum EntityInstancesCommands {
    /// List all entity instances.
    #[non_exhaustive]
    List,
    /// Prints a single entity instance.
    #[non_exhaustive]
    Get(IdArgs),
    /// Prints a single entity instance.
    #[non_exhaustive]
    GetByLabel(LabelArgs),
    // /// List the properties of an entity type.
    // #[non_exhaustive]
    // ListProperties(EntityTypeIdArgs),
    // /// List the extensions of an entity type.
    // #[non_exhaustive]
    // ListExtensions(EntityTypeIdArgs),
    // /// List the components of an entity type.
    // #[non_exhaustive]
    // ListComponents(EntityTypeIdArgs),
    // /// Creates a new entity type.
    // #[non_exhaustive]
    // Create(CreateEntityTypeArgs),
    /// Deletes an entity instance.
    #[non_exhaustive]
    Delete(IdArgs),
    // /// Adds a property to an entity type.
    // #[non_exhaustive]
    // AddProperty(EntityTypeAddPropertyArgs),
    // /// Removes a property from an entity type.
    // #[non_exhaustive]
    // RemoveProperty(EntityTypePropertyArgs),
    // /// Adds an extension to an entity type.
    // #[non_exhaustive]
    // AddExtension(EntityTypeAddExtensionArgs),
    // /// Removes an extension from an entity type.
    // #[non_exhaustive]
    // RemoveExtension(EntityExtensionTypeIdArgs),
    // /// Adds a component to an entity type.
    // #[non_exhaustive]
    // AddComponent(EntityComponentTypeIdArgs),
    // /// Removes a component from an entity type.
    // #[non_exhaustive]
    // RemoveComponent(EntityComponentTypeIdArgs),
    // /// Updates the description of an entity type.
    // #[non_exhaustive]
    // UpdateDescription(EntityTypeUpdateDescriptionArgs),
}
