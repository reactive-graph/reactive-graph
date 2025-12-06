use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;

#[derive(Clone, Copy)]
pub enum Visibility {
    Private,
    Public,
}

impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Visibility::Private => {}
            Visibility::Public => {
                tokens.extend(quote! { pub });
            }
        }
    }
}
