use std::sync::Arc;

use crate::client::error::CommandError::MissingSubCommand;
use crate::client::error::CommandError::NamespacedTypeNotFound;
use crate::client::error::CommandError::NoContent;
use crate::client::error::CommandError::NotCreated;
use crate::client::error::CommandError::NotFound;
use crate::client::error::CommandError::NotRemoved;
use crate::client::result::CommandResult;
use crate::client::types::components::output_format::ComponentsOutputFormatWrapper;
use crate::client::types::entities::args::EntityTypesArgs;
use crate::client::types::entities::commands::EntityTypesCommands;
use crate::client::types::entities::output_format::EntityTypesOutputFormatWrapper;
use crate::client::types::extension::output_format::ExtensionsOutputFormatWrapper;
use crate::client::types::property_type::output_format::PropertyTypesOutputFormatWrapper;
use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeGetter;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod output_format;

pub(crate) async fn entity_types(client: &Arc<ReactiveGraphClient>, entity_type_args: EntityTypesArgs) -> CommandResult {
    let output_format_wrapper: EntityTypesOutputFormatWrapper = entity_type_args.output_format.clone().into();
    let Some(command) = entity_type_args.commands else {
        return Err(MissingSubCommand);
    };
    match command {
        EntityTypesCommands::List => match client.types().entities().get_all_entity_types().await {
            Ok(Some(entity_types)) => output_format_wrapper.collection(entity_types.to_vec()),
            Ok(None) => Err(NoContent("No entity types found".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Get { entity_ty } => match client.types().entities().get_entity_type_by_type(&entity_ty).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NamespacedTypeNotFound(entity_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::ListProperties { entity_ty } => match client.types().entities().get_entity_type_by_type(&entity_ty).await {
            Ok(Some(entity_type)) => {
                let output_format_wrapper: PropertyTypesOutputFormatWrapper = entity_type_args.output_format.into();
                output_format_wrapper.collection(entity_type.properties.to_vec())
            }
            Ok(None) => Err(NamespacedTypeNotFound(entity_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::ListExtensions { entity_ty } => match client.types().entities().get_entity_type_by_type(&entity_ty).await {
            Ok(Some(entity_type)) => {
                let output_format_wrapper: ExtensionsOutputFormatWrapper = entity_type_args.output_format.into();
                output_format_wrapper.collection(entity_type.extensions.to_vec())
            }
            Ok(None) => Err(NamespacedTypeNotFound(entity_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::ListComponents { entity_ty } => match client.types().entities().get_entity_type_components(&entity_ty).await {
            Ok(Some(components)) => {
                let output_format_wrapper: ComponentsOutputFormatWrapper = entity_type_args.output_format.into();
                output_format_wrapper.collection(components.to_vec())
            }
            Ok(None) => Err(NamespacedTypeNotFound(entity_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::GetJsonSchema { entity_ty } => match client.types().entities().json_schema_for_entity_type_by_type(&entity_ty).await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NamespacedTypeNotFound(entity_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Create(args) => match client
            .types()
            .entities()
            .create_entity_type(
                EntityType::builder()
                    .ty(&args.entity_ty)
                    .description(args.description.unwrap_or_default())
                    .build(),
            )
            .await
        {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Entity type wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Delete { entity_ty } => match client.types().entities().delete_entity_type(&entity_ty).await {
            Ok(Some(true)) => Ok(format!("Entity Type {} deleted", entity_ty).into()),
            Ok(Some(false)) => Ok(format!("Entity Type {} not deleted", entity_ty).into()),
            Ok(None) => Err(NamespacedTypeNotFound(entity_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::AddProperty(args) => match client.types().entities().add_property(&args.entity_ty, &args.property_type).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Property wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::RemoveProperty(args) => match client.types().entities().remove_property(&args.entity_ty, args.property_name.clone()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NamespacedTypeNotFound(args.entity_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::AddExtension(args) => match client.types().entities().add_extension(&args.entity_ty, &args.extension).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Extension wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::RemoveExtension(args) => match client.types().entities().remove_extension(&args).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NamespacedTypeNotFound(args.entity_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::AddComponent(args) => match client.types().entities().add_component(&args).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Component wasn't added to entity type".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::RemoveComponent(args) => match client.types().entities().remove_component(&args).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotRemoved("Component wasn't added to entity type".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::UpdateDescription(args) => match client.types().entities().update_description(&args.entity_ty, args.description).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NamespacedTypeNotFound(args.entity_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::JsonSchema => match client.json_schema().types().entities().await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NotFound("JSON Schema not available".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}
