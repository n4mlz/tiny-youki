use crate::*;
use nix::unistd::{setgid, setuid};
use oci_spec::runtime::LinuxNamespaceType;
use std::{
    ffi::OsString,
    io::{Error, ErrorKind, Result},
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

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

        mounter.setup()?;

        if self.config.process().is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "no process defined"));
        }
        let process = self.config.process().as_ref().unwrap();

        setuid(process.user().uid().into())?;
        setgid(process.user().gid().into())?;

        let process_args = process.args().as_ref().unwrap();
        let mut cmd = Command::new(&process_args[0]);

        let envs = process
            .env()
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(|entry| {
                let mut split = entry.splitn(2, '=');
                if let (Some(key), Some(value)) = (split.next(), split.next()) {
                    Some((OsString::from(key), OsString::from(value)))
                } else {
                    None
                }
            });

        cmd.args(&process_args[1..])
            .envs(envs)
            .current_dir(process.cwd())
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        // TODO: wait for start signal
        cmd.exec();

        socket.send("created")?;

        Ok(())
    }
}
