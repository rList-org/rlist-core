use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn auto_get_vfs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the type to implement the trait for
    let name = &input.ident;

    // Generate the implementation
    let expanded = quote! {
        #[async_trait::async_trait]
        impl rlist_vfs::driver::GetVfs for #name {
            async fn get_vfs(&self) -> Result<CombinableDir<StaticCombinableFile>, String> {
                self.reload_vfs().await
            }
        }
    };

    // Return the generated implementation
    TokenStream::from(expanded)
}