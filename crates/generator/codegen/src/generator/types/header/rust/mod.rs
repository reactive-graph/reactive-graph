use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_header_generated_code() -> TokenStream {
    quote! {
        //! ---------------------------------------------
        //! This file was generated automatically.
        //! ---------------------------------------------
        #![allow(dead_code, unused)]
        #![cfg_attr(rustfmt, rustfmt_skip)]
    }
}
