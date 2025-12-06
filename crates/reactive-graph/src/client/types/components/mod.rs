use std::sync::Arc;

use crate::client::error::CommandError;
use crate::client::error::CommandError::NoContent;
use crate::client::error::CommandError::NotCreated;
use crate::client::error::CommandError::NotFound;
use crate::client::result::CommandResult;
use crate::client::types::components::args::ComponentsArgs;
use crate::client::types::components::commands::ComponentsCommands;
use crate::client::types::components::output_format::ComponentsOutputFormatWrapper;
use crate::client::types::extension::output_format::ExtensionsOutputFormatWrapper;
use crate::client::types::property_type::output_format::PropertyTypesOutputFormatWrapper;
use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_graph::Component;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeGetter;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod output_format;

pub(crate) async fn components(client: &Arc<ReactiveGraphClient>, component_args: ComponentsArgs) -> CommandResult {
    let output_format_wrapper: ComponentsOutputFormatWrapper = component_args.output_format.clone().into();
    let Some(command) = component_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        ComponentsCommands::List => match client.types().components().get_all_components().await {
            Ok(Some(components)) => output_format_wrapper.collection(components.to_vec()),
            Ok(None) => Err(NoContent("No components found".to_string())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::Get { component_ty } => match client.types().components().get_component_by_type(&component_ty).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(component_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::ListProperties { component_ty } => match client.types().components().get_component_by_type(&component_ty).await {
            Ok(Some(component)) => {
                let output_format_wrapper: PropertyTypesOutputFormatWrapper = component_args.output_format.into();
                output_format_wrapper.collection(component.properties.to_vec())
            }
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(component_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::ListExtensions { component_ty } => match client.types().components().get_component_by_type(&component_ty).await {
            Ok(Some(component)) => {
                let output_format_wrapper: ExtensionsOutputFormatWrapper = component_args.output_format.into();
                output_format_wrapper.collection(component.extensions.to_vec())
            }
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(component_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::GetJsonSchema { component_ty } => match client.types().components().json_schema_for_component_by_type(&component_ty).await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(component_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::Create(args) => match client
            .types()
            .components()
            .create_component(
                Component::builder()
                    .ty(args.component_ty)
                    .description(args.description.unwrap_or_default())
                    .build(),
            )
            .await
        {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(NotCreated("Component wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::Delete { component_ty } => match client.types().components().delete_component(&component_ty).await {
            Ok(Some(true)) => Ok(format!("Component {} deleted", component_ty).into()),
            Ok(Some(false)) => Ok(format!("Component {} not deleted", component_ty).into()),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(component_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::AddProperty(args) => match client.types().components().add_property(args.component_ty, &args.property_type).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(NotCreated("Property wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::RemoveProperty(args) => match client
            .types()
            .components()
            .remove_property(&args.component_ty, args.property_name.clone())
            .await
        {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(NotFound(format!("Component {} not found", args.component_ty))),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::AddExtension(args) => match client.types().components().add_extension(args.component_ty, &args.extension).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(NotCreated("Extension wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::RemoveExtension(args) => match client.types().components().remove_extension(&args).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(args.component_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::UpdateDescription(args) => match client.types().components().update_description(&args.component_ty, args.description).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(args.component_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::JsonSchema => match client.json_schema().types().components().await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NotFound("JSON Schema not available".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}
