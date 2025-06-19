use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ide_programs: IdePrograms,
}

/// IDE program settings for different platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdePrograms {
    pub visual_studio: Option<String>,
    pub visual_studio_code: Option<String>,
    pub clion: Option<String>,
    pub rider: Option<String>,
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
            visual_studio: Self::detect_visual_studio(),
            visual_studio_code: Self::detect_vscode(),
            clion: Self::detect_clion(),
            rider: Self::detect_rider(),
            custom_programs: HashMap::new(),
        }
    }
}

impl IdePrograms {
    /// Attempt to detect Visual Studio installation
    fn detect_visual_studio() -> Option<String> {
        #[cfg(target_os = "windows")]
        {
            // Common Visual Studio paths
            let vs_paths = [
                r"C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\devenv.exe",
                r"C:\Program Files\Microsoft Visual Studio\2022\Professional\Common7\IDE\devenv.exe",
                r"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\Common7\IDE\devenv.exe",
                r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\Common7\IDE\devenv.exe",
                r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Professional\Common7\IDE\devenv.exe",
                r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\devenv.exe",
            ];
            
            for path in &vs_paths {
                if std::path::Path::new(path).exists() {
                    return Some(path.to_string());
                }
            }
        }
        None
    }
    
    /// Attempt to detect Visual Studio Code installation
    fn detect_vscode() -> Option<String> {
        #[cfg(target_os = "windows")]
        {
            let vscode_paths = [
                r"C:\Users\%USERNAME%\AppData\Local\Programs\Microsoft VS Code\Code.exe",
                r"C:\Program Files\Microsoft VS Code\Code.exe",
                r"C:\Program Files (x86)\Microsoft VS Code\Code.exe",
            ];
            
            for path in &vscode_paths {
                let expanded_path = path.replace("%USERNAME%", &std::env::var("USERNAME").unwrap_or_default());
                if std::path::Path::new(&expanded_path).exists() {
                    return Some(expanded_path);
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            let vscode_path = "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code";
            if std::path::Path::new(vscode_path).exists() {
                return Some(vscode_path.to_string());
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Try to find code in PATH
            if let Ok(output) = std::process::Command::new("which").arg("code").output() {
                if output.status.success() {
                    if let Ok(path) = String::from_utf8(output.stdout) {
                        return Some(path.trim().to_string());
                    }
                }
            }
        }
        
        None
    }
    
    /// Attempt to detect CLion installation
    fn detect_clion() -> Option<String> {
        #[cfg(target_os = "windows")]
        {
            // CLion is typically installed in JetBrains Toolbox or standalone
            let clion_paths = [
                r"C:\Users\%USERNAME%\AppData\Local\JetBrains\Toolbox\apps\CLion\ch-0\*\bin\clion64.exe",
                r"C:\Program Files\JetBrains\CLion *\bin\clion64.exe",
            ];
            
            for path_pattern in &clion_paths {
                let expanded_path = path_pattern.replace("%USERNAME%", &std::env::var("USERNAME").unwrap_or_default());
                // For now, return the pattern - in a real implementation, you'd use glob matching
                if expanded_path.contains("*") {
                    // This is a simplified detection - you might want to use glob crate for pattern matching
                    continue;
                }
                if std::path::Path::new(&expanded_path).exists() {
                    return Some(expanded_path);
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            let clion_path = "/Applications/CLion.app/Contents/MacOS/clion";
            if std::path::Path::new(clion_path).exists() {
                return Some(clion_path.to_string());
            }
        }
        
        None
    }
    
    /// Attempt to detect JetBrains Rider installation
    fn detect_rider() -> Option<String> {
        #[cfg(target_os = "windows")]
        {
            let rider_paths = [
                r"C:\Users\%USERNAME%\AppData\Local\JetBrains\Toolbox\apps\Rider\ch-0\*\bin\rider64.exe",
                r"C:\Program Files\JetBrains\Rider *\bin\rider64.exe",
            ];
            
            for path_pattern in &rider_paths {
                let expanded_path = path_pattern.replace("%USERNAME%", &std::env::var("USERNAME").unwrap_or_default());
                if expanded_path.contains("*") {
                    continue;
                }
                if std::path::Path::new(&expanded_path).exists() {
                    return Some(expanded_path);
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            let rider_path = "/Applications/Rider.app/Contents/MacOS/rider";
            if std::path::Path::new(rider_path).exists() {
                return Some(rider_path.to_string());
            }
        }
        
        None
    }
}