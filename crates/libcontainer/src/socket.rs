use std::io::{Read, Result, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct UnixSocketServer {
    listener: UnixListener,
}

impl UnixSocketServer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            std::fs::remove_file(&path)?;
        }
        let listener = UnixListener::bind(path)?;

        Ok(Self { listener })
    }

    pub fn accept(&self) -> Result<UnixSocketConnection> {
        let (stream, _) = self.listener.accept()?;

        Ok(UnixSocketConnection::new(stream))
    }
}

pub struct UnixSocketClient;

impl UnixSocketClient {
    pub fn connect<P: AsRef<Path>>(path: P) -> Result<UnixSocketConnection> {
        let stream = UnixStream::connect(path)?;

        Ok(UnixSocketConnection::new(stream))
    }
}

pub struct UnixSocketConnection {
    stream: Arc<Mutex<UnixStream>>,
}

impl UnixSocketConnection {
    fn new(stream: UnixStream) -> Self {
        Self {
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    pub fn send(&self, message: &str) -> Result<()> {
        let mut stream = self.stream.lock().unwrap();
        stream.write_all(message.as_bytes())?;
        stream.flush()
    }

    pub fn receive(&self) -> Result<String> {
        let mut stream = self.stream.lock().unwrap();
        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..size]).to_string())
    }
}
