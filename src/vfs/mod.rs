pub mod path_compress;
pub mod combine;
pub mod hide_url;

pub trait VfsBasicMeta: Send + Sync {
    fn name(&self) -> &str;
    fn size(&self) -> u64;  // in bytes
    fn last_modified(&self) -> std::time::SystemTime;
}

pub trait VfsFile: VfsBasicMeta {
    fn on_download(&self) -> String;
}

#[derive(Clone)]
pub enum VfsEntry<File: VfsFile, Dir: VfsDir<File> + Clone> {
    File(File),
    Dir(Dir),
}

impl<F,D> VfsBasicMeta for VfsEntry<F,D>
    where F: VfsFile, D: VfsDir<F> + Clone
{
    fn name(&self) -> &str {
        match self {
            VfsEntry::File(file) => file.name(),
            VfsEntry::Dir(dir) => dir.name()
        }
    }

    fn size(&self) -> u64 {
        match self {
            VfsEntry::File(file) => file.size(),
            VfsEntry::Dir(dir) => dir.size()
        }
    }
    fn last_modified(&self) -> std::time::SystemTime {
        match self {
            VfsEntry::File(file) => file.last_modified(),
            VfsEntry::Dir(dir) => dir.last_modified()
        }
    }
}

pub trait VfsDir<File: VfsFile>: VfsBasicMeta {
    fn list(&self) -> Vec<VfsEntry<File, Self>> where Self: Sized + Clone;
}