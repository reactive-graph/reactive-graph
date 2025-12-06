use crate::CodeFormatterError;
use crate::CodeGenerationConfig;

pub trait CodeFormatter {
    fn format(unformatted: String, config: &CodeGenerationConfig) -> Result<String, CodeFormatterError>;
}
