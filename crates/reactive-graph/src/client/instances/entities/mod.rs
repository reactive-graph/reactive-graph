use crate::client::error::CommandError;
use crate::client::error::CommandError::NoContent;
use crate::client::error::CommandError::NotFound;
use crate::client::instances::entities::args::EntityInstancesArgs;
use crate::client::instances::entities::commands::EntityInstancesCommands;
use crate::client::instances::entities::output_format::EntityInstancesOutputFormatWrapper;
use crate::client::instances::properties::output_format::PropertyInstancesOutputFormatWrapper;
use crate::client::result::CommandResult;
use crate::client::types::components::output_format::ComponentTypeIdsOutputFormatWrapper;
use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_table_model::instances::properties::PropertyInstance;
use std::sync::Arc;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod output_format;

pub(crate) async fn entity_instances(client: &Arc<ReactiveGraphClient>, entity_instances_args: EntityInstancesArgs) -> CommandResult {
    let output_format_wrapper: EntityInstancesOutputFormatWrapper = entity_instances_args.output_format.clone().into();
    let Some(command) = entity_instances_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        EntityInstancesCommands::List(args) => match client.instances().entities().search((&args).into()).await {
            Ok(Some(entity_instances)) => output_format_wrapper.collection(entity_instances.to_vec()),
            Ok(None) => Err(NoContent("No entity instances found".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::Get(args) => match client.instances().entities().get_entity_instance_by_id(args.clone()).await {
            Ok(Some(entity_instance)) => output_format_wrapper.single(entity_instance),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::GetByLabel(args) => match client.instances().entities().get_entity_instance_by_label(args.label.clone()).await {
            Ok(Some(entity_instance)) => output_format_wrapper.single(entity_instance),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::ListProperties(args) => match client.instances().entities().get_entity_instance_by_id(args.clone()).await {
            Ok(Some(entity_instance)) => {
                let output_format_wrapper: PropertyInstancesOutputFormatWrapper = entity_instances_args.output_format.into();
                let property_instances = entity_instance
                    .properties
                    .iter()
                    .map(|property_instance| PropertyInstance::new(property_instance.key().clone(), property_instance.value().clone()))
                    .collect();
                output_format_wrapper.collection(property_instances)
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::GetProperty(args) => match client.instances().entities().get_entity_instance_by_id(args.id).await {
            Ok(Some(entity_instance)) => Ok(entity_instance.get(&args.property_name).ok_or(args.property_not_found())?.into()),
            Ok(None) => Err(args.id_not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::SetProperty(args) => match client.instances().entities().set_property(args.id, args.name.clone(), args.value.clone()).await {
            Ok(Some(entity_instance)) => Ok(entity_instance.get(&args.name).ok_or(args.property_not_found())?.into()),
            Ok(None) => Err(args.id_not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::AddProperty(args) => match client.instances().entities().add_property(args.id, &args.property_type).await {
            Ok(Some(entity_instance)) => output_format_wrapper.single(entity_instance),
            Ok(None) => Err(args.id_not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::RemoveProperty(args) => match client.instances().entities().remove_property(args.id, args.property_name.clone()).await {
            Ok(Some(entity_instance)) => output_format_wrapper.single(entity_instance),
            Ok(None) => Err(args.id_not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::ListComponents(args) => match client.instances().entities().get_entity_instance_by_id(args.clone()).await {
            Ok(Some(entity_instance)) => {
                let output_format_wrapper: ComponentTypeIdsOutputFormatWrapper = entity_instances_args.output_format.into();
                let component_tys = entity_instance.components.iter().map(|ty| ty.clone()).collect();
                output_format_wrapper.collection(component_tys)
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::AddComponent(args) => {
            let component_ty: ComponentTypeId = args.component_ty.clone().into();
            match client.instances().entities().add_component(args.id, component_ty).await {
                Ok(Some(entity_instance)) => output_format_wrapper.single(entity_instance),
                Ok(None) => Err(args.id_not_found()),
                Err(e) => Err(e.into()),
            }
        }
        EntityInstancesCommands::RemoveComponent(args) => {
            let component_ty: ComponentTypeId = args.component_ty.clone().into();
            match client.instances().entities().remove_component(args.id, component_ty).await {
                Ok(Some(entity_instance)) => output_format_wrapper.single(entity_instance),
                Ok(None) => Err(args.id_not_found()),
                Err(e) => Err(e.into()),
            }
        }
        EntityInstancesCommands::Create(args) => match client
            .instances()
            .entities()
            .create(args.entity_ty.clone(), args.id, args.description.clone(), args.properties())
            .await
        {
            Ok(Some(entity_instance)) => output_format_wrapper.single(entity_instance),
            Ok(None) => Err(NoContent("Entity instance not created".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::Delete(args) => match client.instances().entities().delete_entity_instance(args.id).await {
            Ok(Some(true)) => Ok(format!("Entity instance {} deleted", args.id).into()),
            Ok(Some(false)) => Ok(format!("Entity instance {} not deleted", args.id).into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::JsonSchema => match client.json_schema().instances().entities().await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NotFound("JSON Schema not available".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}
