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

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use super::*;

    #[test]
    fn test_file_without_link() {
        let file = StaticCombinableFile {
            name: "test".to_string(),
            size: 1024,
            last_modified: SystemTime::now(),
            links: vec!["https://example.com".to_string()],
        };
        let without_link: FileWithoutLink = file.clone().into();
        assert_eq!(without_link.name, "test");
        assert_eq!(without_link.size, 1024);
    }

    #[test]
    fn test_dir_without_link() {
        let file1 = StaticCombinableFile {
            name: "test1".to_string(),
            size: 1024,
            last_modified: SystemTime::now(),
            links: vec!["https://example.com/1".to_string()],
        };
        let file2 = StaticCombinableFile {
            name: "test2".to_string(),
            size: 1024,
            last_modified: SystemTime::now(),
            links: vec!["https://example.com/2".to_string()],
        };
        let file3 = StaticCombinableFile {
            name: "test3".to_string(),
            size: 1024,
            last_modified: SystemTime::now(),
            links: vec!["https://example.com/3".to_string()],
        };
        let dir1 = CombinableDir::new(
            "dir1".to_string(), vec![file1.clone(), file2.clone()], vec![]
        );
        let dir2 = CombinableDir::new(
            "dir2".to_string(), vec![file3.clone()], vec![]
        );
        let dir3 = CombinableDir::new(
            "dir3".to_string(), vec![], vec![dir1.clone(), dir2.clone()]
        );

        // dir3
        // ├── dir1
        // │   ├── file1
        // │   └── file2
        // └── dir2
        //     └── file3

        let without_link: DirWithoutLink = dir3.clone().into();
        // dir3
        assert_eq!(without_link.name, "dir3");
        assert_eq!(without_link.size, 1024 * 3);
        assert_eq!(without_link.files.len(), 0);
        assert_eq!(without_link.subdirectories.len(), 2);

        // dir3.dir1
        assert_eq!(without_link.subdirectories.len(), 2);
        let dir1 = &without_link.subdirectories[0];
        assert_eq!(dir1.name, "dir1");
        assert_eq!(dir1.size, 1024 * 2);
        assert_eq!(dir1.files.len(), 2);
        assert_eq!(dir1.subdirectories.len(), 0);
        let file1 = &dir1.files[0];
        assert_eq!(file1.name, "test1");
        assert_eq!(file1.size, 1024);
        let file2 = &dir1.files[1];
        assert_eq!(file2.name, "test2");
        assert_eq!(file2.size, 1024);

        // dir3.dir2
        let dir2 = &without_link.subdirectories[1];
        assert_eq!(dir2.name, "dir2");
        assert_eq!(dir2.size, 1024);
        assert_eq!(dir2.files.len(), 1);
        assert_eq!(dir2.subdirectories.len(), 0);
        let file3 = &dir2.files[0];
        assert_eq!(file3.name, "test3");
    }

    fn new_file(name: String) -> StaticCombinableFile {
        // 2023-1-14 13:20:00 UTC-0
        let last_modified = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1673702400);
        let link1 = format!("https://example.com/{}", name);
        let link2 = format!("https://example.org/{}", name);

        StaticCombinableFile {
            name,
            size: 1024,
            last_modified,
            links: vec![link1, link2],
        }
    }

    #[test]
    fn test_serialize_file() {
        let file = new_file("test".to_string());
        let without_link: FileWithoutLink = file.clone().into();
        let json = serde_json::to_string(&without_link).unwrap();
        assert_eq!(json, r#"{"name":"test","size":1024,"last_modified":"2023-01-14T13:20:00Z"}"#);
    }

    #[test]
    fn test_serialize_dir() {
        let file1 = new_file("test1".to_string());
        let dir1 = CombinableDir::new("dir1".to_string(), vec![file1], vec![]);
        let file2 = new_file("test2".to_string());
        let dir2 = CombinableDir::new("dir2".to_string(), vec![file2], vec![dir1]);
        let without_link: DirWithoutLink = dir2.clone().into();
        let json = serde_json::to_string(&without_link).unwrap();
        assert_eq!(json, r#"{"name":"dir2","files":[{"name":"test2","size":1024,"last_modified":"2023-01-14T13:20:00Z"}],"subdirectories":[{"name":"dir1","files":[{"name":"test1","size":1024,"last_modified":"2023-01-14T13:20:00Z"}],"subdirectories":[],"size":1024,"last_modified":"2023-01-14T13:20:00Z"}],"size":2048,"last_modified":"2023-01-14T13:20:00Z"}"#);
    }
}