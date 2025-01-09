use crate::*;
use nix::unistd;
use std::io::Result;

impl ContainerBuilder {
    pub fn main_process(socket: UnixSocketConnection, child_pid: unistd::Pid) -> Result<()> {
        let message = socket.receive()?;
        println!("Received message: {}", message);

        socket.send("Hello, world!")?;

        Ok(())
    }
}
