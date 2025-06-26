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
    pub docs_url: Option<String>, // URL to the plugin documentation, if available
    pub size_on_disk: Option<u64>, // Size on disk in bytes (None if is_in_project is false)
    pub last_scan_date: u64, // Last scan date of the plugin (seconds since UNIX epoch)
}

#[derive(Debug, Deserialize)]
pub struct UpluginFile {
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,
    #[serde(rename = "MarketplaceURL")]
    pub marketplace_url: Option<String>,
    #[serde(rename = "DocsURL")]
    docs_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UprojectPluginEntry {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "MarketplaceURL")]
    pub marketplace_url: Option<String>,
    #[serde(rename = "DocsURL")]
    docs_url: Option<String>,
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
                docs_url: None,
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

        // Determine docs URL (prefer .uproject data, fallback to .uplugin data)
        let docs_url = uproject_plugin_data
            .and_then(|data| data.docs_url.clone())
            .or(uplugin_data.docs_url);

        // Calculate size on disk for the plugin directory
        let plugin_dir = uplugin_path.parent().unwrap();
        let size_on_disk = Some(fs_extra::dir::get_size(plugin_dir).unwrap_or(0));

        // Get current timestamp
        let last_scan_date = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(ProjectPlugin {
            name: final_name,
            is_enabled,
            is_in_project: true, // This plugin was found in the project's Plugins folder
            marketplace_url,
            docs_url,
            size_on_disk,
            last_scan_date,
        })
    }

    /// Creates a ProjectPlugin from .uproject plugin data only (for plugins not in project folder)
    pub fn from_uproject_data(uproject_plugin_data: &UprojectPluginEntry) -> Self {
        let last_scan_date = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        ProjectPlugin {
            name: uproject_plugin_data.name.clone(),
            is_enabled: uproject_plugin_data.enabled.unwrap_or(true),
            is_in_project: false, // This plugin is not in the project's Plugins folder
            marketplace_url: uproject_plugin_data.marketplace_url.clone(),
            docs_url: uproject_plugin_data.docs_url.clone(),
            size_on_disk: None, // No size since it's not in the project
            last_scan_date,
        }
    }

    /// Rescans a plugin and updates its metadata
    pub fn rescan(&mut self, uplugin_path: &Path, uproject_plugin_data: Option<&UprojectPluginEntry>) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_in_project {
            // For external plugins, just update the scan date and enabled status
            if let Some(uproject_data) = uproject_plugin_data {
                self.is_enabled = uproject_data.enabled.unwrap_or(true);
                self.marketplace_url = uproject_data.marketplace_url.clone();
                self.docs_url = uproject_data.docs_url.clone();
            }
            self.last_scan_date = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            return Ok(());
        }

        // For project plugins, recalculate size and update metadata
        let plugin_dir = uplugin_path.parent().unwrap();
        self.size_on_disk = Some(fs_extra::dir::get_size(plugin_dir).unwrap_or(0));

        // Update enabled status from .uproject data
        if let Some(uproject_data) = uproject_plugin_data {
            self.is_enabled = uproject_data.enabled.unwrap_or(true);
            // Update marketplace URL if available in .uproject
            if uproject_data.marketplace_url.is_some() {
                self.marketplace_url = uproject_data.marketplace_url.clone();
            }
            // Update docs URL if available in .uproject
            if uproject_data.docs_url.is_some() {
                self.docs_url = uproject_data.docs_url.clone();
            }
        }

        // Update scan date
        self.last_scan_date = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(())
    }
}