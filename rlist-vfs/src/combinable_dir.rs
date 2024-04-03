use std::collections::HashMap;
use std::time::SystemTime;
use crate::static_combinable::StaticDownloadLinkFile;
use crate::{VfsBasicMeta, VfsDirMeta};
use crate::combinable::Combinable;

#[derive(Clone)]
pub struct CombinableDir<File: StaticDownloadLinkFile> {
    name: String,
    files: Vec<File>,
    subdirectories: Vec<CombinableDir<File>>,
    size: u64,
    last_modified: SystemTime,
}

impl<File: StaticDownloadLinkFile> CombinableDir<File> {
    pub fn new(name: String, files: Vec<File>, subdirectories: Vec<CombinableDir<File>>) -> Self {
        let size_file = files.iter().map(|x| x.size()).sum::<u64>();
        let size_subdirectories = subdirectories.iter().map(|x| x.size()).sum::<u64>();
        let size = size_file + size_subdirectories;
        let last_modified = if files.is_empty() && subdirectories.is_empty() {
            SystemTime::now()
        } else {
            let files_last_modified = files.iter().map(|x| x.last_modified());
            let subdirectories_last_modified = subdirectories.iter().map(|x| x.last_modified());
            files_last_modified.chain(subdirectories_last_modified)
                .max().unwrap()
        };
        Self {
            name,
            files,
            subdirectories,
            size,
            last_modified,
        }
    }

    /// Destructor, returns
    /// 1. name
    /// 2. files
    /// 3. subdirectories
    pub fn destruct(self) -> (String, Vec<File>, Vec<CombinableDir<File>>) {
        (self.name, self.files, self.subdirectories)
    }

    /// Move the root to the given path
    pub fn mount(self, path: Vec<String>) -> CombinableDir<File> {
        let path_reverse = path.into_iter().rev().collect::<Vec<_>>();
        let mut dir = self;
        for name in path_reverse {
            dir = CombinableDir::new(name, vec![], vec![dir]);
        }
        return dir;
    }

    pub fn compress_path(self) -> HashMap<String, File> {
        // TODO: Test this method
        let mut map: HashMap<String, File> = HashMap::new();
        let mut stack: Vec<(String, CombinableDir<File>)> = vec![(String::new(), self)];
        while !stack.is_empty() {
            let (path, dir) = stack.pop().unwrap();
            for file in dir.files {
                map.insert(path.clone() + file.name(), file);
            }
            for subdirectory in dir.subdirectories {
                stack.push((path.clone() + subdirectory.name() + "/", subdirectory));
            }
        }
        return map;
    }
}

impl<File: StaticDownloadLinkFile> VfsBasicMeta for CombinableDir<File> {
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

impl<File: StaticDownloadLinkFile> VfsDirMeta<File> for CombinableDir<File> {
    fn files(&self) -> &Vec<File> {
        &self.files
    }
    fn subdirectories(&self) -> &Vec<Self> {
        &self.subdirectories
    }
}

impl<File: StaticDownloadLinkFile> Combinable for CombinableDir<File> {
    fn combine(from: Vec<Self>) -> Self {
        let from = from.into_iter()
            .map(|x| x.destruct())
            .collect::<Vec<_>>();
        let new_name = from[0].0.clone();
        let (files, subdirectories): (
            Vec<Vec<File>>,
            Vec<Vec<CombinableDir<File>>>
        ) = from.into_iter()
            .map(|x| (x.1, x.2))
            .unzip();
        let files = files.into_iter()
            .flatten().collect::<Vec<_>>();
        let subdirectories = subdirectories.into_iter()
            .flatten().collect::<Vec<_>>();
        let files = divide_by_name(files);
        let files = files.into_iter()
            .map(|(_, files)| {
                File::combine(files)
            })
            .collect::<Vec<_>>();
        let subdirectories = divide_by_name(subdirectories);
        let subdirectories = subdirectories.into_iter()
            .map(|(_, subdirectories)| {
                CombinableDir::combine(subdirectories)
            })
            .collect::<Vec<_>>();
        return CombinableDir::new(new_name, files, subdirectories);
    }
}

fn divide_by_name<T: VfsBasicMeta>(items: Vec<T>) -> HashMap<String, Vec<T>> {
    let mut map: HashMap<String, Vec<T>> = HashMap::new();
    for item in items {
        let name = item.name().to_string();
        if map.contains_key(&name) {
            map.get_mut(&name).unwrap().push(item);
        } else {
            map.insert(name, vec![item]);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::static_combinable::StaticCombinableFile;

    fn generate_file(name: &str, size: u64, download_links_prefix: Vec<&str>) -> StaticCombinableFile {
        // last modified: 2023-1-1 00:00:00 UTC-0
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1672531200);

        let links = download_links_prefix.iter()
            .map(|x| format!("{}/{}", x, name))
            .collect();

        StaticCombinableFile {
            name: name.to_string(),
            size,
            last_modified: time,
            links,
        }
    }

    #[test]
    fn test_dir_new() {
        let file1 = generate_file("file1", 1024, vec!["https://example.com"]);
        let file2 = generate_file("file2", 2048, vec!["https://example.com"]);

        let dir1 = CombinableDir::new("dir1".to_string(), vec![file1, file2], vec![]);

        assert_eq!(dir1.name(), "dir1");
        assert_eq!(dir1.size(), 1024 + 2048);
        assert_eq!(dir1.last_modified(), SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1672531200));
        assert_eq!(dir1.files().len(), 2);
        assert_eq!(dir1.subdirectories().len(), 0);
    }

    #[test]
    fn test_dir_combine_1() {
        // combine 2 directories who have same file and no subdirectories
        let file1 = generate_file("test_file", 2048, vec!["https://example.com"]);
        let file2 = generate_file("test_file", 2048, vec!["https://example.org", "https://example.net"]);

        let dir1 = CombinableDir::new("dir1".to_string(), vec![file1], vec![]);
        let dir2 = CombinableDir::new("dir1".to_string(), vec![file2], vec![]);

        let combined = CombinableDir::combine(vec![dir1, dir2]);

        assert_eq!(combined.name(), "dir1");
        assert_eq!(combined.size(), 2048);
        assert_eq!(combined.last_modified(), SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1672531200));
        assert_eq!(combined.files().len(), 1);

        let file = combined.files()[0].clone();
        assert_eq!(file.name(), "test_file");
        assert_eq!(file.size(), 2048);
        assert_eq!(file.last_modified(), SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1672531200));
        assert_eq!(file.links(), &vec!["https://example.com/test_file", "https://example.org/test_file", "https://example.net/test_file"]);
    }

    #[test]
    fn test_dir_combine_2() {
        // combine 2 directories who have different files and no subdirectories
        let file1 = generate_file("test_file1", 2048, vec!["https://example.com"]);
        let file2 = generate_file("test_file2", 4096, vec!["https://example.com", "https://example.org"]);

        let dir1 = CombinableDir::new("dir1".to_string(), vec![file1], vec![]);
        let dir2 = CombinableDir::new("dir1".to_string(), vec![file2], vec![]);

        let combined = CombinableDir::combine(vec![dir1, dir2]);

        assert_eq!(combined.name(), "dir1");
        assert_eq!(combined.size(), 2048 + 4096);
        assert_eq!(combined.last_modified(), SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1672531200));
        assert_eq!(combined.files().len(), 2);

        let file1 = combined.files()[0].clone();
        assert_eq!(file1.name(), "test_file1");
        assert_eq!(file1.size(), 2048);
        assert_eq!(file1.links(), &vec!["https://example.com/test_file1"]);

        let file2 = combined.files()[1].clone();
        assert_eq!(file2.name(), "test_file2");
        assert_eq!(file2.size(), 4096);
        assert_eq!(file2.links(), &vec!["https://example.com/test_file2", "https://example.org/test_file2"]);
    }

    #[test]
    fn test_dir_combine_3() {
        // combine 2 directories who have different files and subdirectories
        // dir1
        // ├── file1
        // └── dir2
        //     └── file2
        // dir3
        // ├── file3
        // └── dir4
        //     └── file4

        let file1 = generate_file("file1", 2048, vec!["https://example.com"]);
        let file2 = generate_file("file2", 4096, vec!["https://example.com"]);
        let file3 = generate_file("file3", 8192, vec!["https://example.com"]);
        let file4 = generate_file("file4", 16384, vec!["https://example.com"]);

        let dir2 = CombinableDir::new("dir2".to_string(), vec![file2], vec![]);
        let dir4 = CombinableDir::new("dir4".to_string(), vec![file4], vec![]);

        let dir1 = CombinableDir::new("dir1".to_string(), vec![file1], vec![dir2]);
        let dir3 = CombinableDir::new("dir1".to_string(), vec![file3], vec![dir4]);

        assert_eq!(dir1.size(), 2048 + 4096);
        assert_eq!(dir3.size(), 8192 + 16384);

        let combined = CombinableDir::combine(vec![dir1, dir3]);
        assert_eq!(combined.name(), "dir1");
        assert_eq!(combined.size(), 2048 + 4096 + 8192 + 16384);
        assert_eq!(combined.last_modified(), SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1672531200));
        assert_eq!(combined.files().len(), 2);
        assert_eq!(combined.subdirectories().len(), 2);

        let mut file_names_in_combined = combined.files().iter().map(|x| x.name()).collect::<Vec<_>>();
        file_names_in_combined.sort();
        assert_eq!(file_names_in_combined, vec!["file1", "file3"]);

        let mut subdirectory_names_in_combined = combined.subdirectories().iter().map(|x| x.name()).collect::<Vec<_>>();
        subdirectory_names_in_combined.sort();
        assert_eq!(subdirectory_names_in_combined, vec!["dir2", "dir4"]);

        let file2 = combined.subdirectories().iter().find(|x| x.name() == "dir2").unwrap().files()[0].clone();
        assert_eq!(file2.name(), "file2");

        let file4 = combined.subdirectories().iter().find(|x| x.name() == "dir4").unwrap().files()[0].clone();
        assert_eq!(file4.name(), "file4");
    }

    #[test]
    fn test_mount() {
        let file1 = generate_file("file1", 2048, vec!["https://example.com"]);
        let dir = CombinableDir::new("dir".to_string(), vec![file1], vec![]);

        let path = vec!["root".to_owned(), "home".to_owned(), "user".to_owned()];

        let mounted = dir.mount(path);
        let mut dir_ptr = &mounted;
        assert_eq!(dir_ptr.name(), "root");
        dir_ptr = dir_ptr.subdirectories().iter().find(|x| x.name() == "home").unwrap();
        assert_eq!(dir_ptr.name(), "home");
        dir_ptr = dir_ptr.subdirectories().iter().find(|x| x.name() == "user").unwrap();
        assert_eq!(dir_ptr.name(), "user");
        assert_eq!(dir_ptr.subdirectories().len(), 1);
        dir_ptr = dir_ptr.subdirectories().iter().find(|x| x.name() == "dir").unwrap();
        assert_eq!(dir_ptr.name(), "dir");
        assert_eq!(dir_ptr.files().len(), 1);
        assert_eq!(dir_ptr.files()[0].name(), "file1");

        assert_eq!(mounted.size(), 2048);
    }
}