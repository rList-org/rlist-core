mod static_combinable;
mod combinable;
mod combinable_dir;

/// Basic VFS (Virtual File System) traits
pub trait VfsBasicMeta: Send + Sync + Sized + 'static {
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

