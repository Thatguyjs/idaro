use std::path::PathBuf;


pub struct WebOptions {
    pub host_path: PathBuf
}

impl Default for WebOptions {
    fn default() -> Self {
        WebOptions { host_path: "./src".into() }
    }
}
