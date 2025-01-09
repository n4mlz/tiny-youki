use crate::*;
use std::{
    io::Result,
    path::{Path, PathBuf},
};

pub struct Container {
    pub state: State,
    pub root: PathBuf,
}

impl Container {
    pub fn new<P: AsRef<Path>>(container_id: &str, bundle_path: P) -> Result<Self> {
        let bundle_path = bundle_path.as_ref();

        let root = PathBuf::from(BASE_PATH)
            .join(CONTAINER_PATH)
            .join(container_id);

        if root.exists() {
            panic!("Container {} already exists", container_id);
        }

        let new_container = Container {
            state: State::new(container_id, bundle_path),
            root,
        };

        new_container.save()?;

        Ok(new_container)
    }

    pub fn save(&self) -> Result<()> {
        std::fs::create_dir_all(&self.root)?;
        let file_path = self.root.join("state.json");
        self.state.save(file_path)?;

        Ok(())
    }
}
