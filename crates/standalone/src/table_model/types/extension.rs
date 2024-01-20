use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;

// use inexor_rgf_graph::NamespacedTypeContainer;
use serde_json::Value;
use tabled::settings::object::Columns;
use tabled::settings::Modify;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;

use crate::table_model::styles::modern_inline::modern_inline;
use crate::table_model::types::json_value::pretty_json;
use inexor_rgf_graph::NamespacedTypeGetter;

#[derive(Clone, Debug, Tabled)]
pub struct ExtensionTypeId {
    pub name: String,
    pub namespace: String,
}

impl From<inexor_rgf_graph::ExtensionTypeId> for ExtensionTypeId {
    fn from(ty: inexor_rgf_graph::ExtensionTypeId) -> Self {
        ExtensionTypeId {
            name: ty.type_name(),
            namespace: ty.namespace(),
        }
    }
}

pub struct ExtensionDefinition {
    pub type_: ExtensionTypeId,
    pub description: String,
    pub extension: Value,
}

impl From<inexor_rgf_graph::Extension> for ExtensionDefinition {
    fn from(extension: inexor_rgf_graph::Extension) -> Self {
        ExtensionDefinition {
            type_: extension.ty.into(),
            description: extension.description,
            extension: extension.extension,
        }
    }
}

// pub fn display_extensions(extensions: &Vec<Extension>) -> String {
//     Table::new(extensions).to_string()
// }

pub fn display_extensions_inline(extensions: &Vec<Extension>) -> String {
    if extensions.is_empty() {
        return String::from("No extensions");
    }

    Table::new(extensions)
        .with(modern_inline())
        .with(Modify::new(Columns::new(0..1)).with(Width::increase(22)))
        .with(Modify::new(Columns::new(1..2)).with(Width::increase(22)))
        .with(Modify::new(Columns::new(2..3)).with(Width::wrap(40)))
        .with(Modify::new(Columns::new(3..4)).with(Width::wrap(80)))
        .to_string()
}

pub struct ExtensionDefinitions(pub Vec<ExtensionDefinition>);

impl From<ExtensionDefinitions> for Vec<ExtensionDefinition> {
    fn from(extensions: ExtensionDefinitions) -> Self {
        extensions.0.into_iter().collect()
    }
}

impl From<inexor_rgf_graph::Extensions> for ExtensionDefinitions {
    fn from(extensions: inexor_rgf_graph::Extensions) -> Self {
        ExtensionDefinitions(extensions.into_iter().map(|(_extension_ty, extension)| extension.into()).collect())
    }
}

#[derive(Clone, Debug, Tabled)]
pub struct Extension {
    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    pub description: String,

    /// The extension as JSON representation.
    #[tabled(display_with("pretty_json"))]
    pub extension: Value,
}

impl From<Extension> for inexor_rgf_graph::Extension {
    fn from(extension: Extension) -> Self {
        let ty = inexor_rgf_graph::ExtensionTypeId::new_from_type(extension.namespace, extension.name);
        inexor_rgf_graph::Extension {
            ty,
            description: extension.description,
            extension: extension.extension,
        }
    }
}

impl From<inexor_rgf_graph::Extension> for Extension {
    fn from(extension: inexor_rgf_graph::Extension) -> Self {
        Extension {
            namespace: extension.namespace(),
            name: extension.type_name(),
            description: extension.description,
            extension: extension.extension,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Extensions(pub Vec<Extension>);

impl Deref for Extensions {
    type Target = Vec<Extension>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Extensions> for inexor_rgf_graph::Extensions {
    fn from(extensions: Extensions) -> Self {
        extensions.0.into_iter().map(|extension| extension.into()).collect()
    }
}

impl From<Extensions> for Vec<Extension> {
    fn from(extensions: Extensions) -> Self {
        extensions.0
    }
}

impl From<inexor_rgf_graph::Extensions> for Extensions {
    fn from(extensions: inexor_rgf_graph::Extensions) -> Self {
        Extensions(extensions.into_iter().map(|(_extension_ty, extension)| extension.into()).collect())
    }
}

impl fmt::Display for Extensions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", Table::new(self.0.iter().cloned()))
    }
}
