use std::cell::RefCell;
use std::io::{Read, Result, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

pub struct UnixSocket {
    listener: UnixListener,
}

impl UnixSocket {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if path.exists() {
            std::fs::remove_file(path)?;
        }
        let listener = UnixListener::bind(path)?;

        Ok(Self { listener })
    }

    pub fn connect(&self) -> Result<(UnixSocketConnection, UnixSocketConnection)> {
        let client_stream = UnixStream::connect_addr(&self.listener.local_addr()?)?;
        let client = UnixSocketConnection::new(client_stream);

        let (server_stream, _) = self.listener.accept()?;
        let server = UnixSocketConnection::new(server_stream);

        Ok((server, client))
    }
}

pub struct UnixSocketConnection {
    stream: RefCell<UnixStream>,
}

impl UnixSocketConnection {
    fn new(stream: UnixStream) -> Self {
        Self {
            stream: stream.into(),
        }
    }

    pub fn send(&self, message: &str) -> Result<()> {
        let mut stream = self.stream.borrow_mut();
        stream.write_all(message.as_bytes())?;
        stream.flush()
    }

    pub fn receive(&self) -> Result<String> {
        let mut stream = self.stream.borrow_mut();
        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..size]).to_string())
    }
}
