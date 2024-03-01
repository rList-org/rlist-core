use crate::vfs::combine::{CombinableVfsDir, CombinableVfsFile};
use crate::vfs::{VfsDir, VfsEntry, VfsFile};

/// # Cloud Driver
/// The cloud driver is a driver that can be used to access a cloud storage service, then use the cloud storage service as a virtual file system(VFS).
#[async_trait::async_trait]
pub trait CloudDriver<Config: Send + Sync> {
    /// Convert the driver into VFS directory.
    fn into_combinable(self) -> CombinableVfsDir;
    async fn new(config: &Config) -> Result<Self, String> where Self: Sized;
}

pub trait CloudDriverFile: VfsFile + Sized {
    fn into_combinable(self) -> CombinableVfsFile {
        CombinableVfsFile::new(
            vec![self.on_download()],
            self.name().to_string(),
            self.size(),
            self.last_modified(),
        )
    }
}

pub trait CloudDriverDir<File: CloudDriverFile>: VfsDir<File> + Sized + Clone {
    fn into_combinable(self) -> CombinableVfsDir {
        let list = self.list();
        let mut dirs = Vec::new();
        let mut files = Vec::new();
        list.into_iter().for_each(|entry| {
            match entry {
                VfsEntry::File(file) => files.push(file.into_combinable()),
                VfsEntry::Dir(dir) => dirs.push(dir.into_combinable())
            }
        });
        let size = self.size();
        let name = self.name().to_string();
        CombinableVfsDir::new(name, dirs, files, size)
    }
}