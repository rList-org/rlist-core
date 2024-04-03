use std::time::SystemTime;
use rand::{Rng, thread_rng};
use crate::combinable::Combinable;
use crate::{VfsBasicMeta, VfsFileMeta};

/// The download link can be determined **when instance is created**.
pub trait StaticDownloadLinkFile: VfsBasicMeta {
    fn new(name: String, size: u64, last_modified: SystemTime, links: Vec<String>) -> Self;

    /// Get download links
    fn links(&self) -> &Vec<String>;

    /// Destructor, returns
    /// 1. name
    /// 2. size
    /// 3. last_modified
    /// 4. links
    fn destruct(self) -> (String, u64, SystemTime, Vec<String>);
}

impl<T: StaticDownloadLinkFile> Combinable for T {
    /// Combine **same** files which have different download links to one file.
    fn combine(from: Vec<Self>) -> Self {
        let destructed: Vec<(String, u64, SystemTime, Vec<String>)> = from.into_iter()
            .map(|x| x.destruct())
            .collect::<Vec<_>>();
        let new_name = destructed[0].0.clone();
        let new_size = destructed.iter().map(|x| x.1).max().unwrap();
        let new_last_modified = destructed.iter().map(|x| x.2).max().unwrap();
        let download_links: Vec<String> = destructed.iter().map(|x| x.3.clone()).flatten().collect();
        return Self::new(new_name, new_size, new_last_modified.clone(), download_links);
    }
}

impl<T> VfsFileMeta for T
where T: StaticDownloadLinkFile
{
    /// return a random link in list
    fn on_download(&self) -> String {
        let links = self.links();
        let index = thread_rng().gen_range(0..links.len());
        return links[index].clone()
    }
}

#[derive(Clone)]
/// minimal implementation of `StaticDownloadLinkFile`
pub struct StaticCombinableFile {
    pub name: String,
    pub size: u64,
    pub last_modified: SystemTime,
    pub links: Vec<String>,
}

impl StaticCombinableFile {
    pub fn random_link(&self) -> String {
        let index = thread_rng().gen_range(0..self.links.len());
        return self.links[index].clone()
    }
}

impl VfsBasicMeta for StaticCombinableFile {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn last_modified(&self) -> SystemTime {
        self.last_modified
    }
}

impl StaticDownloadLinkFile for StaticCombinableFile {
    fn new(name: String, size: u64, last_modified: SystemTime, links: Vec<String>) -> Self {
        Self {
            name,
            size,
            last_modified,
            links,
        }
    }

    fn links(&self) -> &Vec<String> {
        &self.links
    }

    fn destruct(self) -> (String, u64, SystemTime, Vec<String>) {
        (self.name, self.size, self.last_modified, self.links)
    }
}

#[cfg(test)]
mod tests {
    use crate::combine;
    use super::*;

    #[test]
    fn test_static_combinable_file() {
        // last modified: 2023-1-1 00:00:00 UTC-0
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1672531200);
        let file = StaticCombinableFile {
            name: "test".to_string(),
            size: 1024,
            last_modified: time,
            links: vec!["https://example.com".to_string(), "https://example.org".to_string()],
        };
        assert_eq!(file.name(), "test");
        assert_eq!(file.size(), 1024);
        assert_eq!(file.last_modified(), time);
        assert_eq!(file.links(), &vec!["https://example.com".to_string(), "https://example.org".to_string()]);
    }

    #[test]
    fn combine_files() {
        // last modified: 2023-1-1 00:00:00 UTC-0
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1672531200);
        let name = "test".to_string();
        let size = 1024;

        let file1 = StaticCombinableFile {
            name: name.clone(),
            size,
            last_modified: time,
            links: vec!["https://example.com".to_string()],
        };

        let file2 = StaticCombinableFile {
            name: name.clone(),
            size,
            last_modified: time,
            links: vec!["https://example.org".to_string()],
        };

        let file3 = StaticCombinableFile {
            name: name.clone(),
            size,
            last_modified: time,
            links: vec!["https://example.net".to_string()],
        };

        let combined = combine![file1, file2, file3];
        assert_eq!(combined.name(), "test");
        assert_eq!(combined.size(), 1024);
        assert_eq!(combined.last_modified(), time);
        assert_eq!(
            combined.links(), &vec![
                "https://example.com".to_string(),
                "https://example.org".to_string(),
                "https://example.net".to_string()
            ]
        );
    }
}