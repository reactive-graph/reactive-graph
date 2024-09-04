// use crate::cli::instances::relations::args::add_property::AddPropertyArgs;
// use crate::cli::instances::relations::args::create::CreateRelationInstanceArgs;
// use crate::cli::instances::relations::args::id::IdArgs;
// use crate::cli::instances::relations::args::id_and_component::IdAndComponentArgs;
// use crate::cli::instances::relations::args::id_and_property::IdAndPropertyArgs;
// use crate::cli::instances::relations::args::label::LabelArgs;
use crate::cli::instances::relations::args::search::SearchRelationInstancesArgs;
// use crate::cli::instances::relations::args::set_property::SetPropertyArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum RelationInstancesCommands {
    /// List all relation instances.
    #[non_exhaustive]
    List(SearchRelationInstancesArgs),
    // /// Prints a single relation instance.
    // #[non_exhaustive]
    // Get(IdArgs),
    // /// Prints a single relation instance.
    // #[non_exhaustive]
    // GetByLabel(LabelArgs),
    // /// Lists the properties of a relation instance.
    // #[non_exhaustive]
    // ListProperties(IdArgs),
    // /// Prints the value of a property of a relation instance.
    // #[non_exhaustive]
    // GetProperty(IdAndPropertyArgs),
    // /// Sets the value of a property of a relation instance.
    // #[non_exhaustive]
    // SetProperty(SetPropertyArgs),
    // /// Adds a new property to a relation instance.
    // #[non_exhaustive]
    // AddProperty(AddPropertyArgs),
    // /// Removes a property from a relation instance.
    // #[non_exhaustive]
    // RemoveProperty(IdAndPropertyArgs),
    // /// Lists the components of a relation instance.
    // #[non_exhaustive]
    // ListComponents(IdArgs),
    // /// Adds a component to a relation instance.
    // #[non_exhaustive]
    // AddComponent(IdAndComponentArgs),
    // /// Removes a component from a relation instance.
    // #[non_exhaustive]
    // RemoveComponent(IdAndComponentArgs),
    // /// Creates a new relation type.
    // #[non_exhaustive]
    // Create(CreateRelationInstanceArgs),
    // // Deletes a relation instance.
    // #[non_exhaustive]
    // Delete(IdArgs),
}
