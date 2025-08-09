use reactive_graph_command_types::CommandComponentsProviderAssets;
use reactive_graph_command_types::CommandEntityTypesProviderAssets;
use reactive_graph_command_types::CommandFlowTypesProviderAssets;
use reactive_graph_command_types::CommandRelationTypesProviderAssets;
use reactive_graph_generator_documentation::GenerateDocumentations;
use reactive_graph_graph::NamespacedTypeComponentPropertiesContainer;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../types");
    let components = CommandComponentsProviderAssets::get_components();
    components.write_documentations()?;
    let entity_types = CommandEntityTypesProviderAssets::get_entity_types();
    entity_types.merge_component_properties(components.clone())?;
    entity_types.write_documentations()?;
    let relation_types = CommandRelationTypesProviderAssets::get_relation_types();
    relation_types.merge_component_properties(components)?;
    relation_types.write_documentations()?;
    CommandFlowTypesProviderAssets::get_flow_types().write_documentations()?;
    Ok(())
}
