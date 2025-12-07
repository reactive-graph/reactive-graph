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
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::MatchingInboundOutboundType;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
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
            Ok(Some(relation_types)) => output_format_wrapper.collection(relation_types.to_vec()),
            Ok(None) => Err(NoContent("No relation types found".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::Get { relation_ty } => match client.types().relations().get_relation_type_by_type(&relation_ty).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(relation_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::ListProperties { relation_ty } => match client.types().relations().get_relation_type_by_type(&relation_ty).await {
            Ok(Some(relation_type)) => {
                let output_format_wrapper: PropertyTypesOutputFormatWrapper = relation_type_args.output_format.into();
                output_format_wrapper.collection(relation_type.properties.to_vec())
            }
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(relation_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::ListExtensions { relation_ty } => match client.types().relations().get_relation_type_by_type(&relation_ty).await {
            Ok(Some(relation_type)) => {
                let output_format_wrapper: ExtensionsOutputFormatWrapper = relation_type_args.output_format.into();
                output_format_wrapper.collection(relation_type.extensions.to_vec())
            }
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(relation_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::ListComponents { relation_ty } => match client.types().relations().get_relation_type_components(&relation_ty).await {
            Ok(Some(components)) => {
                let output_format_wrapper: ComponentsOutputFormatWrapper = relation_type_args.output_format.into();
                output_format_wrapper.collection(components.to_vec())
            }
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(relation_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::GetJsonSchema { relation_ty } => match client.types().relations().json_schema_for_relation_type_by_type(&relation_ty).await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(relation_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::Create(args) => match client
            .types()
            .relations()
            .create_relation_type(
                RelationType::builder()
                    .outbound_type(InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(args.outbound_ty.clone())))
                    .ty(&args.relation_ty)
                    .inbound_type(InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(args.inbound_ty.clone())))
                    .description(args.description.unwrap_or_default())
                    .build(),
            )
            .await
        {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(NotCreated("Relation type wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::Delete { relation_ty } => match client.types().relations().delete_relation_type(&relation_ty).await {
            Ok(Some(true)) => Ok(format!("Entity Type {} deleted", relation_ty).into()),
            Ok(Some(false)) => Ok(format!("Entity Type {} not deleted", relation_ty).into()),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(relation_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::AddProperty(args) => match client.types().relations().add_property(&args.relation_ty, &args.property_type).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(NotCreated("Property wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::RemoveProperty(args) => match client.types().relations().remove_property(&args.relation_ty, args.property_name.clone()).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(args.relation_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::AddExtension(args) => match client.types().relations().add_extension(&args.relation_ty, &args.extension).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(NotCreated("Extension wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::RemoveExtension(args) => match client.types().relations().remove_extension(&args).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(args.relation_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::AddComponent(args) => match client.types().relations().add_component(&args).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(NotCreated("Component wasn't added to relation type".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::RemoveComponent(args) => match client.types().relations().remove_component(&args).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(args.relation_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::UpdateDescription(args) => match client.types().relations().update_description(&args.relation_ty, args.description).await {
            Ok(Some(relation_type)) => output_format_wrapper.single(relation_type),
            Ok(None) => Err(CommandError::NamespacedTypeNotFound(args.relation_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        RelationTypesCommands::JsonSchema => match client.json_schema().types().relations().await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NotFound("JSON Schema not available".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}
