use nix::mount::{mount, MsFlags};
use oci_spec::runtime::Mount;
use std::{
    fs::{copy, create_dir_all, read_link},
    io::Result,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

fn copy_dir_all<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    create_dir_all(dst)?;

    for entry in src.read_dir()? {
        let entry = entry?;
        let entry_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if entry_path.is_symlink() {
            let target = read_link(&entry_path)?;
            symlink(target, dst_path)?;
        } else if entry_path.is_dir() {
            copy_dir_all(entry_path, dst_path)?;
        } else if entry_path.is_file() {
            copy(entry_path, dst_path)?;
        } else {
            // TODO: handle other file types
            println!("unsupported file type: {:?}", entry_path);
        }
    }

    Ok(())
}

fn perse_options(options: &Vec<String>) -> (MsFlags, String) {
    let mut flags = MsFlags::empty();
    let mut data = Vec::new();

    for option in options {
        match option.to_string().as_str() {
            // TODO: confirm the correctness of the following mappings
            // TODO: add more mappings
            "bind" => flags |= MsFlags::MS_BIND,
            "rbind" => flags |= MsFlags::MS_BIND | MsFlags::MS_REC,
            "ro" => flags |= MsFlags::MS_RDONLY,
            "rro" => flags |= MsFlags::MS_RDONLY,
            "nosuid" => flags |= MsFlags::MS_NOSUID,
            "rnosuid" => flags |= MsFlags::MS_NOSUID | MsFlags::MS_REC,
            "nodev" => flags |= MsFlags::MS_NODEV,
            "noexec" => flags |= MsFlags::MS_NOEXEC,
            "rnoexec" => flags |= MsFlags::MS_NOEXEC | MsFlags::MS_REC,
            "sync" => flags |= MsFlags::MS_SYNCHRONOUS,
            "remount" => flags |= MsFlags::MS_REMOUNT,
            "dirsync" => flags |= MsFlags::MS_DIRSYNC,
            "noatime" => flags |= MsFlags::MS_NOATIME,
            "rnoatime" => flags |= MsFlags::MS_NOATIME | MsFlags::MS_REC,
            "unbundable" => flags |= MsFlags::MS_UNBINDABLE,
            "runbindable" => flags |= MsFlags::MS_UNBINDABLE | MsFlags::MS_REC,
            "private" => flags |= MsFlags::MS_PRIVATE,
            "rprivate" => flags |= MsFlags::MS_PRIVATE | MsFlags::MS_REC,
            "slave" => flags |= MsFlags::MS_SLAVE,
            "rslave" => flags |= MsFlags::MS_SLAVE | MsFlags::MS_REC,
            "shared" => flags |= MsFlags::MS_SHARED,
            "rshared" => flags |= MsFlags::MS_SHARED | MsFlags::MS_REC,
            "iversion" => flags |= MsFlags::MS_I_VERSION,
            "strictatime" => flags |= MsFlags::MS_STRICTATIME,
            "rstictatime" => flags |= MsFlags::MS_STRICTATIME | MsFlags::MS_REC,
            "lazytime" => flags |= MsFlags::MS_LAZYTIME,
            "nodiratime" => flags |= MsFlags::MS_NODIRATIME,
            _ => data.push(option.to_string()),
        }
    }

    (flags, data.join(","))
}

pub struct Mounter {
    bundle_root: PathBuf,
    new_root: PathBuf,
    mounts: Vec<Mount>,
}

impl Mounter {
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>>(
        bundle_root: P,
        new_root: Q,
        mounts: Vec<Mount>,
    ) -> Self {
        Mounter {
            bundle_root: bundle_root.as_ref().to_path_buf(),
            new_root: new_root.as_ref().to_path_buf(),
            mounts,
        }
    }

    fn setup_mount(&self) -> Result<()> {
        for m in self.mounts.iter() {
            let source = self
                .bundle_root
                .join(m.source().as_deref().unwrap_or(Path::new("")));

            let target = self
                .new_root
                .join(m.destination().strip_prefix("/").unwrap());

            create_dir_all(target.as_path())?;

            let (flags, data) = if let Some(options) = m.options() {
                perse_options(options)
            } else {
                (MsFlags::empty(), "".to_string())
            };

            // TODO: implement mount for cgroup
            if m.typ().as_ref().unwrap() == "cgroup" {
                println!("mount for cgroup has not yet been implemented");
                continue;
            }

            mount(
                Some(source.as_path()),
                target.as_path(),
                m.typ().as_deref(),
                flags,
                Some(data.as_str()),
            )?;
        }

        Ok(())
    }

    pub fn setup_rootfs(&self) -> Result<()> {
        // recursively copy the rootfs from bundle to new_root
        copy_dir_all(&self.bundle_root, &self.new_root)?;

        // mount as private
        mount::<str, str, str, str>(
            Some("none"),
            "/",
            None,
            MsFlags::MS_REC | MsFlags::MS_PRIVATE,
            None,
        )?;

        // mount rootfs
        mount::<Path, Path, str, str>(
            Some(self.new_root.as_path()),
            self.new_root.as_path(),
            None,
            MsFlags::MS_BIND | MsFlags::MS_REC,
            None,
        )?;

        // setup mounts
        self.setup_mount()
    }
}
