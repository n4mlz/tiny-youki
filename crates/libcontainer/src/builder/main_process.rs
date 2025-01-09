use crate::*;
use nix::unistd;
use std::io::Result;

impl ContainerBuilder {
    pub fn main_process(&self, socket: UnixSocketConnection, child_pid: unistd::Pid) -> Result<()> {
        let message = socket.receive()?;
        if message != "ready" {
            panic!("Expected 'ready', got '{}'", message);
        }

        socket.send("Hello, world!")?;

        Ok(())
    }
}
