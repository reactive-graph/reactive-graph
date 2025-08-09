pub mod component_type_id_container;
pub mod extension_container;
pub mod property_type_container;
pub mod type_definition;
pub mod type_definition_json_schema;
pub mod type_description;
pub mod variables_container;

use markdown_builder::Markdown;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct MarkdownDocumentation<TY> {
    pub(crate) ty: TY,
    pub(crate) document: Markdown,
}

impl<TY> MarkdownDocumentation<TY>
where
    TY: Clone,
{
    pub fn new(ty: TY) -> Self {
        Self { ty, document: Markdown::new() }
    }
}

impl<TY> Display for MarkdownDocumentation<TY> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.document.render())
    }
}
