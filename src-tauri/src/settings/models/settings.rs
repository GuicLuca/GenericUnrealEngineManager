use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ide_programs: IdePrograms,
    pub cleaning_defaults: CleaningDefaults,
}

/// IDE program settings - only custom programs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdePrograms {
    pub custom_programs: HashMap<String, String>,
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

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ide_programs: IdePrograms::default(),
            cleaning_defaults: CleaningDefaults::default(),
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