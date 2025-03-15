use std::sync::Arc;

use crate::client::error::CommandError;
use crate::client::error::CommandError::NoContent;
use crate::client::error::CommandError::NotCreated;
use crate::client::error::CommandError::NotFound;
use crate::client::result::CommandResult;
use crate::client::types::components::output_format::ComponentsOutputFormatWrapper;
use crate::client::types::entities::args::EntityTypesArgs;
use crate::client::types::entities::commands::EntityTypesCommands;
use crate::client::types::entities::output_format::EntityTypesOutputFormatWrapper;
use crate::client::types::extension::output_format::ExtensionsOutputFormatWrapper;
use crate::client::types::property_type::output_format::PropertyTypesOutputFormatWrapper;
use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_graph::NamespacedTypeContainer;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod output_format;

pub(crate) async fn entity_types(client: &Arc<ReactiveGraphClient>, entity_type_args: EntityTypesArgs) -> CommandResult {
    let output_format_wrapper: EntityTypesOutputFormatWrapper = entity_type_args.output_format.clone().into();
    let Some(command) = entity_type_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        EntityTypesCommands::List => match client.types().entities().get_all_entity_types().await {
            Ok(Some(entity_types)) => output_format_wrapper.collection(entity_types),
            Ok(None) => Err(NoContent("No entity types found".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Get(args) => match client.types().entities().get_entity_type_by_type(args.clone()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::ListProperties(args) => match client.types().entities().get_entity_type_by_type(args.clone()).await {
            Ok(Some(entity_type)) => {
                let output_format_wrapper: PropertyTypesOutputFormatWrapper = entity_type_args.output_format.into();
                output_format_wrapper.collection(entity_type.properties.to_vec())
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::ListExtensions(args) => match client.types().entities().get_entity_type_by_type(args.clone()).await {
            Ok(Some(entity_type)) => {
                let output_format_wrapper: ExtensionsOutputFormatWrapper = entity_type_args.output_format.into();
                output_format_wrapper.collection(entity_type.extensions.to_vec())
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::ListComponents(args) => match client.types().entities().get_entity_type_components(args.clone()).await {
            Ok(Some(components)) => {
                let output_format_wrapper: ComponentsOutputFormatWrapper = entity_type_args.output_format.into();
                output_format_wrapper.collection(components.to_vec())
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Create(args) => match client.types().entities().create_entity_type_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Entity type wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Delete(args) => match client.types().entities().delete_entity_type_with_variables((&args).into()).await {
            Ok(Some(true)) => Ok(format!("Entity type {}__{} deleted", args.namespace, args.name).into()),
            Ok(Some(false)) => Ok(format!("Entity type {}__{} not deleted", args.namespace, args.name).into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::AddProperty(args) => match client.types().entities().add_property_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Property wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::RemoveProperty(args) => match client.types().entities().remove_property_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotFound(format!("Entity type {}__{} not found", args.ty.namespace, args.ty.name))),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::AddExtension(args) => match client.types().entities().add_extension_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Extension wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::RemoveExtension(args) => match client.types().entities().remove_extension_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(args.entity_ty.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::AddComponent(args) => match client.types().entities().add_component((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Component wasn't added to entity type".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::RemoveComponent(args) => match client.types().entities().remove_component((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(args.ty.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::UpdateDescription(args) => match client.types().entities().update_description_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(args.ty.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::JsonSchema => match client.json_schema().types().entities().await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NotFound("JSON Schema not available".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}
