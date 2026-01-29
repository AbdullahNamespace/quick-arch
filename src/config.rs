use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub project: ProjectMeta,
    pub features: FeatureSet,
    pub directories: Vec<DirectoryItem>,
    pub files: Vec<FileItem>,
    #[serde(default)]
    pub custom_scripts: CustomScripts,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectMeta {
    pub name: String,
    #[serde(rename = "type")]
    pub project_type: Option<String>,
    pub description: Option<String>,
}

pub type FeatureSet = std::collections::HashMap<String, Value>;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CustomScripts {
    #[serde(default)]
    pub post_create: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DirectoryItem {
    Simple(String),
    Complex(ComplexItem),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FileItem {
    Simple(String),
    Complex(ComplexFile),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComplexItem {
    pub path: String,
    pub condition: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComplexFile {
    pub path: String,
    pub content: Option<String>,
    pub condition: Option<String>,
}

pub struct GenerationStats {
    pub project_name: String,
    pub output_dir: String,
    pub dirs_count: u32,
    pub files_count: u32,
    pub skipped_count: u32,
    pub scripts_to_run: Option<Vec<String>>,
}
