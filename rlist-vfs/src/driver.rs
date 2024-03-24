use async_trait::async_trait;
use serde::Deserialize;
use crate::combinable_dir::CombinableDir;
use crate::static_combinable::StaticCombinableFile;

#[async_trait]
pub trait CloudDriver<Config, State>: GetVfs
    where Config: Clone + Send + Sync + for<'a> Deserialize<'a>
{
    /// Create a new instance of the driver.
    fn new(state: State) -> Self;

    /// If `Config` is `State`, returns `Config` directly.
    async fn load_config(config: Config) -> State;

    async fn reload_vfs(state: &State) -> Result<CombinableDir<StaticCombinableFile>, String>;
}

#[async_trait]
pub trait GetVfs: Send + Sync {
    /// You **must** implement this method by calling `reload_vfs` then return the result.
    /// ## Example:
    /// ```ignore
    /// async fn get_vfs(&self) ->
    ///     Result<CombinableDir<rlist-vfs::static_combinable::StaticCombinableFile>, String> {
    ///         self.reload_vfs().await
    /// }
    /// ```
    async fn get_vfs(&self) -> Result<CombinableDir<StaticCombinableFile>, String>;
}