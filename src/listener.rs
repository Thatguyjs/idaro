// A TcpListener that can be shut down

use std::{
    io::{self, Write},
    net::{self, ToSocketAddrs},
    sync::{Arc, atomic::{AtomicBool, Ordering}}
};


pub struct TcpListener {
    listener: net::TcpListener,
    should_close: Arc<AtomicBool>
}

impl TcpListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<(Self, TcpShutdown)> {
        let listener = net::TcpListener::bind(&addr)?;

        let listener_close = Arc::new(AtomicBool::new(false));
        let shutdown_close = listener_close.clone();

        Ok((
            TcpListener {
                listener,
                should_close: listener_close
            },
            TcpShutdown::new(addr, shutdown_close)?
        ))
    }

    pub fn incoming(&self) -> TcpIncoming {
        TcpIncoming(self)
    }
}


pub struct TcpIncoming<'a>(&'a TcpListener);

impl<'a> Iterator for TcpIncoming<'a> {
    type Item = io::Result<net::TcpStream>;

    fn next(&mut self) -> Option<Self::Item> {
        let sk = self.0.listener.accept().map(|(sk, _)| sk);

        match self.0.should_close.load(Ordering::Relaxed) {
            true => None,
            false => Some(sk)
        }
    }
}


pub struct TcpShutdown {
    address: net::SocketAddr,
    should_close: Arc<AtomicBool>
}

impl TcpShutdown {
    pub fn new<A: ToSocketAddrs>(addr: A, should_close: Arc<AtomicBool>) -> io::Result<Self> {
        let address = addr.to_socket_addrs()?.next().expect("Expected a socket address for TcpShutdown");
        Ok(TcpShutdown { address, should_close })
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        self.should_close.store(true, Ordering::SeqCst);
        let mut s = net::TcpStream::connect(self.address)?;
        s.write(b"GET / HTTP/1.1\r\n\r\n")?;

        Ok(())
    }
}
