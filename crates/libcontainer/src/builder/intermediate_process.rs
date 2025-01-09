use crate::*;
use std::io::Result;

impl ContainerBuilder {
    pub fn intermediate_process(socket: UnixSocketConnection) -> Result<()> {
        socket.send("Hello, world!")?;

        let message = socket.receive()?;
        println!("Received message: {}", message);

        Ok(())
    }
}
