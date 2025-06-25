use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ide_programs: IdePrograms,
}

/// IDE program settings - only custom programs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdePrograms {
    pub custom_programs: HashMap<String, String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ide_programs: IdePrograms::default(),
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