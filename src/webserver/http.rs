use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::TcpStream
};


pub fn mime_type(ext: &str) -> &str {
    match ext {
        "md" => "text/html",
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "mjs" => "application/javascript",

        "png" => "image/png",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "ico" => "image/x-icon",

        _ => "text/plain"
    }
}


#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl Request {
    fn parse_start_line(data: &str) -> io::Result<(String, String, String)> {
        match data.lines().next() {
            Some(line) => {
                let parts: Vec<&str> = line.split(' ').collect();
                Ok((parts[0].to_uppercase(), parts[1].into(), parts[2].into()))
            },
            None => Err(io::Error::new(io::ErrorKind::Other, "Invalid HTTP request: Invalid start line"))
        }
    }

    fn parse_headers(data: &str) -> io::Result<HashMap<String, String>> {
        let mut headers = HashMap::new();

        for line in data.lines().skip(1) {
            let (header, val) = line.split_once(':').unwrap(); // TODO: Error handling
            headers.insert(header.trim().to_lowercase(), val.trim().into());
        }

        Ok(headers)
    }

    pub fn parse(stream: &mut TcpStream) -> io::Result<Self> {
        let mut data = [0u8; 4096];
        let data_len = stream.read(&mut data)?;
        let data = std::str::from_utf8(&data[0..data_len]).unwrap();

        let start_line = Self::parse_start_line(data)?;

        let (header_data, body_data) = match data.split_once("\r\n\r\n") {
            Some(vals) => vals,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Invalid HTTP request: No header end"))
        };

        let req = Request {
            method: start_line.0,
            path: start_line.1,
            version: start_line.2,
            headers: Self::parse_headers(header_data)?,
            body: body_data.into()
        };

        Ok(req)
    }
}


#[derive(Debug)]
pub struct Response<'a> {
    stream: &'a TcpStream,
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

impl<'a> Write for Response<'a> {
    fn write(&mut self, data: &[u8]) -> Result<usize, io::Error> {
        self.body.write(data)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.body.flush()
    }
}

impl<'a> Response<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Response {
            stream,
            status: 200,
            headers: HashMap::new(),
            body: Vec::new()
        }
    }

    pub fn set_status(&mut self, status: u16) -> &mut Self {
        self.status = status;
        self
    }

    pub fn write_header(&mut self, header: &str, value: &str) -> &mut Self {
        self.headers.insert(header.into(), value.into());
        self
    }

    fn status_message(status: u16) -> String {
        match status {
            200 => "Ok",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => ""
        }.into()
    }

    fn send_headers(&mut self) -> Result<(), io::Error> {
        let mut header_string = format!("HTTP/1.1 {} {}\r\n", self.status, Self::status_message(self.status));

        for (header, val) in &self.headers {
            header_string += &format!("{}: {}\r\n", header, val);
        }

        header_string += "\r\n";

        self.stream.write(header_string.as_bytes())?;
        Ok(())
    }

    pub fn send(&mut self) -> Result<(), io::Error> {
        self.send_headers()?;
        self.stream.write(&self.body)?;
        self.stream.flush()?;

        Ok(())
    }
}
