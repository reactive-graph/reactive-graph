use crate::client::instances::entities::args::add_property::AddPropertyArgs;
use crate::client::instances::entities::args::create::CreateEntityInstanceArgs;
use crate::client::instances::entities::args::id::IdArgs;
use crate::client::instances::entities::args::id_and_component::IdAndComponentArgs;
use crate::client::instances::entities::args::id_and_property::IdAndPropertyArgs;
use crate::client::instances::entities::args::label::LabelArgs;
use crate::client::instances::entities::args::search::SearchEntityInstancesArgs;
use crate::client::instances::entities::args::set_property::SetPropertyArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum EntityInstancesCommands {
    /// List all entity instances.
    #[non_exhaustive]
    List(SearchEntityInstancesArgs),
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
    /// Adds a new property to an entity instance.
    #[non_exhaustive]
    AddProperty(AddPropertyArgs),
    /// Removes a property from an entity instance.
    #[non_exhaustive]
    RemoveProperty(IdAndPropertyArgs),
    /// Lists the components of an entity instance.
    #[non_exhaustive]
    ListComponents(IdArgs),
    /// Adds a component to an entity instance.
    #[non_exhaustive]
    AddComponent(IdAndComponentArgs),
    /// Removes a component from an entity instance.
    #[non_exhaustive]
    RemoveComponent(IdAndComponentArgs),
    /// Creates a new entity type.
    #[non_exhaustive]
    Create(CreateEntityInstanceArgs),
    // Deletes an entity instance.
    #[non_exhaustive]
    Delete(IdArgs),
}
