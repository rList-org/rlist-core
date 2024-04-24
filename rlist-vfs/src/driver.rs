use crate::combinable_dir::CombinableDir;
use crate::static_combinable::StaticCombinableFile;
use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
/// # Main trait of the driver
///
/// The trait means that the driver can
/// 1. load `Config` and get `State`
/// 2. keep the `State`
/// 3. get the VFS from the `State`
///
/// ## `Config`
///
/// The configuration of the driver. The `Config` will be read from the configuration file directly.
///
/// For example, the API key of the cloud storage can be the `Config`.
///
/// ## `State`
///
/// The **immutable** state of the driver. For example, the connection to the cloud storage.
///
/// Of course, you can just copy `Config` to `State` if you don't need a state.
pub trait CloudDriver<Config, State>: GetVfs
where
    Config: Clone + Send + Sync + for<'a> Deserialize<'a>,
{
    /// Create a new instance of the driver.
    async fn new(state: State) -> Self;

    /// If `Config` is `State`, returns `Config` directly.
    async fn load_config(config: Config) -> State;

    async fn reload_vfs(state: &State) -> Result<CombinableDir<StaticCombinableFile>, String>;
}

#[async_trait]
/// The trait means that the driver **instance** can get the VFS from state by calling `get_vfs` function.
///
/// You **must** implement this trait by calling `reload_vfs` then return the result.
///
/// You should implement this trait by using `#[derive(GetVfs)]`.
pub trait GetVfs: Send + Sync {
    /// You should implement this trait by using `#[derive(GetVfs)]`.
    async fn get_vfs(&self) -> Result<CombinableDir<StaticCombinableFile>, String>;
}
