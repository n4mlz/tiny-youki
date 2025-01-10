use nix::{
    sched::{unshare, CloneFlags},
    unistd,
};
use oci_spec::runtime::LinuxNamespaceType;

use crate::*;
use std::io::Result;

impl ContainerBuilder {
    pub fn intermediate_process(&self, socket: UnixSocketConnection) -> Result<()> {
        let namespaces = self
            .config
            .linux()
            .as_ref()
            .and_then(|l| l.namespaces().as_ref());

        if namespaces.is_none() {
            return Ok(());
        }

        if namespaces
            .unwrap()
            .iter()
            .any(|ns| ns.typ() == LinuxNamespaceType::User)
        {
            unshare(CloneFlags::CLONE_NEWUSER)?;
        }

        socket.send("ready")?;

        let message = socket.receive()?;
        if message != "done" {
            panic!("Expected 'done', got '{}'", message);
        }

        unistd::setuid(unistd::Uid::from_raw(0))?;
        unistd::setgid(unistd::Gid::from_raw(0))?;

        if namespaces
            .unwrap()
            .iter()
            .any(|ns| ns.typ() == LinuxNamespaceType::Pid)
        {
            unshare(CloneFlags::CLONE_NEWPID)?;
        }

        Ok(())
    }
}
