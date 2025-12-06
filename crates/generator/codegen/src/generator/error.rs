use crate::CodeGenerationTargets;
use crate::formatter::CodeFormatterError;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeResolveError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodeGenerationError {
    #[error("The target {0} is not supported.")]
    TargetNotSupported(CodeGenerationTargets),
    #[error("Failed to read {0}: {1}")]
    ReadError(PathBuf, std::io::Error),
    #[error("Failed to parse {0}: {1}")]
    ParserError(PathBuf, syn::Error),
    #[error("Failed to serialize {0}")]
    SerializeError(TypeDefinition),
    #[error("Failed to format source code for {0}: {1}")]
    FormatterError(PathBuf, CodeFormatterError),
    #[error("Failed to write source code to {0}: {1}")]
    WriteError(PathBuf, std::io::Error),
    #[error("Failed to create module to {0}")]
    CreateModuleDirError(PathBuf),
    #[error("Failed to create module to {0}: {1}")]
    CreateModuleError(PathBuf, std::io::Error),
    #[error("Failed to create source code folder {0}")]
    PathError(PathBuf),
    #[error("Failed to get parent namespace")]
    ParentNamespaceError(Namespace),
    #[error("Datatype error")]
    DataTypeError,
    #[error("Failed to resolve type {0}")]
    TypeResolveError(#[from] TypeResolveError),
    #[error("Failed to parse namespace {0}")]
    NamespaceParseError(Namespace),
}
