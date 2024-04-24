use crate::combinable::Combinable;
use crate::combinable_dir::CombinableDir;
use crate::driver::GetVfs;
use crate::rcu::ReadCopyUpdate;
use crate::static_combinable::StaticCombinableFile;
use crate::without_link::DirWithoutLink;
use futures::future::join_all;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{self, Duration};

pub struct Wheel {
    pub drivers: Vec<Box<dyn GetVfs>>,
    pub path_map: ReadCopyUpdate<HashMap<String, StaticCombinableFile>>,
    pub tree: ReadCopyUpdate<String>,
}

impl Wheel {
    pub async fn new(drivers: Vec<Box<dyn GetVfs>>) -> Arc<Self> {
        let dirs = join_all(drivers.iter().map(|x| x.get_vfs()).collect::<Vec<_>>()).await;
        let dirs = dirs
            .into_iter()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        let combined = CombinableDir::combine(dirs);
        let combined_clone = combined.clone();
        let path_map = ReadCopyUpdate::new(combined.compress_path());
        let tree: DirWithoutLink = combined_clone.into();
        let tree = serde_json::to_string(&tree).unwrap();
        let tree = ReadCopyUpdate::new(tree);
        Self {
            drivers,
            path_map,
            tree,
        }
        .set_refresh_interval()
    }

    async fn refresh(&self) {
        let dirs = join_all(self.drivers.iter().map(|x| x.get_vfs()).collect::<Vec<_>>()).await;
        let dirs = dirs
            .into_iter()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        let combined = CombinableDir::combine(dirs);
        let combined_clone = combined.clone();
        let new_path_map = combined.compress_path();
        let new_tree: DirWithoutLink = combined_clone.into();
        let new_tree = serde_json::to_string(&new_tree).unwrap();
        self.path_map.update(new_path_map);
        self.tree.update(new_tree);
    }

    fn set_refresh_interval(self) -> Arc<Self> {
        let arc_self = Arc::new(self);
        let arc_self_clone = arc_self.clone();
        tokio::spawn(async move {
            loop {
                time::sleep(Duration::from_secs(60)).await;
                arc_self_clone.refresh().await;
            }
        });
        arc_self
    }
}
