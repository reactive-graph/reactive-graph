use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NoContent;
use crate::cli::instances::properties::output_format::PropertyInstancesOutputFormatWrapper;
use crate::cli::instances::relations::args::RelationInstancesArgs;
use crate::cli::instances::relations::commands::RelationInstancesCommands;
use crate::cli::instances::relations::output_format::RelationInstancesOutputFormatWrapper;
use crate::cli::result::CommandResult;
use crate::cli::types::components::output_format::ComponentTypeIdsOutputFormatWrapper;
use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyType;
use reactive_graph_table_model::instances::properties::PropertyInstance;
use std::sync::Arc;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod output_format;

pub(crate) async fn relation_instances(client: &Arc<ReactiveGraphClient>, relation_instances_args: RelationInstancesArgs) -> CommandResult {
    let output_format_wrapper: RelationInstancesOutputFormatWrapper = relation_instances_args.output_format.clone().into();
    let Some(command) = relation_instances_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        RelationInstancesCommands::List(args) => match client.instances().relation_instances().search((&args).into()).await {
            Ok(Some(relation_instances)) => output_format_wrapper.collection(relation_instances),
            Ok(None) => Err(NoContent("No relation instances found".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationInstancesCommands::Get(args) => match client.instances().relation_instances().get_by_id(&args).await {
            Ok(Some(relation_instance)) => output_format_wrapper.single(relation_instance),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationInstancesCommands::ListProperties(args) => match client.instances().relation_instances().get_by_id(&args).await {
            Ok(Some(relation_instance)) => {
                let output_format_wrapper: PropertyInstancesOutputFormatWrapper = relation_instances_args.output_format.into();
                let property_instances = relation_instance
                    .properties
                    .iter()
                    .map(|property_instance| PropertyInstance::new(property_instance.key().clone(), property_instance.value().clone()))
                    .collect();
                output_format_wrapper.collection(property_instances)
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationInstancesCommands::GetProperty(args) => match client.instances().relation_instances().get_by_id(&args).await {
            Ok(Some(relation_instance)) => Ok(relation_instance.get(args.property_name.clone()).ok_or(args.property_not_found())?.into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationInstancesCommands::SetProperty(args) => match client
            .instances()
            .relation_instances()
            .set_property(&args.id, &args.property_instance.property_name, args.property_instance.property_value.clone())
            .await
        {
            Ok(Some(relation_instance)) => Ok(relation_instance
                .get(&args.property_instance.property_name)
                .ok_or(args.property_not_found())?
                .into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationInstancesCommands::AddProperty(args) => {
            let property_type: PropertyType = args.property_type.clone().into();
            match client.instances().relation_instances().add_property(&args.id, property_type.clone()).await {
                Ok(Some(relation_instance)) => output_format_wrapper.single(relation_instance),
                Ok(None) => Err(args.id_not_found()),
                Err(e) => Err(e.into()),
            }
        }
        RelationInstancesCommands::RemoveProperty(args) => {
            match client
                .instances()
                .relation_instances()
                .remove_property(&args.id, args.property_name.clone())
                .await
            {
                Ok(Some(relation_instance)) => output_format_wrapper.single(relation_instance),
                Ok(None) => Err(args.not_found()),
                Err(e) => Err(e.into()),
            }
        }
        RelationInstancesCommands::ListComponents(args) => match client.instances().relation_instances().get_by_id(&args).await {
            Ok(Some(relation_instance)) => {
                let output_format_wrapper: ComponentTypeIdsOutputFormatWrapper = relation_instances_args.output_format.into();
                let component_tys = relation_instance.components.iter().map(|ty| ty.clone()).collect();
                output_format_wrapper.collection(component_tys)
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        RelationInstancesCommands::AddComponent(args) => {
            let component_ty: ComponentTypeId = args.component_ty.clone().into();
            match client.instances().relation_instances().add_component(&args, component_ty).await {
                Ok(Some(relation_instance)) => output_format_wrapper.single(relation_instance),
                Ok(None) => Err(args.not_found()),
                Err(e) => Err(e.into()),
            }
        }
        RelationInstancesCommands::RemoveComponent(args) => {
            let component_ty: ComponentTypeId = args.component_ty.clone().into();
            match client.instances().relation_instances().remove_component(&args, component_ty).await {
                Ok(Some(relation_instance)) => output_format_wrapper.single(relation_instance),
                Ok(None) => Err(args.not_found()),
                Err(e) => Err(e.into()),
            }
        }
        RelationInstancesCommands::Create(args) => match client
            .instances()
            .relation_instances()
            .create(&args.id, args.description.clone(), args.properties())
            .await
        {
            Ok(Some(relation_instance)) => output_format_wrapper.single(relation_instance),
            Ok(None) => Err(NoContent("Relation instance not created".to_string())),
            Err(e) => Err(e.into()),
        },
        RelationInstancesCommands::Delete(args) => match client.instances().relation_instances().delete(&args).await {
            Ok(Some(true)) => Ok(format!("Relation instance {} deleted", &args).into()),
            Ok(Some(false)) => Ok(format!("Relation instance {} not deleted", &args).into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
    }
}
