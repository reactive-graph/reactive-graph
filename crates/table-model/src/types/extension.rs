use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use tabled::derive::display;

use serde_json::Value;
use table_to_html::HtmlTable;
use tabled::Table;
use tabled::Tabled;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::settings::object::Columns;

use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;
use crate::styles::modern_inline::modern_inline;
use crate::types::json_value::pretty_json;
use reactive_graph_graph::NamespacedTypeGetter;

#[derive(Clone, Debug, Tabled)]
pub struct ExtensionTypeId {
    pub namespace: String,
    pub name: String,
}

impl From<reactive_graph_graph::ExtensionTypeId> for ExtensionTypeId {
    fn from(ty: reactive_graph_graph::ExtensionTypeId) -> Self {
        ExtensionTypeId {
            namespace: ty.path().to_string(),
            name: ty.type_name().to_string(),
        }
    }
}

pub struct ExtensionDefinition {
    pub type_: ExtensionTypeId,
    pub description: String,
    pub extension: Value,
}

impl From<reactive_graph_graph::Extension> for ExtensionDefinition {
    fn from(extension: reactive_graph_graph::Extension) -> Self {
        ExtensionDefinition {
            type_: extension.ty.into(),
            description: extension.description,
            extension: extension.extension,
        }
    }
}

pub fn display_extensions_inline_str(extensions: &[Extension]) -> String {
    if extensions.is_empty() {
        String::new()
    } else {
        display_extensions_inline(extensions).to_string()
    }
}

pub fn display_extensions_inline(extensions: &[Extension]) -> Table {
    let extensions = extensions.to_vec();
    Table::new(extensions)
        .with(modern_inline())
        .with(Modify::new(Columns::new(0..1)).with(Width::increase(22)))
        .with(Modify::new(Columns::new(1..2)).with(Width::increase(22)))
        .with(Modify::new(Columns::new(2..3)).with(Width::wrap(40)))
        .with(Modify::new(Columns::new(3..4)).with(Width::wrap(80)))
        .to_owned()
}

pub fn display_extensions_html_inline(extensions: &[Extension]) -> String {
    let extensions = extensions.to_vec();
    if extensions.is_empty() {
        return String::new();
    }
    HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder(&extensions)))
        .to_string()
        .split_whitespace()
        .collect()
}

pub struct ExtensionDefinitions(pub Vec<ExtensionDefinition>);

impl From<ExtensionDefinitions> for Vec<ExtensionDefinition> {
    fn from(extensions: ExtensionDefinitions) -> Self {
        extensions.0.into_iter().collect()
    }
}

impl From<reactive_graph_graph::Extensions> for ExtensionDefinitions {
    fn from(extensions: reactive_graph_graph::Extensions) -> Self {
        ExtensionDefinitions(extensions.into_iter().map(|(_extension_ty, extension)| extension.into()).collect())
    }
}

#[derive(Clone, Debug, Tabled)]
pub struct Extension {
    /// The namespace of the extension.
    #[tabled(rename = "Namespace")]
    pub namespace: String,

    /// The type name of the extension.
    #[tabled(rename = "Type Name")]
    pub name: String,

    /// The namespace of the entity type.
    #[tabled(rename = "Entity Namespace", display("display::option", ""))]
    pub entity_namespace: Option<String>,

    /// The type name of the entity type.
    #[tabled(rename = "Entity Type Name", display("display::option", ""))]
    pub entity_type_name: Option<String>,

    /// Textual description of the extension.
    #[tabled(rename = "Description")]
    pub description: String,

    /// The extension as JSON representation.
    #[tabled(rename = "Extension", display("pretty_json"))]
    pub extension: Value,

    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

// impl Extension {
//     pub fn entity_ty(&self) -> Option<EntityTypeId> {
//         if let (Some(namespace), Some(type_name)) = (&self.entity_namespace, &self.entity_type_name) {
//             Some(reactive_graph_graph::EntityTypeId::try_from(namespace, type_name))
//         } else {
//             None
//         }
//     }
// }

impl TableInlineFormatSetter for Extension {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

// impl From<Extension> for reactive_graph_graph::Extension {
//     fn from(extension: Extension) -> Self {
//         let entity_ty = extension.entity_ty();
//         let ty = reactive_graph_graph::ExtensionTypeId::new_from_type(extension.namespace, extension.name);
//         reactive_graph_graph::Extension {
//             ty,
//             entity_ty,
//             description: extension.description,
//             extension: extension.extension,
//         }
//     }
// }

impl From<reactive_graph_graph::Extension> for Extension {
    fn from(extension: reactive_graph_graph::Extension) -> Self {
        Extension {
            namespace: extension.path().to_string(),
            name: extension.type_name().to_string(),
            entity_namespace: extension.entity_ty.clone().map(|entity_ty| entity_ty.path().to_string()),
            entity_type_name: extension.entity_ty.map(|entity_ty| entity_ty.type_name().to_string()),
            description: extension.description,
            extension: extension.extension,
            inline_format: Default::default(),
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

// impl From<Extensions> for reactive_graph_graph::Extensions {
//     fn from(extensions: Extensions) -> Self {
//         extensions.0.into_iter().map(|extension| extension.into()).collect()
//     }
// }

impl From<Extensions> for Vec<Extension> {
    fn from(extensions: Extensions) -> Self {
        extensions.0
    }
}

impl From<reactive_graph_graph::Extensions> for Extensions {
    fn from(extensions: reactive_graph_graph::Extensions) -> Self {
        Extensions(extensions.into_iter().map(|(_extension_ty, extension)| extension.into()).collect())
    }
}

impl fmt::Display for Extensions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", Table::new(self.0.iter().cloned()))
    }
}

pub struct ExtensionsTableOptions;

impl TableOptions for ExtensionsTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended())
    }
}
