// A simple TCP listener which can be shut down

use socket2::{Socket, Domain, Type};

use std::{
    io,
    net::{SocketAddr, Shutdown, TcpStream},
    thread::{self, JoinHandle},
    sync::{Arc, mpsc::{self, Receiver, Sender}, Mutex}
};


enum Event {
    Stream(io::Result<TcpStream>),
    Shutdown
}


pub struct TcpListener {
    // addr: SocketAddr,
    socket: Arc<Socket>,
    listen_handle: Option<JoinHandle<()>>,

    send: Arc<Mutex<Sender<Event>>>,
    recv: Receiver<Event>
}

impl TcpListener {
    pub fn bind(addr: SocketAddr) -> io::Result<(Self, TcpShutdown)> {
        let sk = Socket::new(Domain::IPV4, Type::STREAM, None)?;
        sk.set_reuse_address(true)?;
        sk.bind(&addr.into())?;
        let sk = Arc::new(sk);

        let (send, recv) = mpsc::channel();

        Ok((
            TcpListener {
                // addr,
                socket: sk.clone(),
                listen_handle: None,

                send: Arc::new(Mutex::new(send)),
                recv
            },
            TcpShutdown(sk.clone())
        ))
    }

    pub fn listen(&mut self) -> io::Result<()> {
        self.socket.listen(32)?;

        let thread_send = self.send.clone();
        let thread_sk = self.socket.clone();

        self.listen_handle = Some(thread::spawn(move || {
            loop {
                match thread_sk.accept() {
                    Ok((sk, _)) => {
                        thread_send.lock().unwrap().send(Event::Stream(Ok(TcpStream::from(sk)))).unwrap();
                    },
                    Err(e) => {
                        if e.kind() == io::ErrorKind::InvalidInput {
                            thread_send.lock().unwrap().send(Event::Shutdown).unwrap();
                            break;
                        }

                        thread_send.lock().unwrap().send(Event::Stream(Err(e))).unwrap();
                    }
                }
            }
        }));

        Ok(())
    }

    pub fn incoming(&self) -> TcpIncoming {
        TcpIncoming(&self.recv)
    }

    pub fn wait(&mut self) {
        match self.listen_handle.take() {
            Some(handle) => {
                handle.join().unwrap();
            },
            None => ()
        }
    }
}


pub struct TcpIncoming<'a>(&'a Receiver<Event>);

impl<'a> Iterator for TcpIncoming<'a> {
    type Item = io::Result<TcpStream>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.recv().unwrap() {
            Event::Stream(stream) => Some(stream),
            Event::Shutdown => None
        }
    }
}


pub struct TcpShutdown(Arc<Socket>);

impl TcpShutdown {
    pub fn shutdown(&self) -> io::Result<()> {
        self.0.shutdown(Shutdown::Both)
    }
}
