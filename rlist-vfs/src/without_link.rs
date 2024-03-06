use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::combinable_dir::CombinableDir;
use crate::static_combinable::StaticCombinableFile;
use crate::VfsBasicMeta;

#[derive(Clone, Serialize)]
pub struct FileWithoutLink {
    pub name: String,
    pub size: u64,
    pub last_modified: DateTime<Utc>,
}

#[derive(Clone, Serialize)]
pub struct DirWithoutLink {
    pub name: String,
    pub files: Vec<FileWithoutLink>,
    pub subdirectories: Vec<DirWithoutLink>,
    pub size: u64,
    pub last_modified: DateTime<Utc>,
}

impl Into<FileWithoutLink> for StaticCombinableFile {
    fn into(self) -> FileWithoutLink {
        FileWithoutLink {
            name: self.name,
            size: self.size,
            last_modified: self.last_modified.into(),
        }
    }
}

impl Into<DirWithoutLink> for CombinableDir<StaticCombinableFile> {
    fn into(self) -> DirWithoutLink {
        let size = self.size();
        let last_modified = self.last_modified();
        let (name, files, subdirectories) = self.destruct();
        let files = files.into_iter().map(|x| x.into()).collect();
        let subdirectories = subdirectories.into_iter().map(|x| x.into()).collect();
        DirWithoutLink {
            name,
            files,
            subdirectories,
            size,
            last_modified: last_modified.into(),
        }
    }
}