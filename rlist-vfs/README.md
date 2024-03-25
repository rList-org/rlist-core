# rList VFS library

Basically, it is used to implement the driver for rList and get the `Whell` as the server state.

## How to implement a driver

### Step 1: Define the driver struct

First, define the configuration struct, like this:

```rust
#[derive(Debug, Deserialize)]
pub struct OnedriveConfig {
    /// The refresh token for the onedrive account.
    /// *For further information, please refer to the official documentation of Microsoft OAuth 2.0 authorization flow.*
    pub refresh_token: String,

    /// The client id for the application.
    /// You can get it from the Azure portal with the client secret.
    pub client_id: String,

    /// The client secret for the application.
    /// You can get it from the Azure portal with the client id.
    pub client_secret: String,
}
```

You should make sure that the struct can be loaded from the config file.

Besides, you should make sure that there is no `driver_name` field in the configuration struct, because it is preserved for identifying the driver.

Don't worry about the `driver_name` field. If you add it into the config struct, there should be a compile error.

