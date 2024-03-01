use std::cell::UnsafeCell;
use std::sync::{Arc};
use std::time::Duration;
use tokio::time::interval;
use crate::vfs::combine::{CombinableVfsDir, CombinableVfsFile, combine_vfs_dirs};
use crate::vfs::hide_url::{hide_url_for_dir, UrlHiddenDir};
use crate::vfs::path_compress::IndexedVfs;


type PathMap = IndexedVfs<CombinableVfsFile, CombinableVfsDir>;
pub struct DriveWheel {
    path_map: UnsafeCell<Arc<PathMap>>,
    hidden_url: UnsafeCell<Arc<UrlHiddenDir>>,
    drive_config: Vec<DriveConfig>,
    stop_signal: UnsafeCell<StopSignal>,
}


unsafe impl Send for DriveWheel {}
unsafe impl Sync for DriveWheel {}


impl DriveWheel {
    async fn new_data(drive_config: &Vec<DriveConfig>) -> (Arc<PathMap>, Arc<UrlHiddenDir>) {
        let vfs = get_vfs(drive_config).await;
        let hidden = hide_url_for_dir(&vfs);
        let compressed_path = IndexedVfs::new(vfs);
        return (
            Arc::new(compressed_path),
            Arc::new(hidden)
        );
    }
    fn refresh(&self, data: (Arc<PathMap>, Arc<UrlHiddenDir>)) {
        let (_path_map, _hidden_url) = data;
        let path_map = self.path_map.get();
        let hidden_url = self.hidden_url.get();
        unsafe {
            *path_map = _path_map;
            *hidden_url = _hidden_url;
        }
    }
    pub async fn new(drive_config: Vec<DriveConfig>, refresh_time: u64) -> Arc<DriveWheel> {
        let (compressed_path, hidden_url) = Self::new_data(&drive_config).await;
        let stop_signal = StopSignal::new();
        let instance = Arc::new(DriveWheel {
            path_map: UnsafeCell::new(compressed_path.clone()),
            hidden_url: UnsafeCell::new(hidden_url.clone()),
            drive_config,
            stop_signal,
        });
        let instance_clone = instance.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(refresh_time));
            loop {
                interval.tick().await;
                if unsafe {
                    match instance_clone.stop_signal.get().as_ref() {
                        Some(signal) => signal.is_stop(),
                        None => true
                    }
                } {
                    break;
                }
                let data = Self::new_data(&instance_clone.drive_config).await;
                instance_clone.refresh(data);
            }
        });
        return instance;
    }
    pub fn get_path_map(&self) -> Arc<PathMap> {
        unsafe {
            (&*self.path_map.get()).clone()
        }
    }
    pub fn get_hidden_url(&self) -> Arc<UrlHiddenDir> {
        unsafe {
            (&*self.hidden_url.get()).clone()
        }
    }
}

impl Drop for DriveWheel {
    fn drop(&mut self) {
        unsafe {
            self.stop_signal.get().as_mut().map(|signal| signal.stop());
        }
    }
}

struct StopSignal {
    stop: bool
}

impl StopSignal {
    pub fn new() -> UnsafeCell<Self> {
        UnsafeCell::new(StopSignal {
            stop: false
        })
    }
    pub fn is_stop(&self) -> bool {
        self.stop
    }
    pub fn stop(&mut self) {
        self.stop = true;
    }
}