// use std::sync::Arc;
// use std::sync::RwLock;
// use std::time;
//
// use serde_json::json;
// use tokio::task;
//
// use inexor_rgf_graph::PropertyInstanceGetter;
// use inexor_rgf_command_model::builder::CommandBuilder;
// use inexor_rgf_command_model::entity::Command;
// use inexor_rgf_command_model::entity::CommandArg;
// use inexor_rgf_command_model::error::CommandBuilderError;
// use inexor_rgf_runtime_model::ShutdownProperties::DELAY;
// use inexor_rgf_runtime_model::ENTITY_TYPE_SHUTDOWN;
//
// use crate::api::RemotesManager;
// use crate::api::UUID_SHUTDOWN;
// use crate::api::UUID_SHUTDOWN_TRIGGER;
//
// pub(crate) fn get_remotes_command(remotes_manager: Arc<dyn RemotesManager>) -> Result<Command, CommandBuilderError> {
//     let remotes_manager_1 = remotes_manager.clone();
//     CommandBuilder::new()
//         .singleton(&ENTITY_TYPE_SHUTDOWN.clone())
//         .help("Get the remotes")
//         .arguments()
//         .argument(
//             CommandArg::new(DELAY)
//                 .short('d')
//                 .long("delay")
//                 .help("Delay shutdown by N seconds")
//                 .required(false),
//             json!(0),
//         )
//         .no_properties()
//         .executor_with_handle(move |e| remotes_manager_1.get_all(), Some(UUID_SHUTDOWN_TRIGGER.as_u128()))
//         .id(UUID_SHUTDOWN)
//         .build()
// }
