use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ide_programs: IdePrograms,
    pub engine_programs: EnginePrograms,
    pub cleaning_defaults: CleaningDefaults,
    pub general: GeneralSettings,
    pub compression: CompressionSettings,
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

/// Compression settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionSettings {
    pub filename_format: String,
    pub custom_presets: HashMap<String, String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ide_programs: IdePrograms::default(),
            engine_programs: EnginePrograms::default(),
            cleaning_defaults: CleaningDefaults::default(),
            general: GeneralSettings::default(),
            compression: CompressionSettings::default(),
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

impl Default for CompressionSettings {
    fn default() -> Self {
        let mut custom_presets = HashMap::new();
        
        // Add comprehensive default presets
        custom_presets.insert("Default".to_string(), "[Project]_[YYYY][MM][DD][HH][mm]".to_string());
        custom_presets.insert("Default Extended".to_string(), "[Project]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]".to_string());
        custom_presets.insert("Simple".to_string(), "[Project]_[Type]".to_string());
        custom_presets.insert("Detailed".to_string(), "[Project]_[Engine]_[Type]_[YYYY][MM][DD]_[HH][mm]".to_string());
        custom_presets.insert("Archive Style".to_string(), "[YYYY]-[MM]-[DD]_[Project]_[Algorithm]".to_string());
        custom_presets.insert("User Specific".to_string(), "[User]_[Project]_[Mon][DD]_[HH][mm]".to_string());
        custom_presets.insert("Timestamp".to_string(), "[Project]_[Timestamp]".to_string());
        custom_presets.insert("Size Aware".to_string(), "[Project]_[SizeGB]GB_[YYYY][MM][DD]".to_string());
        custom_presets.insert("Engine Specific".to_string(), "UE[Engine]_[Project]_[Type]_[YYYY][MM][DD]".to_string());
        custom_presets.insert("Professional".to_string(), "[Project]_v[YYYY].[MM].[DD]_[Type]_[Algorithm]".to_string());
        custom_presets.insert("Minimal".to_string(), "[Project]_[YY][MM][DD]".to_string());
        custom_presets.insert("Verbose".to_string(), "[Computer]_[User]_[Project]_[Engine]_[Type]_[YYYY]-[MM]-[DD]_[HH]-[mm]-[ss]".to_string());
        
        Self {
            filename_format: "[Project]_[YYYY][MM][DD][HH][mm]".to_string(),
            custom_presets,
        }
    }
}