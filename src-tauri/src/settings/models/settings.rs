use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ide_programs: IdePrograms,
    pub engine_programs: EnginePrograms,
    pub cleaning_defaults: CleaningDefaults,
    pub general: GeneralSettings,
}

/// IDE program settings - only custom programs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdePrograms {
    pub custom_programs: HashMap<String, String>,
}

/// Engine program settings - only custom engines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnginePrograms {
    pub custom_engines: HashMap<String, String>,
}

/// Default cleaning selections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningDefaults {
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
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub autostart_enabled: bool,
    pub show_welcome_popup: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ide_programs: IdePrograms::default(),
            engine_programs: EnginePrograms::default(),
            cleaning_defaults: CleaningDefaults::default(),
            general: GeneralSettings::default(),
        }
    }
}

impl Default for IdePrograms {
    fn default() -> Self {
        Self {
            custom_programs: HashMap::new(),
        }
    }
}

impl Default for EnginePrograms {
    fn default() -> Self {
        Self {
            custom_engines: HashMap::new(),
        }
    }
}

impl Default for CleaningDefaults {
    fn default() -> Self {
        Self {
            ide_files: true,
            binaries: true,
            build: true,
            intermediate: true,
            derived_data_cache: false,
            saved: false,
            analyze_plugins: false,
            plugin_binaries: false,
            plugin_intermediate: false,
            plugin_node_size_cache: false,
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            autostart_enabled: false,
            show_welcome_popup: true,
        }
    }
}