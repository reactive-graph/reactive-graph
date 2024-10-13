use std::sync::Arc;

use crate::client::commands::ClientCommands;
use crate::client::instances::entities::entity_instances;
use crate::client::instances::relations::relation_instances;
use crate::client::result::CommandResult;
use crate::client::system::command::execute_command;
use crate::client::system::instance::instance_info;
use crate::client::system::plugin::plugins;
use crate::client::system::remotes::remotes;
use crate::client::system::shutdown::shutdown;
use crate::client::types::components::components;
use crate::client::types::entities::entity_types;
use crate::client::types::relations::relation_types;
use reactive_graph_client::ReactiveGraphClient;

pub(crate) async fn handle_command(client: &Arc<ReactiveGraphClient>, command: ClientCommands) -> CommandResult {
    match command {
        // System
        ClientCommands::ExecuteCommand(args) => execute_command(client, args).await,
        ClientCommands::InstanceInfo(args) => instance_info(client, args).await,
        ClientCommands::Plugins(args) => plugins(client, args).await,
        ClientCommands::Remotes(args) => remotes(client, args).await,
        ClientCommands::Shutdown => shutdown(client).await,
        // Types
        ClientCommands::Components(args) => components(client, args).await,
        ClientCommands::EntityTypes(args) => entity_types(client, args).await,
        ClientCommands::RelationTypes(args) => relation_types(client, args).await,
        // TODO: Flow Types
        ClientCommands::EntityInstances(args) => entity_instances(client, args).await,
        ClientCommands::RelationInstances(args) => relation_instances(client, args).await,
        // TODO: Flow Instances
    }
}
