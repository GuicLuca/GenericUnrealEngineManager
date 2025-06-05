use std::path::{PathBuf};
use crate::projects::models::plugins::ProjectPlugin;

/// Represents the association of a project with a specific Unreal Engine version.
#[derive(Debug, Clone)]
pub enum EngineAssociation {
    Standard(String), // For standard version (4.27, 5.0, etc.)
    Custom, // For custom engines (Unreal Source, etc.)
}

/// A project represents an Unreal Engine project with its associated metadata.
/// It's build from the .uproject file and allows to access various properties.
#[derive(Debug, Clone)]
pub struct Project {
    pub name: String, // Name of the project (from .uproject file)
    pub description: String, // Description of the project (from .uproject file)
    pub engine_association: EngineAssociation, // Engine version or "Custom" for Unreal Source
    pub path: PathBuf, // Path to the project (.uproject file)
    pub has_cpp: bool, // Indicates if the project has C++ code
    pub plugins: Vec<ProjectPlugin>, // List of plugins associated with the project
}