use crate::misc::progress::TaskStatus;
use crate::projects::actions::engine_discovery::DetectedEngine;
use crate::projects::actions::project_compressor::CompressionAlgorithm;
use crate::projects::models::project::Project;
use serde::{Deserialize, Serialize};

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
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EngineDiscoveryResult {
    pub engines: Vec<DetectedEngine>,
    pub total_found: usize,
    pub scan_duration_ms: u128,
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
pub struct PackageRequest {
    pub project: Project,
    pub build_type: String,
    pub target_platform: String,
    pub output_directory: String,
    pub create_archive: bool,
    pub archive_format: Option<CompressionAlgorithm>,
    pub archive_filename_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageResult {
    pub success: bool,
    pub output_path: String,
    pub archive_path: Option<String>,
    pub package_duration_ms: u64,
    pub total_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionRequest {
    pub project_path: String,
    pub destination_path: String,
    pub compression_algorithm: CompressionAlgorithm,
    pub clean_before_compress: bool,
    pub cleaning_selection: Option<CleaningRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    pub output_path: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningRequest {
    pub ide_files: bool,
    pub binaries: bool,
    pub build: bool,
    pub intermediate: bool,
    pub derived_data_cache: bool,
    pub saved: bool,
    pub analyze_plugins: bool,
    pub plugin_binaries: bool,
    pub plugin_intermediate: bool,
    pub plugin_node_size_cache: bool,
    pub save_as_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningResult {
    pub original_size: u64,
    pub new_size: u64,
    pub saved_size: u64,
    pub cleaned_items: Vec<String>,
}
