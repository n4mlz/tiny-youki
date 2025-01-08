use crate::*;
use std::path::PathBuf;

pub struct Container {
    pub state: State,
    pub root: PathBuf,
}

impl Container {
    pub fn new(container_id: &str, bundle_path: &PathBuf) -> Self {
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

        new_container.save();

        new_container
    }

    pub fn save(&self) {
        std::fs::create_dir_all(&self.root).unwrap();
        let file_path = self.root.join("state.json");
        let file = std::fs::File::create(file_path).unwrap();
        serde_json::to_writer(file, &self.state).unwrap();
    }
}
