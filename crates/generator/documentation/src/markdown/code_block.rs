use markdown_builder::Markdown;
use markdown_builder::MarkdownElement;

#[derive(Clone, Debug)]
pub struct MarkdownCodeBlock(pub String, pub String);

impl MarkdownElement for MarkdownCodeBlock {
    fn render(&self) -> String {
        format!("```{}\n{}\n```\n", self.0, self.1)
    }
}

pub trait MarkdownCodeBlockExt {
    fn code_block(&mut self, lang: String, code: String) -> &mut Self;
}

impl MarkdownCodeBlockExt for Markdown {
    fn code_block(&mut self, lang: String, code: String) -> &mut Self {
        self.elements.push(Box::new(MarkdownCodeBlock(lang, code)));
        self
    }
}
