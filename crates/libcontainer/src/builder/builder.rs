use crate::*;
use nix::unistd;
use oci_spec::runtime::Spec;
use std::{
    io::Result,
    path::{Path, PathBuf},
};

pub struct ContainerBuilder {
    pub bundle_path: PathBuf,
    pub container: Option<Container>,
    pub config: Spec,
}

impl ContainerBuilder {
    pub fn new<P: AsRef<Path>>(bundle_path: P) -> Result<Self> {
        let bundle_path = bundle_path.as_ref();
        let config = Spec::load(bundle_path.join("config.json"))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        Ok(ContainerBuilder {
            bundle_path: bundle_path.to_path_buf(),
            container: None,
            config,
        })
    }

    pub fn create(&mut self, container_id: &str) -> Result<()> {
        let container = Container::new(container_id, &self.bundle_path)?;
        self.container = Some(container);

        let socket_path = self.container.as_ref().unwrap().root.join("notify.sock");

        let unix_socket = UnixSocket::new(&socket_path)?;
        let (server, client) = unix_socket.connect()?;

        match unsafe { unistd::fork()? } {
            unistd::ForkResult::Parent { child } => {
                self.main_process(client, child)?;
            }

            unistd::ForkResult::Child => {
                self.intermediate_process(server)?;
            }
        }

        Ok(())
    }
}
