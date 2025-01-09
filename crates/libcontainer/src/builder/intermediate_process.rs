use nix::sched::{unshare, CloneFlags};

use crate::*;
use std::io::Result;

impl ContainerBuilder {
    pub fn intermediate_process(&self, socket: UnixSocketConnection) -> Result<()> {
        unshare(CloneFlags::CLONE_NEWUSER)?;

        socket.send("ready")?;

        let message = socket.receive()?;
        println!("Received message: {}", message);

        Ok(())
    }
}
