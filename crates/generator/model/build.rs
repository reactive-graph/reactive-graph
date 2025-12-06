// use reactive_graph_command_types::CommandComponentsProviderAssets;
// use reactive_graph_command_types::CommandEntityTypesProviderAssets;
// use reactive_graph_command_types::CommandFlowTypesProviderAssets;
// use reactive_graph_command_types::CommandRelationTypesProviderAssets;
use reactive_graph_generator_codegen::CodeGenerationTargets::Rust;
use reactive_graph_generator_codegen::GenerateTypeSystemTypesForTarget;
use reactive_graph_generator_codegen::SRC_DIRECTORY;
use reactive_graph_generator_codegen::config::CodeGenerationConfig;
use reactive_graph_generator_documentation::DOCS_DIRECTORY;
use reactive_graph_generator_documentation::types::collections::GenerateTypeSystemDocumentations;
use reactive_graph_generator_documentation::types::config::DocumentationConfig;
use reactive_graph_generator_documentation::types::config::DocumentationConfigPreset;
use reactive_graph_generator_documentation::types::config::FromDocumentationConfigPreset;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::TypeDefinitionImporter;
use reactive_graph_graph::TypeResolver;
use reactive_graph_graph::TypeSystem;
use std::path::PathBuf;
use std::str::FromStr;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const CARGO_CRATE_NAME: &str = env!("CARGO_CRATE_NAME");
const TYPES_DIR: &str = "types";

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=types");
    let types_path = PathBuf::from(CARGO_MANIFEST_DIR).join(TYPES_DIR);
    let type_system = TypeSystem::import(types_path)?;
    type_system.merge_own_component_properties()?;
    let resolver = TypeResolver::new();
    resolver.insert(CARGO_CRATE_NAME.to_string(), type_system.clone());
    // generate_docs(&type_system, &resolver)?;
    // codegen(&type_system, &resolver)?;
    Ok(())
}

fn generate_docs(type_system: &TypeSystem, resolver: &TypeResolver) -> anyhow::Result<()> {
    let docs_path = PathBuf::from(CARGO_MANIFEST_DIR).join(DOCS_DIRECTORY);
    let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
    type_system.write_documentations(&docs_path, &config, resolver)?;
    docs_path.try_exists()?;
    Ok(())
}

fn codegen(type_system: &TypeSystem, resolver: &TypeResolver) -> anyhow::Result<()> {
    let src_path = PathBuf::from(CARGO_MANIFEST_DIR).join(SRC_DIRECTORY);
    let config = CodeGenerationConfig::new(src_path)
        .id(Namespace::from_str("reactive_graph::generator")?)
        .enable_builders()
        .ignore_formatter_errors();
    type_system.generate_types_for_target(Rust, &config, resolver)?;
    config.as_ref().try_exists()?;
    Ok(())
}
