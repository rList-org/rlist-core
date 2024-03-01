use std::time::UNIX_EPOCH;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::vfs::combine::{CombinableVfsDir, CombinableVfsFile};
use crate::vfs::{VfsBasicMeta, VfsDir, VfsEntry};

#[derive(Debug, Clone)]
pub struct UrlHiddenFile {
    name: String,
    size: u64,
    last_modified: std::time::SystemTime,
}
#[derive(Debug, Clone)]
pub struct UrlHiddenDir {
    name: String,
    size: u64,
    last_modified: std::time::SystemTime,
    children: Vec<UrlHiddenEntry>,
}

#[derive(Debug, Clone)]
pub enum UrlHiddenEntry {
    File(UrlHiddenFile),
    Dir(UrlHiddenDir),
}

impl Serialize for UrlHiddenFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("UrlHiddenFile", 4)?;
        state.serialize_field("_type", "file")?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("size", &self.size)?;
        let last_modified = self.last_modified.duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        state.serialize_field("last_modified", &last_modified)?;
        state.end()
    }
}

impl Serialize for UrlHiddenDir {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("UrlHiddenDir", 4)?;
        state.serialize_field("_type", "dir")?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("size", &self.size)?;
        let last_modified = self.last_modified.duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        state.serialize_field("last_modified", &last_modified)?;
        state.serialize_field("children", &self.children)?;
        state.end()
    }
}

impl Serialize for UrlHiddenEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match *self {
            UrlHiddenEntry::File(ref file) => file.serialize(serializer),
            UrlHiddenEntry::Dir(ref dir) => dir.serialize(serializer),
        }
    }
}

fn hide_url_for_file(file: &CombinableVfsFile) -> UrlHiddenFile {
    UrlHiddenFile {
        name: file.name().to_string(),
        size: file.size(),
        last_modified: file.last_modified(),
    }
}

pub fn hide_url_for_dir(dir: &CombinableVfsDir) -> UrlHiddenDir {
    let children = dir.list().into_iter().map(|entry| {
        match entry {
            VfsEntry::File(file) => UrlHiddenEntry::File(hide_url_for_file(&file)),
            VfsEntry::Dir(dir) => UrlHiddenEntry::Dir(hide_url_for_dir(&dir)),
        }
    }).collect();
    UrlHiddenDir {
        name: dir.name().to_string(),
        size: dir.size(),
        last_modified: dir.last_modified(),
        children,
    }
}