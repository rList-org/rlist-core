use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_auto_static_combinable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl Into<StaticCombinableFile> for #name {
            fn into(self) -> StaticCombinableFile {
                StaticCombinableFile {
                    name: self.name().to_string(),
                    size: self.size(),
                    last_modified: self.last_modified(),
                    links: self.links().clone(),
                }
            }
        }
    };

    TokenStream::from(expanded)
}