use crate::client::types::relations::args::add_extension::RelationTypeAddExtensionArgs;
use crate::client::types::relations::args::add_property::RelationTypeAddPropertyArgs;
use crate::client::types::relations::args::create::CreateRelationTypeArgs;
use crate::client::types::relations::args::relation_component_type::RelationComponentTypeIdArgs;
use crate::client::types::relations::args::relation_extension_type::RelationExtensionTypeIdArgs;
use crate::client::types::relations::args::relation_type_property::RelationTypePropertyArgs;
use crate::client::types::relations::args::type_id::RelationTypeIdArgs;
use crate::client::types::relations::args::update_description::RelationTypeUpdateDescriptionArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum RelationTypesCommands {
    /// List all relation types.
    #[non_exhaustive]
    List,
    /// Prints a single relation type.
    #[non_exhaustive]
    Get(RelationTypeIdArgs),
    /// List the properties of an relation type.
    #[non_exhaustive]
    ListProperties(RelationTypeIdArgs),
    /// List the extensions of an relation type.
    #[non_exhaustive]
    ListExtensions(RelationTypeIdArgs),
    /// List the components of an relation type.
    #[non_exhaustive]
    ListComponents(RelationTypeIdArgs),
    /// Prints the JSON Schema of an relation type.
    #[non_exhaustive]
    GetJsonSchema(RelationTypeIdArgs),
    /// Creates a new relation type.
    #[non_exhaustive]
    Create(CreateRelationTypeArgs),
    /// Deletes a relation type.
    #[non_exhaustive]
    Delete(RelationTypeIdArgs),
    /// Adds a property to a relation type.
    #[non_exhaustive]
    AddProperty(RelationTypeAddPropertyArgs),
    /// Removes a property from a relation type.
    #[non_exhaustive]
    RemoveProperty(RelationTypePropertyArgs),
    /// Adds an extension to a relation type.
    #[non_exhaustive]
    AddExtension(RelationTypeAddExtensionArgs),
    /// Removes an extension from a relation type.
    #[non_exhaustive]
    RemoveExtension(RelationExtensionTypeIdArgs),
    /// Adds a component to a relation type.
    #[non_exhaustive]
    AddComponent(RelationComponentTypeIdArgs),
    /// Removes a component from a relation type.
    #[non_exhaustive]
    RemoveComponent(RelationComponentTypeIdArgs),
    /// Updates the description of a relation type.
    #[non_exhaustive]
    UpdateDescription(RelationTypeUpdateDescriptionArgs),
    /// Prints the JSON Schema of relation types.
    #[non_exhaustive]
    JsonSchema,
}
