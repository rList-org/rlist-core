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

impl<T: StaticDownloadLinkFile> Combinable<T> for T {
    /// Combine **same** files which have different download links to one file.
    fn combine(from: Vec<Self>) -> Self {
        let destructed: Vec<(String, u64, SystemTime, Vec<String>)> = from.into_iter()
            .map(|x| x.destruct())
            .collect::<Vec<_>>();
        let new_name = destructed[0].0.clone();
        let new_size = destructed.iter().map(|x| x.1).sum();
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