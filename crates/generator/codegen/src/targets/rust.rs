use crate::CodeFormatter;
use crate::CodeFormatterError;
use crate::CodeGenerationConfig;
use crate::CodeGenerationTarget;
use proc_macro2::TokenStream;
use quote::quote;
use semver::VersionReq;
use syn::{self};

pub struct Rust {}

impl Rust {
    pub fn multiline_doc_comment<S: Into<String>>(comment: S) -> TokenStream {
        let mut doc_comments = vec![];
        for line in comment.into().split("\n") {
            let mut line = line.trim().to_owned();
            line.insert_str(0, " ");
            doc_comments.push(quote! {
                #[doc = #line]
            });
        }
        quote! {
            #(#doc_comments)*
        }
    }
}
impl CodeGenerationTarget for Rust {
    fn name() -> String {
        "rust".to_string()
    }

    fn version() -> VersionReq {
        VersionReq::parse(">=1.80").unwrap()
    }

    fn extension() -> String {
        "rs".to_string()
    }
}

impl CodeFormatter for Rust {
    fn format(unformatted: String, config: &CodeGenerationConfig) -> Result<String, CodeFormatterError> {
        if config.formatting {
            let file = match syn::parse_file(&unformatted) {
                Ok(file) => file,
                Err(e) => {
                    if config.ignore_formatter_errors {
                        return Ok(unformatted.clone());
                    }
                    return Err(CodeFormatterError::ParserError(e.to_string(), unformatted));
                }
            };
            let formatted = prettyplease::unparse(&file);
            let formatted = formatted.replace("#[doc(newline)]\n", "\n");
            Ok(formatted)
        } else {
            Ok(unformatted)
        }
    }
}
