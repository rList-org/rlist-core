mod driver_index;
mod get_vfs;
mod into_static_combinable_file;
mod static_link_file;
mod vfs_meta;

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

#[proc_macro_derive(GetVfs)]
pub fn auto_get_vfs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    get_vfs::auto_get_vfs(input)
}

#[proc_macro_attribute]
pub fn rlist_driver_index(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    driver_index::rlist_driver_index(attr, item)
}
