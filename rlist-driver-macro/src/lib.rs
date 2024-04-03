mod vfs_meta;
mod static_link_file;
mod into_static_combinable_file;
mod get_vfs;

#[proc_macro_derive(VfsMeta)]
pub fn vfs_meta_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    vfs_meta::vfs_meta_derive(input)
}

#[proc_macro_derive(StaticDownloadLinkFile)]
pub fn static_download_link_file_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_link_file::static_download_link_file_derive(input)
}

#[proc_macro_derive(StaticCombinableFile)]
pub fn derive_auto_static_combinable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    into_static_combinable_file::derive_auto_static_combinable(input)
}