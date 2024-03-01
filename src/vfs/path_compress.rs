use std::collections::HashMap;
use crate::vfs::{VfsBasicMeta, VfsDir, VfsEntry, VfsFile};

pub struct IndexedVfs<File, Dir>
    where File: VfsFile, Dir: VfsDir<File> + Clone
{
    compressed_path: HashMap<String, VfsEntry<File, Dir>>
}

pub enum TryPathResult<File, Dir> {
    NotFound,
    Dir(Dir),
    File(File)
}

impl<F,D> IndexedVfs<F,D>
    where F: VfsFile + Clone, D: VfsDir<F> + Clone
{
    pub fn new(root: D) -> IndexedVfs<F,D> {
        let mut compressed_path = HashMap::new();
        IndexedVfs::compress_path(root.clone(), &mut compressed_path, "");
        IndexedVfs {
            compressed_path
        }
    }

    fn compress_path(dir: D, compressed_path: &mut HashMap<String, VfsEntry<F,D>>, path: &str) {
        for entry in dir.list() {
            let entry_path = format!("{}/{}", path, entry.name());
            compressed_path.insert(entry_path.clone(), entry.clone());
            match entry {
                VfsEntry::Dir(dir) => {
                    IndexedVfs::compress_path(dir, compressed_path, &entry_path);
                },
                VfsEntry::File(_) => {}
            }
        }
    }

    pub fn try_path(&self, path: &str) -> TryPathResult<F,D> {
        match self.compressed_path.get(path) {
            None => TryPathResult::NotFound,
            Some(entry) => {
                match entry {
                    VfsEntry::Dir(dir) => TryPathResult::Dir(dir.clone()),
                    VfsEntry::File(file) => TryPathResult::File(file.clone())
                }
            }
        }
    }
}