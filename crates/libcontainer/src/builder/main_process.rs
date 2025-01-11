use crate::*;
use nix::unistd;
use std::{io::Result, process::Command};

impl ContainerBuilder {
    pub fn main_process(
        &mut self,
        socket: UnixSocketConnection,
        child_pid: unistd::Pid,
    ) -> Result<()> {
        let message = socket.receive()?;
        if message != "ready" {
            panic!("Expected 'ready', got '{}'", message);
        }

        let linux = self.config.linux().as_ref();

        if linux.is_none() {
            return Ok(());
        }

        if let Some(uid_mappings) = linux.unwrap().uid_mappings() {
            let mut uid_mapping_args = vec![child_pid.to_string()];

            for m in uid_mappings.iter() {
                uid_mapping_args.push(m.container_id().to_string());
                uid_mapping_args.push(m.host_id().to_string());
                uid_mapping_args.push(m.size().to_string());
            }

            let output = Command::new(NEWUIDMAP).args(&uid_mapping_args).output()?;

            if !output.status.success() {
                panic!(
                    "Failed to set uid mappings: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }

        if let Some(gid_mappings) = linux.unwrap().gid_mappings() {
            let mut gid_mapping_args = vec![child_pid.to_string()];

            for m in gid_mappings.iter() {
                gid_mapping_args.push(m.container_id().to_string());
                gid_mapping_args.push(m.host_id().to_string());
                gid_mapping_args.push(m.size().to_string());
            }

            let output = Command::new(NEWGIDMAP).args(&gid_mapping_args).output()?;

            if !output.status.success() {
                panic!(
                    "Failed to set gid mappings: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }

        socket.send("done")?;

        let message = socket.receive()?;
        if !message.starts_with("init:") {
            panic!("Expected 'init:', got '{}'", message);
        }

        let init_pid: i32 = message.strip_prefix("init: ").unwrap().parse().unwrap();
        self.container.as_mut().unwrap().set_pid(init_pid)?;

        let message = socket.receive()?;
        if message != "created" {
            panic!("Expected 'created', got '{}'", message);
        }

        self.container
            .as_mut()
            .unwrap()
            .update_status(ContainerState::Created)?;

        Ok(())
    }
}
