use serde::{Deserialize, Serialize};

/// Represents a plugin in an Unreal Engine project read from the .uplugin file.
/// It does not include all the metadata from the .uplugin file, but only the 
/// data related to the plugin's association with a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPlugin {
    pub name: String, // Name of the plugin (from .uplugin file)
    pub is_enabled: bool, // Indicates if the plugin is enabled
    pub is_in_project: bool, // Indicates if the plugin is part of the project (in ./Plugins directory) or in the engine (in ENGINE/Plugins/... directory)
    pub marketplace_url: Option<String>, // URL to the plugin on the Unreal Marketplace, if available
    pub target_allow_list: Vec<String>, // List of target the plugin is embedded in (e.g., "Editor", "Game", etc.)
}