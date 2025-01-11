use crate::*;
use nix::unistd;
use oci_spec::runtime::LinuxNamespaceType;
use std::io::Result;

impl ContainerBuilder {
    pub fn intermediate_process(&self, socket: UnixSocketConnection) -> Result<()> {
        let namespaces = self
            .config
            .linux()
            .as_ref()
            .and_then(|l| l.namespaces().as_ref());

        let namespaces = Namespaces::try_from(namespaces)?;

        namespaces.apply_namespace(&LinuxNamespaceType::User)?;

        socket.send("ready")?;

        let message = socket.receive()?;
        if message != "done" {
            panic!("Expected 'done', got '{}'", message);
        }

        unistd::setuid(unistd::Uid::from_raw(0))?;
        unistd::setgid(unistd::Gid::from_raw(0))?;

        namespaces.apply_namespace(&LinuxNamespaceType::Pid)?;

        if let unistd::ForkResult::Child = unsafe { unistd::fork()? } {
            self.init_process()?;
        }

        Ok(())
    }
}
