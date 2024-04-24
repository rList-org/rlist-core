use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn static_download_link_file_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // name of the struct
    let name = input.ident;

    // necessary fields, contains `
    let mut has_links = false;

    // check if the struct has the necessary fields
    // fields: links
    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                let name = field.ident.as_ref().map(|ident| ident.to_string());
                if let Some(name) = name.as_deref() {
                    match name {
                        "links" => has_links = true,
                        _ => {}
                    }
                }
            } // end of for field in fields.named
        } else {
            // if not named fields
            return TokenStream::from(quote! {
                compile_error!("StaticDownloadLinkFile can only be derived for structs with named fields.");
            });
        } // end of if let Fields::Named(fields)
    } else {
        // if not a struct
        return TokenStream::from(quote! {
            compile_error!("StaticDownloadLinkFile can only be derived for structs.");
        });
    }; // end of if let Data::Struct(data)

    if !has_links {
        return TokenStream::from(quote! {
            compile_error!("StaticDownloadLinkFile Struct must contain 'links' fields.");
        });
    }

    let gen = quote! {
        impl rlist_vfs::static_combinable::StaticDownloadLinkFile for #name {
            fn new(
                name: String, size: u64, last_modified: std::time::SystemTime, links: Vec<String>
            )
            -> Self {
                Self { name, size, last_modified, links }
            }

            fn links(&self) -> &Vec<String> {
                &self.links
            }

            fn destruct(self) -> (String, u64, std::time::SystemTime, Vec<String>) {
                (self.name, self.size, self.last_modified, self.links)
            }
        }
    };

    gen.into()
}
