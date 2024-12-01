use crate::client::instances::relations::args::add_property::AddPropertyArgs;
use crate::client::instances::relations::args::create::CreateRelationInstanceArgs;
use crate::client::instances::relations::args::id::RelationInstanceIdArgs;
use crate::client::instances::relations::args::id_and_component::RelationInstanceIdAndComponentArgs;
use crate::client::instances::relations::args::id_and_property::RelationInstanceIdAndPropertyArgs;
use crate::client::instances::relations::args::search::SearchRelationInstancesArgs;
use crate::client::instances::relations::args::set_property::SetPropertyArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum RelationInstancesCommands {
    /// List all relation instances.
    #[non_exhaustive]
    List(SearchRelationInstancesArgs),
    /// Prints a single relation instance.
    #[non_exhaustive]
    Get(RelationInstanceIdArgs),
    /// Lists the properties of a relation instance.
    #[non_exhaustive]
    ListProperties(RelationInstanceIdArgs),
    /// Prints the value of a property of a relation instance.
    #[non_exhaustive]
    GetProperty(RelationInstanceIdAndPropertyArgs),
    /// Sets the value of a property of a relation instance.
    #[non_exhaustive]
    SetProperty(SetPropertyArgs),
    /// Adds a new property to a relation instance.
    #[non_exhaustive]
    AddProperty(AddPropertyArgs),
    /// Removes a property from a relation instance.
    #[non_exhaustive]
    RemoveProperty(RelationInstanceIdAndPropertyArgs),
    /// Lists the components of a relation instance.
    #[non_exhaustive]
    ListComponents(RelationInstanceIdArgs),
    /// Adds a component to a relation instance.
    #[non_exhaustive]
    AddComponent(RelationInstanceIdAndComponentArgs),
    /// Removes a component from a relation instance.
    #[non_exhaustive]
    RemoveComponent(RelationInstanceIdAndComponentArgs),
    /// Creates a new relation type.
    #[non_exhaustive]
    Create(CreateRelationInstanceArgs),
    // Deletes a relation instance.
    #[non_exhaustive]
    Delete(RelationInstanceIdArgs),
    /// Prints the JSON Schema of relation instances.
    #[non_exhaustive]
    JsonSchema,
}
