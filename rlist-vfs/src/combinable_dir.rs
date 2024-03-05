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
        let last_modified = files.iter().map(|x| x.last_modified()).max().unwrap();
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
        unimplemented!()
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

fn divide_by_name<T: VfsBasicMeta> (items: Vec<T>) -> HashMap<String, Vec<T>> {
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