use std::collections::HashMap;
use async_trait::async_trait;
use crate::combinable_dir::CombinableDir;
use crate::rcu::ReadCopyUpdate;
use crate::static_combinable::{StaticCombinableFile, StaticDownloadLinkFile};

#[async_trait]
pub trait CloudDriver<Config, State> {
    /// Create a new instance of the driver.
    fn new(state: State) -> Self;

    fn get_rcu(&self) -> &ReadCopyUpdate<CombinableDir<StaticCombinableFile>>;
    fn get_state(&self) -> &State;

    /// refresh the VFS
    async fn refresh(&self) {
        let state = self.get_state();
        let new_vfs = Self::reload_vfs(state).await;
        match new_vfs {
            Ok(vfs) => {
                let rcu = self.get_rcu();
                rcu.update(vfs);
            }
            Err(_) => {}
        }
    }

    /// If `Config` is `State`, returns `Config` directly.
    async fn load_config(config: Config) -> State;

    async fn reload_vfs(state: &State) -> Result<CombinableDir<StaticCombinableFile>, String>;
}
