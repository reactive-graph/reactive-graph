use reactive_graph_generator_codegen::CodeGenerationConfig;
use reactive_graph_generator_codegen::CodeGenerationTargets::Rust;
use reactive_graph_generator_codegen::GenerateTypeSystemTypesForTarget;
use reactive_graph_generator_codegen::SRC_DIRECTORY;
use reactive_graph_generator_codegen::TYPES_DIRECTORY;
use reactive_graph_generator_documentation::DOCS_DIRECTORY;
use reactive_graph_generator_documentation::DocumentationConfig;
use reactive_graph_generator_documentation::DocumentationConfigPreset;
use reactive_graph_generator_documentation::FromDocumentationConfigPreset;
use reactive_graph_generator_documentation::GenerateTypeSystemDocumentations;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::TypeDefinitionImporter;
use reactive_graph_graph::TypeResolver;
use reactive_graph_graph::TypeSystem;
use std::path::PathBuf;
use std::str::FromStr;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const TYPE_SYSTEM_NAMESPACE: &str = "reactive_graph::runtime";

fn main() -> anyhow::Result<()> {
    println!("cargo::rerun-if-changed={}", TYPES_DIRECTORY);
    let types_path = PathBuf::from(CARGO_MANIFEST_DIR).join(TYPES_DIRECTORY);
    let type_system = TypeSystem::import(types_path)?;
    let resolver = TypeResolver::new();
    resolver.insert("crate".to_string(), type_system.clone());
    resolver.insert("reactive_graph_model_core".to_string(), reactive_graph_model_core::type_system::TYPE_SYSTEM.clone());
    generate_docs(&type_system, &resolver)?;
    codegen(&type_system, &resolver)?;
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
        .id(Namespace::from_str(TYPE_SYSTEM_NAMESPACE)?)
        .enable_builders()
        .ignore_formatter_errors();
    type_system.generate_types_for_target(Rust, &config, resolver)?;
    config.as_ref().try_exists()?;
    Ok(())
}
