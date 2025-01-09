use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Result, path::Path};

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(rename = "ociVersion")]
    pub version: String,
    pub id: String,
    pub status: ContainerState,
    pub pid: Option<i32>,
    pub bundle: String,
    pub annotations: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub enum ContainerState {
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
}

impl State {
    pub fn new<P: AsRef<Path>>(container_id: &str, bundle_path: P) -> Self {
        let bundle_path = std::fs::canonicalize(bundle_path).unwrap();

        State {
            version: "1.0.0".to_string(), // TODO: implement
            id: container_id.to_string(),
            status: ContainerState::Creating,
            pid: None,
            bundle: bundle_path.to_str().unwrap().to_string(),
            annotations: None, // TODO: implement
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file = std::fs::File::open(path)?;
        let state: State = serde_json::from_reader(file)?;

        Ok(state)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let file = std::fs::File::create(path)?;
        serde_json::to_writer(file, self)?;

        Ok(())
    }
}
