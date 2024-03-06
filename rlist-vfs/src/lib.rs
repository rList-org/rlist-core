pub mod static_combinable;
mod combinable;
mod combinable_dir;
mod rcu;
pub mod driver;
mod without_link;
mod wheel;

pub use wheel::Wheel as RListCore;

/// Basic VFS (Virtual File System) traits
pub trait VfsBasicMeta
    where Self: Send + Sync + Clone + Sized + 'static
{
    fn name(&self) -> &str;
    fn size(&self) -> u64;  // in bytes
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

