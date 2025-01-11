use nix::sched::{unshare, CloneFlags};
use oci_spec::runtime::{LinuxNamespace, LinuxNamespaceType};
use std::io::{Error, Result};

fn get_clone_flag(ns_type: &LinuxNamespaceType) -> Result<CloneFlags> {
    let flag = match ns_type {
        LinuxNamespaceType::User => CloneFlags::CLONE_NEWUSER,
        LinuxNamespaceType::Pid => CloneFlags::CLONE_NEWPID,
        LinuxNamespaceType::Uts => CloneFlags::CLONE_NEWUTS,
        LinuxNamespaceType::Ipc => CloneFlags::CLONE_NEWIPC,
        LinuxNamespaceType::Network => CloneFlags::CLONE_NEWNET,
        LinuxNamespaceType::Cgroup => CloneFlags::CLONE_NEWCGROUP,
        LinuxNamespaceType::Mount => CloneFlags::CLONE_NEWNS,
        LinuxNamespaceType::Time => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Time namespace is not supported",
            ))
        }
    };

    Ok(flag)
}

pub struct Namespaces(Vec<LinuxNamespace>);

impl TryFrom<Option<&Vec<LinuxNamespace>>> for Namespaces {
    type Error = Error;

    fn try_from(namespaces: Option<&Vec<LinuxNamespace>>) -> Result<Self> {
        let namespaces = namespaces.unwrap_or(&Vec::new()).to_vec();

        Ok(Namespaces(namespaces))
    }
}

impl Namespaces {
    pub fn iter(&self) -> std::slice::Iter<LinuxNamespace> {
        self.0.iter()
    }

    pub fn contains(&self, ns_type: &LinuxNamespaceType) -> bool {
        self.0.iter().any(|ns| ns.typ() == *ns_type)
    }

    pub fn get(&self, ns_type: &LinuxNamespaceType) -> Option<&LinuxNamespace> {
        self.0.iter().find(|ns| ns.typ() == *ns_type)
    }

    pub fn apply_namespace(&self, ns_type: &LinuxNamespaceType) -> Result<()> {
        if !self.contains(ns_type) {
            return Ok(());
        }

        let namespace = self.get(ns_type).unwrap();

        match namespace.path() {
            Some(_) => {
                // TODO: implement setns
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "setns is not implemented",
                ));
            }
            None => {
                // TODO: implement unshare for cgroup and network
                if ns_type == &LinuxNamespaceType::Cgroup || ns_type == &LinuxNamespaceType::Network
                {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        "unshare is not implemented for cgroup and network",
                    ));
                }

                let flag = get_clone_flag(ns_type)?;

                unshare(flag)?;
            }
        }

        Ok(())
    }

    pub fn apply_namespaces<F: Fn(&LinuxNamespaceType) -> bool>(&self, filter: F) -> Result<()> {
        let namespaces = self.0.iter().filter(|ns| filter(&ns.typ()));

        for ns in namespaces {
            self.apply_namespace(&ns.typ())?;
        }

        Ok(())
    }
}
