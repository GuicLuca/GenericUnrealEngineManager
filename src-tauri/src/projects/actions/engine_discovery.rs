use crate::misc::errors::{ErrorLevel, Result};
use crate::misc::prelude::log;
use crate::misc::progress::TaskProgress;
use crate::settings::actions::settings_manager;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{command, AppHandle};

#[derive(Debug, Deserialize)]
struct BuildVersion {
    #[serde(rename = "MajorVersion")]
    major_version: u32,
    #[serde(rename = "MinorVersion")]
    minor_version: u32,
    #[serde(rename = "PatchVersion")]
    patch_version: u32,
    #[serde(rename = "BranchName")]
    branch_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DetectedEngine {
    pub name: String,
    pub path: String,
    pub version: String,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EngineDiscoveryResult {
    pub engines: Vec<DetectedEngine>,
    pub total_found: usize,
    pub scan_duration_ms: u128,
}

/// Auto-detect installed Unreal Engine versions on the computer
#[command]
pub async fn auto_detect_engines(app_handle: AppHandle) -> Result<EngineDiscoveryResult> {
    let start_time = std::time::Instant::now();
    let task_id = format!("auto_detect_engines_{}", chrono::Utc::now().timestamp_millis());
    let progress = TaskProgress::new(
        app_handle.clone(),
        task_id,
        "Auto-detecting Unreal Engine installations".to_string()
    );

    info!("Starting auto-detection of Unreal Engine installations");
    log(&app_handle, ErrorLevel::Info, "Starting auto-detection of Unreal Engine installations");

    progress.update(0.1, Some("Scanning system drives...".to_string()));

    let mut detected_engines = Vec::new();

    // Get all available drives/mount points
    let drives = get_system_drives()?;
    let total_drives = drives.len();

    for (index, drive) in drives.iter().enumerate() {
        let drive_progress = 0.1 + (index as f32 / total_drives as f32) * 0.8;
        progress.update(
            drive_progress,
            Some(format!("Scanning drive: {}", drive.display()))
        );

        info!("Scanning drive: {}", drive.display());
        log(
            &app_handle,
            ErrorLevel::Info,
            &format!("Scanning drive: {}", drive.display())
        );

        match scan_drive_for_engines(&app_handle, drive, &progress, drive_progress).await {
            Ok(mut engines) => {
                detected_engines.append(&mut engines);
            }
            Err(e) => {
                error!("Error scanning drive {}: {}", drive.display(), e);
                log(
                    &app_handle,
                    ErrorLevel::Error,
                    &format!("Error scanning drive {}: {}", drive.display(), e)
                );
            }
        }
    }

    progress.update(0.9, Some("Saving detected engines...".to_string()));

    // Save detected engines to settings
    if !detected_engines.is_empty() {
        match save_detected_engines(&app_handle, &detected_engines) {
            Ok(_) => {
                log(
                    &app_handle,
                    ErrorLevel::Info,
                    &format!("Saved {} detected engines to settings", detected_engines.len())
                );
            }
            Err(e) => {
                error!("Failed to save detected engines: {}", e);
                log(
                    &app_handle,
                    ErrorLevel::Error,
                    &format!("Failed to save detected engines: {}", e)
                );
            }
        }
    }

    let duration = start_time.elapsed();
    let completion_msg = format!(
        "Engine auto-detection completed. Found {} engines in {:.2} seconds.",
        detected_engines.len(),
        duration.as_secs_f64()
    );

    info!("{}", completion_msg);
    log(&app_handle, ErrorLevel::Info, &completion_msg);

    progress.complete(Some(format!("Found {} engines", detected_engines.len())));

    Ok(EngineDiscoveryResult {
        total_found: detected_engines.len(),
        engines: detected_engines,
        scan_duration_ms: duration.as_millis(),
    })
}

/// Get all system drives/mount points
fn get_system_drives() -> Result<Vec<PathBuf>> {
    let mut drives = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // On Windows, scan all drive letters
        for letter in b'A'..=b'Z' {
            let drive_path = format!("{}:\\", letter as char);
            let path = PathBuf::from(&drive_path);
            if path.exists() {
                drives.push(path);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // On macOS, scan common locations and mounted volumes
        let common_paths = vec![
            "/",
            "/Applications",
            "/Users",
            "/opt",
            "/usr/local",
        ];
        
        for path_str in common_paths {
            let path = PathBuf::from(path_str);
            if path.exists() {
                drives.push(path);
            }
        }
        
        // Also scan mounted volumes
        if let Ok(entries) = fs::read_dir("/Volumes") {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    drives.push(entry.path());
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // On Linux, scan root and common mount points
        let common_paths = vec![
            "/",
            "/home",
            "/opt",
            "/usr",
            "/usr/local",
            "/var",
        ];
        
        for path_str in common_paths {
            let path = PathBuf::from(path_str);
            if path.exists() {
                drives.push(path);
            }
        }
        
        // Also scan mounted filesystems
        let mount_points = vec!["/mnt", "/media"];
        for mount_point in mount_points {
            if let Ok(entries) = fs::read_dir(mount_point) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        drives.push(entry.path());
                    }
                }
            }
        }
    }

    // Remove duplicates and non-existent paths
    drives.retain(|path| path.exists());
    drives.sort();
    drives.dedup();

    Ok(drives)
}

/// Scan a specific drive for Unreal Engine installations
async fn scan_drive_for_engines(
    app_handle: &AppHandle,
    drive: &Path,
    progress: &TaskProgress,
    base_progress: f32,
) -> Result<Vec<DetectedEngine>> {
    let mut engines = Vec::new();

    // Use glob to find potential engine directories
    // Look for directories that contain an "Engine" subdirectory
    let pattern = format!("{}/**/Engine", drive.display());
    
    info!("Searching pattern: {}", pattern);
    
    let glob_entries: Vec<_> = match glob::glob(&pattern) {
        Ok(entries) => entries.collect(),
        Err(e) => {
            error!("Glob pattern error for {}: {}", pattern, e);
            return Ok(engines);
        }
    };

    let total_candidates = glob_entries.len();
    if total_candidates == 0 {
        return Ok(engines);
    }

    info!("Found {} potential engine directories on {}", total_candidates, drive.display());

    for (index, entry) in glob_entries.into_iter().enumerate() {
        let candidate_progress = base_progress + (index as f32 / total_candidates as f32) * 0.05;
        progress.update(
            candidate_progress,
            Some(format!("Checking potential engine directory {} of {}", index + 1, total_candidates))
        );

        match entry {
            Ok(engine_path) => {
                // engine_path points to the "Engine" directory, we need the parent
                if let Some(engine_root) = engine_path.parent() {
                    match validate_engine_directory(engine_root) {
                        Ok(Some(detected_engine)) => {
                            info!("Detected engine: {} at {}", detected_engine.name, detected_engine.path);
                            log(
                                app_handle,
                                ErrorLevel::Info,
                                &format!("Detected engine: {} at {}", detected_engine.name, detected_engine.path)
                            );
                            engines.push(detected_engine);
                        }
                        Ok(None) => {
                            // Not a valid engine directory, continue
                        }
                        Err(e) => {
                            error!("Error validating engine directory {}: {}", engine_root.display(), e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error reading glob entry: {}", e);
            }
        }
    }

    Ok(engines)
}

/// Validate if a directory is a valid Unreal Engine installation
fn validate_engine_directory(engine_root: &Path) -> Result<Option<DetectedEngine>> {
    let engine_dir = engine_root.join("Engine");
    
    // Check if Engine directory exists
    if !engine_dir.exists() {
        return Ok(None);
    }

    // Check for required subdirectories in Engine
    let required_dirs = ["Binaries", "Source", "Plugins", "Shaders"];
    for dir_name in &required_dirs {
        if !engine_dir.join(dir_name).exists() {
            return Ok(None);
        }
    }

    // Check for Build.version file
    let build_version_path = engine_dir.join("Build").join("Build.version");
    if !build_version_path.exists() {
        return Ok(None);
    }

    // Parse Build.version file
    let build_version_content = fs::read_to_string(&build_version_path)?;
    let build_version: BuildVersion = match serde_json::from_str(&build_version_content) {
        Ok(bv) => bv,
        Err(e) => {
            error!("Failed to parse Build.version at {}: {}", build_version_path.display(), e);
            return Ok(None);
        }
    };

    // Create version string
    let version = format!("{}.{}.{}", 
        build_version.major_version, 
        build_version.minor_version, 
        build_version.patch_version
    );

    // Determine if it's a custom engine based on branch name
    let is_custom = !is_precompiled_branch(&build_version.branch_name);

    // Create engine name
    let name = if is_custom {
        format!("Custom-{}", version)
    } else {
        version.clone()
    };

    Ok(Some(DetectedEngine {
        name,
        path: engine_root.to_string_lossy().to_string(),
        version,
        is_custom,
    }))
}

/// Check if a branch name indicates a precompiled engine
fn is_precompiled_branch(branch_name: &str) -> bool {
    // Regex pattern to match precompiled branch names like:
    // "++UE5+Release-5.1", "++UE4+Release-4.25", etc.
    let pattern = regex::Regex::new(r"^\+\+UE[45]\+Release-\d+\.\d+").unwrap();
    pattern.is_match(branch_name)
}

/// Save detected engines to settings
fn save_detected_engines(app_handle: &AppHandle, engines: &[DetectedEngine]) -> Result<()> {
    let mut settings = settings_manager::load_settings(app_handle)?;

    // Add detected engines to custom engines (avoid duplicates)
    for engine in engines {
        // Check if this engine path is already registered
        let already_exists = settings.engine_programs.custom_engines
            .values()
            .any(|existing_path| existing_path == &engine.path);

        if !already_exists {
            settings.engine_programs.custom_engines.insert(
                engine.name.clone(),
                engine.path.clone()
            );
        }
    }

    settings_manager::store_settings(app_handle, &settings)?;

    Ok(())
}