use crate::client::error::CommandError;
use crate::client::error::CommandError::NoContent;
use crate::client::error::CommandError::NotCreated;
use crate::client::error::CommandError::NotFound;
use crate::client::result::CommandResult;
use crate::client::types::components::output_format::ComponentsOutputFormatWrapper;
use crate::client::types::extension::output_format::ExtensionsOutputFormatWrapper;
use crate::client::types::property_type::output_format::PropertyTypesOutputFormatWrapper;
use crate::client::types::relations::args::RelationTypesArgs;
use crate::client::types::relations::commands::RelationTypesCommands;
use crate::client::types::relations::output_format::RelationTypesOutputFormatWrapper;
use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_graph::NamespacedTypeContainer;
use std::sync::Arc;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod output_format;

pub(crate) async fn relation_types(client: &Arc<ReactiveGraphClient>, relation_type_args: RelationTypesArgs) -> CommandResult {
    let output_format_wrapper: RelationTypesOutputFormatWrapper = relation_type_args.output_format.clone().into();
    let Some(command) = relation_type_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        RelationTypesCommands::List => match client.types().relations().get_all_relation_types().await {
            Ok(Some(relation_types)) => output_format_wrapper.collection(relation_types),
            Ok(None) => Err(NoContent("No relation types found".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::Get(args) => match client.types().relations().get_relation_type_by_type(args.clone()).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::ListProperties(args) => match client.types().relations().get_relation_type_by_type(args.clone()).await {
            Ok(Some(relation_type)) => {
                let output_format_wrapper: PropertyTypesOutputFormatWrapper = relation_type_args.output_format.into();
                output_format_wrapper.collection(relation_type.properties.to_vec())
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::ListExtensions(args) => match client.types().relations().get_relation_type_by_type(args.clone()).await {
            Ok(Some(relation_type)) => {
                let output_format_wrapper: ExtensionsOutputFormatWrapper = relation_type_args.output_format.into();
                output_format_wrapper.collection(relation_type.extensions.to_vec())
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::ListComponents(args) => match client.types().relations().get_relation_type_components(args.clone()).await {
            Ok(Some(components)) => {
                let output_format_wrapper: ComponentsOutputFormatWrapper = relation_type_args.output_format.into();
                output_format_wrapper.collection(components.to_vec())
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::Create(args) => match client.types().relations().create_relation_type_with_variables((&args).into()).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(NotCreated("Relation type wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::Delete(args) => match client.types().relations().delete_relation_type_with_variables((&args).into()).await {
            Ok(Some(true)) => Ok(format!("Relation type {}__{} deleted", args.namespace, args.name).into()),
            Ok(Some(false)) => Ok(format!("Relation type {}__{} not deleted", args.namespace, args.name).into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::AddProperty(args) => match client.types().relations().add_property_with_variables((&args).into()).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(NotCreated("Property wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::RemoveProperty(args) => match client.types().relations().remove_property_with_variables((&args).into()).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(NotFound(format!("Relation type {}__{} not found", args.ty.namespace, args.ty.name))),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::AddExtension(args) => match client.types().relations().add_extension_with_variables((&args).into()).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(NotCreated("Extension wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::RemoveExtension(args) => match client.types().relations().remove_extension_with_variables((&args).into()).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(args.relation_ty.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::AddComponent(args) => match client.types().relations().add_component((&args).into()).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(NotCreated("Component wasn't added to relation type".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::RemoveComponent(args) => match client.types().relations().remove_component((&args).into()).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(args.ty.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::UpdateDescription(args) => match client.types().relations().update_description_with_variables((&args).into()).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(args.ty.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::JsonSchema => match client.json_schema().types().relations().await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NotFound("JSON Schema not available".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}
