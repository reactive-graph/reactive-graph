use crate::cli::instances::entities::args::create::CreateEntityInstanceArgs;
use crate::cli::instances::entities::args::id::IdArgs;
use crate::cli::instances::entities::args::id_and_property::IdAndPropertyArgs;
use crate::cli::instances::entities::args::label::LabelArgs;
use crate::cli::instances::entities::args::set_property::SetPropertyArgs;
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
    /// Lists the properties of an entity instance.
    #[non_exhaustive]
    ListProperties(IdArgs),
    /// Prints the value of a property of an entity instance.
    #[non_exhaustive]
    GetProperty(IdAndPropertyArgs),
    /// Sets the value of a property of an entity instance.
    #[non_exhaustive]
    SetProperty(SetPropertyArgs),
    /// Creates a new entity type.
    #[non_exhaustive]
    Create(CreateEntityInstanceArgs),
    // Deletes an entity instance.
    #[non_exhaustive]
    Delete(IdArgs),
}
