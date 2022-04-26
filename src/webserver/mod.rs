mod options;
mod http;

pub use options::WebOptions;

use crate::{
    listener::{TcpListener, TcpShutdown},
    parser
};

use std::{
    io::{self, Write},
    fs,
    net::{SocketAddr, TcpStream},
    path::PathBuf
};


pub struct WebServer {
    options: WebOptions,
    listener: TcpListener
}

impl WebServer {
    pub fn new<A: Into<SocketAddr>>(addr: A, options: WebOptions) -> io::Result<(Self, TcpShutdown)> {
        let (listener, shutdown) = TcpListener::bind(addr.into())?;

        Ok((
            WebServer { listener, options },
            shutdown
        ))
    }

    pub fn listen(&mut self) -> io::Result<()> {
        self.listener.listen()
    }

    pub fn handle_connections(&mut self) {
        for stream in self.listener.incoming() {
            if let Err(e) = self.handle_stream(stream.unwrap()) {
                println!("Stream Parse Error: {}", e);
            }
        }

        // Just in case the thread hasn't finished
        self.listener.wait();
    }

    fn handle_stream(&self, mut stream: TcpStream) -> io::Result<()> {
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

        let req_path = self.options.host_path.join(&req_path);

        match fs::read(&req_path) {
            Ok(data) => {
                let output: Vec<u8>;

                if is_markdown { // Parse the document
                    let data = std::str::from_utf8(&data).expect("Invalid character in document");
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
                    eprintln!("Error serving file ({:?}): {}", &req_path, e);

                    res.set_status(500).write_header("Content-Type", "text/plain");
                    res.write(b"500 Internal Server Error")?;
                }
            }
        }

        res.send()?;
        Ok(())
    }
}
