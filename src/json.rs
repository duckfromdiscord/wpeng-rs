use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WallpaperEngineConfigJson {
    #[serde(rename = "?installdirectory")]
    pub install_directory: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectJson {
    pub description: Option<String>,
    pub file: String,
    pub title: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub workshopid: Option<String>,
}
