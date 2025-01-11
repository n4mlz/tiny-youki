use crate::*;
use oci_spec::runtime::LinuxNamespaceType;
use std::io::Result;

impl ContainerBuilder {
    pub fn init_process(&self) -> Result<()> {
        let namespaces = self
            .config
            .linux()
            .as_ref()
            .and_then(|l| l.namespaces().as_ref());

        let namespaces = Namespaces::try_from(namespaces)?;

        namespaces.apply_namespaces(|ns_type| {
            ns_type != &LinuxNamespaceType::User && ns_type != &LinuxNamespaceType::Pid
        })?;

        Ok(())
    }
}
