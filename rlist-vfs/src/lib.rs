/// # Files that has static download link
/// There is a default implementation of `Combinable` for `StaticDownloadLinkFile`.
///
/// Dynamic download link is not supported yet.
pub mod static_combinable;

/// # `Combinable` trait means that `Vec<T>` can be combined to `T`
pub mod combinable;

/// # `CombinableDir` is a directory that can be combined
pub mod combinable_dir;
mod rcu;

/// # traits that driver must implement
///
/// if you want to implement a new driver, you must implement these traits
/// - [CloudDriver](driver::CloudDriver)
/// - [GetVfs](driver::GetVfs)
pub mod driver;

mod wheel;
mod without_link;

/// # Static Driver
/// a simple driver whose config is the vfs itself.
pub mod static_driver;

/// The state of rList server
pub use wheel::Wheel;

/// Basic VFS (Virtual File System) traits
pub trait VfsBasicMeta
where
    Self: Send + Sync + Clone + Sized + 'static,
{
    fn name(&self) -> &str;
    fn size(&self) -> u64; // in bytes
    fn last_modified(&self) -> std::time::SystemTime;
}

/// File in VFS
pub trait VfsFileMeta: VfsBasicMeta {
    fn on_download(&self) -> String;
}

/// Directory in VFS
pub trait VfsDirMeta<File: VfsFileMeta>: VfsBasicMeta {
    fn files(&self) -> &Vec<File>;
    fn subdirectories(&self) -> &Vec<Self>;
}
