use std::collections::HashMap;
use std::sync::Arc;
use crate::vfs::{VfsBasicMeta, VfsDir, VfsEntry, VfsFile};
use rand::Rng;
use std::marker::Send;


#[derive(Clone)]
/// # VFS Directory
/// The implement of `VfsDir` trait. As a virtual file system directory, it contains sub directories and files.
/// `CombinableVfsDir` can be combined with other `CombinableVfsDir` to form a new `CombinableVfsDir`.
pub struct CombinableVfsDir {
    _name: String,
    _sub_dirs: Vec<CombinableVfsDir>,
    _files: Vec<CombinableVfsFile>,
    _size: u64,
}

impl CombinableVfsDir {
    pub fn new(name: String, sub_dirs: Vec<CombinableVfsDir>, files: Vec<CombinableVfsFile>, size: u64) -> Self {
        CombinableVfsDir {
            _name: name,
            _sub_dirs: sub_dirs,
            _files: files,
            _size: size,
        }
    }
}

// impl VfsDir for CombinableVfsDir
impl VfsBasicMeta for CombinableVfsDir {
    fn name(&self) -> &str {
        &self._name
    }
    fn size(&self) -> u64 {
        self._size
    }
    fn last_modified(&self) -> std::time::SystemTime {
        self._files.iter().map(|file| file.last_modified()).max().unwrap()
    }
}

impl VfsDir<CombinableVfsFile> for CombinableVfsDir {
    fn list(&self) -> Vec<VfsEntry<CombinableVfsFile, CombinableVfsDir>> {
        let mut entries: Vec<VfsEntry<_, _>> = self._sub_dirs.iter()
            .map(|dir| VfsEntry::Dir(dir.clone())).collect();
        entries.extend(self._files.iter()
            .map(|file| VfsEntry::File(file.clone())));
        entries
    }
}

#[derive(Clone)]
/// # VFS File
/// The implement of `VfsFile` trait. As a virtual file system file, it contains the download link and other meta information.
pub struct CombinableVfsFile {
    _links: Vec<String>,
    _name: String,
    _size: u64,
    _last_modified: std::time::SystemTime,
    _on_download: Arc<dyn Send + Sync + Fn() -> String>,
}

impl CombinableVfsFile {
    fn possible_on_download(&self) -> Vec<String> {
        self._links.clone()
    }
    pub fn new(links: Vec<String>, name: String, size: u64, last_modified: std::time::SystemTime) -> Self {
        let on_download = get_random_selector(links.len(), links.clone());
        CombinableVfsFile {
            _links: links,
            _name: name,
            _size: size,
            _last_modified: last_modified,
            _on_download: Arc::new(on_download),
        }
    }
}

// impl VfsFile for CombinableVfsFile
impl VfsBasicMeta for CombinableVfsFile {
    fn name(&self) -> &str {
        &self._name
    }
    fn size(&self) -> u64 {
        self._size
    }
    fn last_modified(&self) -> std::time::SystemTime {
        self._last_modified
    }
}

impl VfsFile for CombinableVfsFile {
    fn on_download(&self) -> String {
        (self._on_download)()
    }
}

/// When 2 files with same name (and same size, etc) are combined, the download link will be randomly selected from the 2 files.
fn combine_vfs_files(files: Vec<CombinableVfsFile>) -> CombinableVfsFile {
    let maybe_files: Vec<String> = files.iter()
        .map(|file| file.possible_on_download())
        .flatten().collect();
    let on_download = get_random_selector(maybe_files.len(), maybe_files.clone());
    return CombinableVfsFile {
        _links: maybe_files,
        _name: files[0].name().to_owned(),
        _size: files[0].size(),
        _last_modified: files.iter().map(|file| file.last_modified()).max().unwrap(),
        _on_download: Arc::new(on_download),
    };
}

/// High level function to get a random selector function.
fn get_random_selector<T: Clone>(n: usize, possibles: Vec<T>) -> impl Fn() -> T {
    move || {
        let index = rand::thread_rng().gen_range(0..n);
        possibles[index].clone()
    }
}

impl CombinableVfsDir {
    /// Destruct the `CombinableVfsDir` into
    /// - sub directories: `Vec<CombinableVfsDir>`
    /// - files: `Vec<CombinableVfsFile>`
    /// - size: `u64`
    /// - name: `String`
    fn destruct(self) -> (Vec<CombinableVfsDir>, Vec<CombinableVfsFile>, u64, String) {
        let sub_dirs = self._sub_dirs;
        let files = self._files;
        let size = self._size;
        let name = self._name;
        (sub_dirs, files, size, name)
    }
}

/// ### Combine some `CombinableVfsDir` into a new `CombinableVfsDir`.
/// When combined:
/// - sub directories with same name will be combined recursively.
/// - sub directories in only one of the `CombinableVfsDir` will be kept as is.
/// - files with same name will be combined into a new file with `combine_vfs_files`.
/// - files in only one of the `CombinableVfsDir` will be kept as is.
/// - the size of files which are in more than one of the `CombinableVfsDir` will be added up once, not multiple times.
/// - the new size is the sum of all files' size.
pub fn combine_vfs_dirs(dirs: Vec<CombinableVfsDir>) -> CombinableVfsDir {
    // destruct all dirs
    let dirs: Vec<(Vec<CombinableVfsDir>, Vec<CombinableVfsFile>, u64, String)> = dirs.into_iter()
        .map(|dir| dir.destruct()).collect::<Vec<_>>();
    // (sub_dirs, files, size, name)

    // select the first item's name as the name of the new dir
    let name = dirs[0].3.clone();

    // name has been selected, so we can drop it
    // size will be calculated later
    let dirs: (Vec<Vec<CombinableVfsDir>>, Vec<Vec<CombinableVfsFile>>) = dirs.into_iter()
        .map(
            |(sub_dirs, files, _, _)| {
                (sub_dirs, files)
            })
        .unzip();
    // (sub_dirs, files)

    // flatten the sub_dirs and files
    let sub_dirs = dirs.0.into_iter().flatten().collect();
    let files = dirs.1.into_iter().flatten().collect();

    // separate the sub_dirs and files by name
    let sub_dirs = separate_by_name(sub_dirs);
    let files = separate_by_name(files);
    // then combine them
    let sub_dirs: Vec<CombinableVfsDir> = sub_dirs.into_iter().map(|(_, dirs)| {
        combine_vfs_dirs(dirs)
    }).collect();
    let files: Vec<CombinableVfsFile> = files.into_iter().map(|(_, files)| {
        combine_vfs_files(files)
    }).collect();

    // calculate the size
    let size: u64 =
        files.iter()
            .map(|file| file.size()).sum::<u64>()
            +
            sub_dirs.iter()
                .map(|dir| dir.size()).sum::<u64>();

    // return the new dir
    CombinableVfsDir {
        _name: name,
        _sub_dirs: sub_dirs,
        _files: files,
        _size: size,
    }
}

fn separate_by_name<T: VfsBasicMeta>(flat: Vec<T>) -> HashMap<String, Vec<T>> {
    let mut map: HashMap<String, Vec<T>> = HashMap::new();
    for entry in flat {
        let name = entry.name().to_owned();
        if map.contains_key(&name) {
            map.get_mut(&name).unwrap().push(entry);
        } else {
            map.insert(name, vec![entry]);
        }
    }
    map
}