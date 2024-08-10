use std::sync::Arc;

use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NoContent;
use crate::cli::error::CommandError::NotCreated;
use crate::cli::error::CommandError::NotFound;
use crate::cli::result::CommandResult;
use crate::cli::types::components::args::ComponentsArgs;
use crate::cli::types::components::commands::ComponentsCommands;
use crate::table_model::types::component::Components;
use reactive_graph_client::types::components::add_property::queries::AddPropertyVariables;
use reactive_graph_client::types::components::create_component::queries::CreateComponentVariables;
use reactive_graph_client::InexorRgfClient;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn components(client: &Arc<InexorRgfClient>, component_args: ComponentsArgs) -> CommandResult {
    let Some(command) = component_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        ComponentsCommands::List => match client.types().components().get_all_components().await {
            Ok(Some(components)) => Ok(Components::from(components).into()),
            Ok(None) => Err(NoContent("No components found".to_string())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::Get(args) => match client.types().components().get_component_by_type(args.clone()).await {
            Ok(Some(component)) => Ok(Components::from(component).into()),
            Ok(None) => Err(NotFound(format!("Component {}__{} not found", args.namespace, args.name))),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::Create(args) => {
            let variables = CreateComponentVariables::builder()
                .namespace(args.ty.namespace)
                .name(args.ty.name)
                .description(args.description)
                // .properties(None)
                // .extensions(None)
                .build();
            match client.types().components().create_component_with_variables(variables).await {
                Ok(Some(component)) => Ok(Components::from(component).into()),
                Ok(None) => Err(NotCreated("Component wasn't created".to_string())),
                Err(e) => Err(e.into()),
            }
        }
        ComponentsCommands::Delete(args) => match client.types().components().delete_component_by_type(args.clone()).await {
            Ok(Some(true)) => Ok(format!("Component {}__{} deleted", args.namespace, args.name).into()),
            Ok(Some(false)) => Ok(format!("Component {}__{} not deleted", args.namespace, args.name).into()),
            Ok(None) => Err(NotFound(format!("Component {}__{} not found", args.namespace, args.name))),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::AddProperty(args) => {
            // let extensions = args.property_type.extensions.into();
            let variables = AddPropertyVariables::builder()
                .namespace(args.ty.namespace)
                .name(args.ty.name)
                .property(
                    reactive_graph_client::schema_graphql::types::property_type::PropertyTypeDefinition::builder()
                        .name(args.property_type.property_name)
                        .description(args.property_type.description.unwrap_or_default())
                        .data_type(args.property_type.data_type.into())
                        .socket_type(args.property_type.socket_type.into())
                        .mutability(args.property_type.mutability.into())
                        // TODO
                        .extensions(Vec::new())
                        // .extensions(args.property_type.extensions)
                        .build(),
                )
                .build();
            match client.types().components().add_property_with_variables(variables).await {
                Ok(Some(component)) => Ok(Components::from(component).into()),
                Ok(None) => Err(NotCreated("Property wasn't created".to_string())),
                // Ok(Some(component)) => Ok(component.into()),
                // Ok(None) => Err(NotFound(format!("Component {}__{} not found", args.ty.namespace, args.ty.name))),
                Err(e) => Err(e.into()),
            }
        }
    }
}
