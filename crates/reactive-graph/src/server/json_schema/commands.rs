use crate::server::json_schema::args::InstancesArguments;
use crate::server::json_schema::args::TypesArguments;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum JsonSchemaCommands {
    /// Prints the JSON schema of the type system.
    Types(TypesArguments),
    /// Prints the JSON schema of the instance system.
    Instances(InstancesArguments),
}

#[derive(Subcommand, Debug)]
pub enum TypesCommands {
    /// Prints the JSON schema of the component types.
    Components,
    /// Prints the JSON schema of the entity types.
    Entities,
    /// Prints the JSON schema of the relation types.
    Relations,
    /// Prints the JSON schema of the flow types.
    Flows,
}

#[derive(Subcommand, Debug)]
pub enum InstancesCommands {
    /// Prints the JSON schema of the entity instances.
    Entities,
    /// Prints the JSON schema of the relation instances.
    Relations,
    /// Prints the JSON schema of the flow instances.
    Flows,
}
