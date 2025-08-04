use serde::{Deserialize, Serialize};
use crate::projects::models::project::Project;

pub use crate::projects::actions::project_cleaner::CleaningRequest;
pub use crate::projects::actions::project_compressor::CompressionRequest;
pub use crate::projects::actions::project_discovery::DiscoveryRequest;
pub use crate::projects::actions::project_packager::PackageRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInitializedPayload {
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDiscoveryRequest {
    pub base_folder: String,
    pub ignore_engine: bool,
    pub ignore_templates: bool,
    pub ignore_samples: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDiscoveryResult {
    pub projects: Vec<Project>,
    pub total_found: usize,
    pub scan_duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectsUpdatedPayload {
    pub projects: Vec<Project>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgressPayload {
    pub task_id: String,
    pub task_name: String,
    pub progress: f32, // 0.0 to 1.0
    pub status: TaskStatus,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Started,
    InProgress,
    Completed,
    Failed,
}