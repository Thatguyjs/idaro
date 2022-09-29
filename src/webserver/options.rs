use std::path::PathBuf;


pub struct WebOptions {
    pub host_path: PathBuf,
    pub threads: usize
}

impl Default for WebOptions {
    fn default() -> Self {
        let threads = match std::thread::available_parallelism() {
            Ok(n) => n.get(),
            Err(_) => 4
        };

        WebOptions {
            host_path: "./src".into(),
            threads
        }
    }
}
