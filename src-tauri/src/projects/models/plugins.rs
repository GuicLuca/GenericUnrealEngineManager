use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Represents a plugin in an Unreal Engine project read from the .uplugin file.
/// It does not include all the metadata from the .uplugin file, but only the 
/// data related to the plugin's association with a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPlugin {
    pub name: String, // Name of the plugin (from .uplugin file)
    pub is_enabled: bool, // Indicates if the plugin is enabled
    pub is_in_project: bool, // Indicates if the plugin is part of the project (in ./Plugins directory) or in the engine (in ENGINE/Plugins/... directory)
    pub marketplace_url: Option<String>, // URL to the plugin on the Unreal Marketplace, if available
}

#[derive(Debug, Deserialize)]
struct UpluginFile {
    #[serde(rename = "FriendlyName")]
    friendly_name: Option<String>,
    #[serde(rename = "MarketplaceURL")]
    marketplace_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UprojectPluginEntry {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Enabled")]
    enabled: Option<bool>,
    #[serde(rename = "MarketplaceURL")]
    marketplace_url: Option<String>,
}

impl ProjectPlugin {
    /// Creates a ProjectPlugin from a .uplugin file path and optional .uproject plugin data
    pub fn try_from_path(
        uplugin_path: &Path,
        uproject_plugin_data: Option<&UprojectPluginEntry>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Extract plugin name from the file name
        let plugin_name = uplugin_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid plugin file name")?
            .to_string();

        // Read and parse the .uplugin file
        let uplugin_content = fs::read_to_string(uplugin_path)?;
        let uplugin_data: UpluginFile = serde_json::from_str(&uplugin_content)
            .unwrap_or_else(|_| UpluginFile {
                friendly_name: None,
                marketplace_url: None,
            });

        // Determine the final plugin name (prefer friendly name from .uplugin, fallback to file name)
        let final_name = uplugin_data.friendly_name
            .unwrap_or_else(|| plugin_name.clone());

        // Determine if the plugin is enabled (from .uproject data, default to true)
        let is_enabled = uproject_plugin_data
            .and_then(|data| data.enabled)
            .unwrap_or(true);

        // Determine marketplace URL (prefer .uproject data, fallback to .uplugin data)
        let marketplace_url = uproject_plugin_data
            .and_then(|data| data.marketplace_url.clone())
            .or(uplugin_data.marketplace_url);

        Ok(ProjectPlugin {
            name: final_name,
            is_enabled,
            is_in_project: true, // This plugin was found in the project's Plugins folder
            marketplace_url,
        })
    }

    /// Creates a ProjectPlugin from .uproject plugin data only (for plugins not in project folder)
    pub fn from_uproject_data(uproject_plugin_data: &UprojectPluginEntry) -> Self {
        ProjectPlugin {
            name: uproject_plugin_data.name.clone(),
            is_enabled: uproject_plugin_data.enabled.unwrap_or(true),
            is_in_project: false, // This plugin is not in the project's Plugins folder
            marketplace_url: uproject_plugin_data.marketplace_url.clone(),
        }
    }
}