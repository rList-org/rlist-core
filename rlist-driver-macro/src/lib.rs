mod vfs_meta;
mod static_link_file;

#[proc_macro_derive(VfsMeta)]
pub fn vfs_meta_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    vfs_meta::vfs_meta_derive(input)
}

#[proc_macro_derive(StaticDownloadLinkFile)]
pub fn static_download_link_file_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_link_file::static_download_link_file_derive(input)
}