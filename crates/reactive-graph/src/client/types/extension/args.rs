use crate::client::types::entities::args::parse_entity_ty;
use clap::Args;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use serde_json::Value;
use std::error::Error;
use std::str::FromStr;

pub fn parse_extension_ty(namespace: &str) -> Result<ExtensionTypeId, Box<dyn Error + Send + Sync + 'static>> {
    Ok(ExtensionTypeId::from_str(namespace).map_err(Box::new)?)
}

/// The property type.
#[derive(Args, Debug, Clone)]
pub(crate) struct ExtensionDefinitionArgs {
    /// The fully qualified namespace of the extension.
    #[clap(name = "extension_type", value_parser = parse_extension_ty)]
    pub extension_ty: ExtensionTypeId,

    /// The fully qualified namespace of the entity type which is the type constraint of the extension.
    #[clap(long, name = "extension_entity_type", value_parser = parse_entity_ty)]
    pub entity_ty: Option<EntityTypeId>,

    /// Textual description of the extension.
    #[clap(long)]
    pub description: Option<String>,

    /// The extension as JSON representation.
    pub extension: Value,
}

impl From<&ExtensionDefinitionArgs> for Extension {
    fn from(args: &ExtensionDefinitionArgs) -> Self {
        Self {
            ty: args.extension_ty.clone(),
            entity_ty: args.entity_ty.clone(),
            description: args.description.clone().unwrap_or_default(),
            extension: args.extension.clone(),
        }
    }
}
