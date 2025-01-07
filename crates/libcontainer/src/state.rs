use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct State {
    #[serde(rename = "ociVersion")]
    pub version: String,
    pub id: String,
    pub status: ContainerState,
    pub pid: Option<i32>,
    pub bundle: String,
    pub annotations: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
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
