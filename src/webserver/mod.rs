mod options;
mod http;

pub use options::WebOptions;

use crate::{
    listener::{TcpListener, TcpShutdown},
    parser,
    threadpool::Threadpool
};

use std::{
    io::{self, Write},
    fs,
    net::{SocketAddr, TcpStream},
    path::PathBuf, sync::Arc
};


pub struct WebServer {
    options: WebOptions,
    listener: TcpListener,
    threadpool: Threadpool
}

impl WebServer {
    pub fn new<A: Into<SocketAddr>>(addr: A, options: WebOptions) -> io::Result<(Self, TcpShutdown)> {
        let (listener, shutdown) = TcpListener::bind(addr.into())?;
        let threadpool = Threadpool::new(options.threads);

        Ok((
            WebServer { listener, options, threadpool },
            shutdown
        ))
    }

    pub fn handle_connections(self) {
        let host_path = Arc::new(self.options.host_path);

        for stream in self.listener.incoming() {
            let thread_host_path = host_path.clone();

            self.threadpool.execute(move || {
                if let Err(e) = Self::handle_stream(thread_host_path, stream.unwrap()) {
                    eprintln!("Stream Parse Error: {}", e);
                }
            });
        }
    }

    fn handle_stream(host_path: Arc<PathBuf>, mut stream: TcpStream) -> io::Result<()> {
        let req = http::Request::parse(&mut stream)?;
        let mut res = http::Response::new(&stream);

        let mut req_path = PathBuf::from(&req.path[1..]);
        let mut is_markdown = false;

        match req_path.extension() {
            Some(ext) if ext == "html" => {
                req_path.set_extension("md");
                is_markdown = true;
            },
            None => {
                req_path.push("index.md");
                is_markdown = true;
            },
            _ => ()
        }

        let req_path = host_path.join(&req_path);

        match fs::read(&req_path) {
            Ok(data) => {
                let output: Vec<u8>;

                if is_markdown { // Parse the document
                    let data = std::str::from_utf8(&data).map_err(|err|
                        io::Error::new(io::ErrorKind::Other, err)
                    )?;
                    output = parser::parse(data).into_bytes();
                }
                else { // No parsing
                    output = data;
                }

                res.set_status(200);

                if let Some(ext) = req_path.extension() {
                    res.write_header("Content-Type", http::mime_type(ext.to_str().unwrap()));
                }

                res.write(&output)?;
            },
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    res.set_status(404).write_header("Content-Type", "text/plain");
                    res.write(b"404 Not Found")?;
                }
                else {
                    eprintln!("Error serving file ({}): {}", &req_path.display(), e);

                    res.set_status(500).write_header("Content-Type", "text/plain");
                    res.write(b"500 Internal Server Error")?;
                }
            }
        }

        res.send()?;
        Ok(())
    }
}
