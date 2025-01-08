use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize)]
pub struct State {
    #[serde(rename = "ociVersion")]
    pub version: String,
    pub id: String,
    pub status: ContainerState,
    pub pid: Option<i32>,
    pub bundle: String,
    pub annotations: Option<HashMap<String, String>>,
}

#[derive(Serialize)]
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
    pub fn new(container_id: &str, bundle_path: &PathBuf) -> Self {
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
}
