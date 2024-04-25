use crate::combinable_dir::CombinableDir;
use crate::driver::{CloudDriver, GetVfs};
use crate::static_combinable::StaticCombinableFile;
use crate::VfsBasicMeta;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct StaticFile {
    name: String,
    size: u64,
    last_modified: chrono::DateTime<chrono::Utc>,
    links: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StaticDir {
    name: String,
    size: u64,
    last_modified: chrono::DateTime<chrono::Utc>,
    files: Vec<StaticFile>,
    subdirectories: Vec<StaticDir>,
}

impl VfsBasicMeta for StaticFile {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn last_modified(&self) -> std::time::SystemTime {
        self.last_modified.into()
    }
}

impl VfsBasicMeta for StaticDir {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn last_modified(&self) -> std::time::SystemTime {
        self.last_modified.into()
    }
}

impl Into<StaticCombinableFile> for StaticFile {
    fn into(self) -> StaticCombinableFile {
        StaticCombinableFile {
            name: self.name,
            size: self.size,
            last_modified: self.last_modified.into(),
            links: self.links,
        }
    }
}

impl Into<CombinableDir<StaticCombinableFile>> for StaticDir {
    fn into(self) -> CombinableDir<StaticCombinableFile> {
        let subdirectories: Vec<CombinableDir<StaticCombinableFile>> =
            self.subdirectories.into_iter().map(|x| x.into()).collect();
        let files: Vec<StaticCombinableFile> = self.files.into_iter().map(|x| x.into()).collect();
        let name = self.name;
        CombinableDir::new(name, files, subdirectories)
    }
}

pub struct StaticDriver {
    config: StaticDir,
}

#[async_trait]
impl CloudDriver<StaticDir, StaticDir> for StaticDriver {
    async fn new(state: StaticDir) -> Self {
        Self { config: state }
    }

    async fn load_config(config: StaticDir) -> StaticDir {
        config
    }

    async fn reload_vfs(state: &StaticDir) -> Result<CombinableDir<StaticCombinableFile>, String> {
        Ok(state.clone().into())
    }
}

#[async_trait]
impl GetVfs for StaticDriver {
    async fn get_vfs(&self) -> Result<CombinableDir<StaticCombinableFile>, String> {
        StaticDriver::reload_vfs(&self.config).await
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_deserialize_file() {
        let json = r#"
        {
            "name": "file",
            "size": 1024,
            "last_modified": "2021-01-01T00:00:00Z",
            "links": ["https://example.com/file"]
        }
        "#;
        let file: super::StaticFile = serde_json::from_str(json).unwrap();
        assert_eq!(file.name, "file");
        assert_eq!(file.size, 1024);
        assert_eq!(file.last_modified.to_rfc3339(), "2021-01-01T00:00:00+00:00");
        assert_eq!(file.links, vec!["https://example.com/file"]);
    }

    #[test]
    fn test_deserialize_dir() {
        let json = r#"
        {
            "name": "dir",
            "size": 1024,
            "last_modified": "2021-01-01T00:00:00Z",
            "files": [
                {
                    "name": "file",
                    "size": 1024,
                    "last_modified": "2021-01-01T00:00:00Z",
                    "links": ["https://example.com/file"]
                }
            ],
            "subdirectories": [
                {
                    "name": "subdir",
                    "size": 1024,
                    "last_modified": "2021-01-01T00:00:00Z",
                    "files": [],
                    "subdirectories": []
                }
            ]
        }
        "#;
        let dir: super::StaticDir = serde_json::from_str(json).unwrap();
        assert_eq!(dir.name, "dir");
        assert_eq!(dir.size, 1024);
        assert_eq!(dir.last_modified.to_rfc3339(), "2021-01-01T00:00:00+00:00");
        assert_eq!(dir.files.len(), 1);
        assert_eq!(dir.subdirectories.len(), 1);
    }
}
