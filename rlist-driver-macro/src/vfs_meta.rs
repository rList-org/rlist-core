use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};


pub fn vfs_meta_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // name of the struct
    let name = input.ident;

    // necessary fields
    let mut has_name = false;
    let mut has_size = false;
    let mut has_last_modified = false;

    // check if the struct has the necessary fields
    // fields: name, size, last_modified
    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                let name = field.ident.as_ref().map(|ident| ident.to_string());
                if let Some(name) = name.as_deref() {
                    match name {
                        "name" => has_name = true,
                        "size" => has_size = true,
                        "last_modified" => has_last_modified = true,
                        _ => {}
                    }
                }
            } // end of for field in fields.named
        } else { // if not named fields
            return TokenStream::from(quote! {
                compile_error!("VfsMeta can only be derived for structs with named fields.");
            });
        } // end of if let Fields::Named(fields)
    } else { // if not a struct
        return TokenStream::from(quote! {
            compile_error!("VfsMeta can only be derived for structs.");
        });
    }; // end of if let Data::Struct(data)

    // if not all necessary fields are present, return an error
    let mut missing_fields: Vec<&str> = Vec::new();
    if !has_name {
        missing_fields.push("name");
    }
    if !has_size {
        missing_fields.push("size");
    }
    if !has_last_modified {
        missing_fields.push("last_modified");
    }
    if !missing_fields.is_empty() {
        let missing_fields = missing_fields.join(", ");
        let error_message =
            format!("VfsMeta Struct must contain '{}' fields.", missing_fields);
        return TokenStream::from(quote! {
            compile_error!(#error_message);
        });
    }

    // pass the check, generate the implementation
    let gen = quote! {
        impl rlist_vfs::VfsBasicMeta for #name where Self: Send + Sync + Clone + Sized + 'static {
            fn name(&self) -> &str {
                &self.name
            }

            fn size(&self) -> u64 {
                self.size
            }

            fn last_modified(&self) -> std::time::SystemTime {
                self.last_modified
            }
        }
    };

    gen.into()
}