pub mod component_type_id_container;
pub mod extension_container;
pub mod property_type;
pub mod property_type_container;
pub mod type_definition;
pub mod type_definition_json_schema;
pub mod type_description;
pub mod variables_container;

use markdown_builder::Markdown;
use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::Arc;
use std::sync::RwLock;

pub struct TypedMarkdownDocumentation<TY> {
    pub(crate) ty: TY,
    pub(crate) document: Arc<RwLock<Markdown>>,
}

impl<TY> TypedMarkdownDocumentation<TY>
where
    TY: Clone,
{
    pub fn new(ty: TY) -> Self {
        Self {
            ty,
            document: Arc::new(RwLock::new(Markdown::new())),
        }
    }

    pub fn new_with_document(ty: TY, document: Arc<RwLock<Markdown>>) -> Self {
        Self { ty, document }
    }
}

impl<TY> Display for TypedMarkdownDocumentation<TY> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let document = self.document.read().unwrap();
        write!(f, "{}", document.render())
    }
}

pub struct MarkdownDocumentation(Arc<RwLock<Markdown>>);

impl<TY> From<TypedMarkdownDocumentation<TY>> for MarkdownDocumentation {
    fn from(markdown_documentation: TypedMarkdownDocumentation<TY>) -> Self {
        MarkdownDocumentation(markdown_documentation.document)
    }
}

impl<TY> From<&TypedMarkdownDocumentation<TY>> for MarkdownDocumentation {
    fn from(markdown_documentation: &TypedMarkdownDocumentation<TY>) -> Self {
        MarkdownDocumentation(markdown_documentation.document.clone())
    }
}

impl Display for MarkdownDocumentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let document = self.0.read().unwrap();
        write!(f, "{}", document.render())
    }
}
