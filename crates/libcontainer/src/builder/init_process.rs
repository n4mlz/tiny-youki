use crate::*;
use oci_spec::runtime::LinuxNamespaceType;
use std::io::Result;

impl ContainerBuilder {
    pub fn init_process(&self, socket: UnixSocketConnection) -> Result<()> {
        let namespaces = self
            .config
            .linux()
            .as_ref()
            .and_then(|l| l.namespaces().as_ref());

        let namespaces = Namespaces::try_from(namespaces)?;

        namespaces.apply_namespaces(|ns_type| {
            ns_type != &LinuxNamespaceType::User && ns_type != &LinuxNamespaceType::Pid
        })?;

        setup_uts(&self.config)?;

        let mounter = Mounter::new(
            self.bundle_path
                .join(self.config.root().as_ref().unwrap().path()),
            self.container.as_ref().unwrap().root.join("rootfs"),
            self.config.mounts().as_ref().unwrap().to_vec(),
        );

        mounter.setup_rootfs()?;

        socket.send("created")?;

        Ok(())
    }
}
