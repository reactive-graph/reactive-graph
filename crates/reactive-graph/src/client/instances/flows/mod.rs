use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use crate::client::instances::flows::args::FlowInstancesArgs;
use crate::client::instances::flows::commands::FlowInstancesCommands;
use crate::client::instances::flows::output_format::FlowInstancesOutputFormatWrapper;
use crate::client::result::CommandResult;
use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_graph::PropertyInstanceGetter;
use std::sync::Arc;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod output_format;

pub(crate) async fn flow_instances(client: &Arc<ReactiveGraphClient>, flow_instances_args: FlowInstancesArgs) -> CommandResult {
    let output_format_wrapper: FlowInstancesOutputFormatWrapper = flow_instances_args.output_format.clone().into();
    let Some(command) = flow_instances_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        // FlowInstancesCommands::List(args) => match client.instances().flow_instances().search((&args).into()).await {
        //     Ok(Some(flow_instances)) => output_format_wrapper.collection(flow_instances),
        //     Ok(None) => Err(NoContent("No flow instances found".to_string())),
        //     Err(e) => Err(e.into()),
        // },
        // FlowInstancesCommands::Get(args) => match client.instances().flow_instances().get_by_id(&args).await {
        //     Ok(Some(flow_instance)) => output_format_wrapper.single(flow_instance),
        //     Ok(None) => Err(args.not_found()),
        //     Err(e) => Err(e.into()),
        // },
        // FlowInstancesCommands::ListProperties(args) => match client.instances().flow_instances().get_by_id(&args).await {
        //     Ok(Some(flow_instance)) => {
        //         let output_format_wrapper: PropertyInstancesOutputFormatWrapper = flow_instances_args.output_format.into();
        //         let property_instances = flow_instance
        //             .properties
        //             .iter()
        //             .map(|property_instance| PropertyInstance::new(property_instance.key().clone(), property_instance.value().clone()))
        //             .collect();
        //         output_format_wrapper.collection(property_instances)
        //     }
        //     Ok(None) => Err(args.not_found()),
        //     Err(e) => Err(e.into()),
        // },
        // FlowInstancesCommands::GetProperty(args) => match client.instances().flow_instances().get_by_id(&args).await {
        //     Ok(Some(flow_instance)) => Ok(flow_instance.get(args.property_name.clone()).ok_or(args.property_not_found())?.into()),
        //     Ok(None) => Err(args.not_found()),
        //     Err(e) => Err(e.into()),
        // },
        // FlowInstancesCommands::SetProperty(args) => match client
        //     .instances()
        //     .flow_instances()
        //     .set_property(&args.id, &args.property_instance.property_name, args.property_instance.property_value.clone())
        //     .await
        // {
        //     Ok(Some(flow_instance)) => Ok(flow_instance
        //         .get(&args.property_instance.property_name)
        //         .ok_or(args.property_not_found())?
        //         .into()),
        //     Ok(None) => Err(args.not_found()),
        //     Err(e) => Err(e.into()),
        // },
        // FlowInstancesCommands::AddProperty(args) => {
        //     let property_type: PropertyType = args.property_type.clone().into();
        //     match client.instances().flow_instances().add_property(&args.id, property_type.clone()).await {
        //         Ok(Some(flow_instance)) => output_format_wrapper.single(flow_instance),
        //         Ok(None) => Err(args.id_not_found()),
        //         Err(e) => Err(e.into()),
        //     }
        // }
        // FlowInstancesCommands::RemoveProperty(args) => match client.instances().flow_instances().remove_property(&args.id, args.property_name.clone()).await {
        //     Ok(Some(flow_instance)) => output_format_wrapper.single(flow_instance),
        //     Ok(None) => Err(args.not_found()),
        //     Err(e) => Err(e.into()),
        // },
        // FlowInstancesCommands::ListComponents(args) => match client.instances().flow_instances().get_by_id(&args).await {
        //     Ok(Some(flow_instance)) => {
        //         let output_format_wrapper: ComponentTypeIdsOutputFormatWrapper = flow_instances_args.output_format.into();
        //         let component_tys = flow_instance.components.iter().map(|ty| ty.clone()).collect();
        //         output_format_wrapper.collection(component_tys)
        //     }
        //     Ok(None) => Err(args.not_found()),
        //     Err(e) => Err(e.into()),
        // },
        // FlowInstancesCommands::AddComponent(args) => {
        //     let component_ty: ComponentTypeId = args.component_ty.clone().into();
        //     match client.instances().flow_instances().add_component(&args, component_ty).await {
        //         Ok(Some(flow_instance)) => output_format_wrapper.single(flow_instance),
        //         Ok(None) => Err(args.not_found()),
        //         Err(e) => Err(e.into()),
        //     }
        // }
        // FlowInstancesCommands::RemoveComponent(args) => {
        //     let component_ty: ComponentTypeId = args.component_ty.clone().into();
        //     match client.instances().flow_instances().remove_component(&args, component_ty).await {
        //         Ok(Some(flow_instance)) => output_format_wrapper.single(flow_instance),
        //         Ok(None) => Err(args.not_found()),
        //         Err(e) => Err(e.into()),
        //     }
        // }
        // FlowInstancesCommands::Create(args) => match client
        //     .instances()
        //     .flow_instances()
        //     .create(&args.id, args.description.clone(), args.properties())
        //     .await
        // {
        //     Ok(Some(flow_instance)) => output_format_wrapper.single(flow_instance),
        //     Ok(None) => Err(NoContent("Flow instance not created".to_string())),
        //     Err(e) => Err(e.into()),
        // },
        // FlowInstancesCommands::Delete(args) => match client.instances().flow_instances().delete(&args).await {
        //     Ok(Some(true)) => Ok(format!("Flow instance {} deleted", &args).into()),
        //     Ok(Some(false)) => Ok(format!("Flow instance {} not deleted", &args).into()),
        //     Ok(None) => Err(args.not_found()),
        //     Err(e) => Err(e.into()),
        // },
        FlowInstancesCommands::JsonSchema => match client.json_schema().instances().flows().await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NotFound("JSON Schema not available".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}
