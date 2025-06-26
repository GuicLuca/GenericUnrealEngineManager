use crate::misc::errors::Verror::MessageError;
use crate::misc::errors::{ErrorLevel, Result};
use crate::misc::prelude::{format_size, log};
use crate::misc::progress::TaskProgress;
use crate::projects::models::project::Project;
use crate::settings::actions::settings_manager;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{command, AppHandle};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningSelection {
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
    pub save_as_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningResult {
    pub original_size: u64,
    pub new_size: u64,
    pub saved_size: u64,
    pub cleaned_items: Vec<String>,
}

/// Clean project temporary and generated files
#[command]
pub async fn clean_project(
    app_handle: AppHandle,
    project_path: String,
    selection: CleaningSelection,
) -> Result<CleaningResult> {
    let project_path = PathBuf::from(project_path);

    if !project_path.exists() {
        let error_msg = format!("Project file does not exist: {}", project_path.display());
        error!("{}", error_msg);
        log(&app_handle, ErrorLevel::Error, &error_msg);
        return Err(MessageError(error_msg));
    }

    let project_dir = project_path.parent().unwrap();
    let project_name = project_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown");

    let task_id = format!("clean_project_{}", chrono::Utc::now().timestamp_millis());
    let progress = TaskProgress::new(
        app_handle.clone(),
        task_id,
        format!("Cleaning project: {}", project_name)
    );

    info!("Starting cleaning process for project: {}", project_name);
    log(
        &app_handle,
        ErrorLevel::Info,
        &format!("Starting cleaning process for project: {}", project_name),
    );

    progress.update(0.1, Some("Calculating original size...".to_string()));

    // Get the original size
    let original_size = fs_extra::dir::get_size(project_dir).unwrap_or(0);

    let mut cleaned_items = Vec::new();

    progress.update(0.2, Some("Cleaning project directories...".to_string()));

    // Clean project-level directories
    if selection.ide_files {
        clean_directory(project_dir, ".vs", &mut cleaned_items);
        clean_directory(project_dir, ".idea", &mut cleaned_items);
    }

    if selection.binaries {
        clean_directory(project_dir, "Binaries", &mut cleaned_items);
    }

    if selection.build {
        clean_directory(project_dir, "Build", &mut cleaned_items);
    }

    if selection.intermediate {
        clean_directory(project_dir, "Intermediate", &mut cleaned_items);
    }

    if selection.derived_data_cache {
        clean_directory(project_dir, "DerivedDataCache", &mut cleaned_items);
    }

    if selection.saved {
        clean_directory(project_dir, "Saved", &mut cleaned_items);
    }

    progress.update(0.6, Some("Cleaning plugin directories...".to_string()));

    // Clean plugin directories if requested
    if selection.analyze_plugins {
        let plugins_dir = project_dir.join("Plugins");
        if plugins_dir.exists() {
            if let Ok(entries) = fs::read_dir(&plugins_dir) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        let plugin_dir = entry.path();

                        if selection.plugin_binaries {
                            clean_directory(&plugin_dir, "Binaries", &mut cleaned_items);
                        }

                        if selection.plugin_intermediate {
                            clean_directory(&plugin_dir, "Intermediate", &mut cleaned_items);
                        }

                        if selection.plugin_node_size_cache {
                            clean_directory(&plugin_dir, "NodeSizeCache", &mut cleaned_items);
                        }
                    }
                }
            }
        }
    }

    progress.update(0.8, Some("Saving settings...".to_string()));

    // Save as default if requested
    if selection.save_as_default {
        if let Err(e) = save_cleaning_defaults(&app_handle, &selection) {
            error!("Failed to save cleaning defaults: {}", e);
            log(
                &app_handle,
                ErrorLevel::Error,
                &format!("Failed to save cleaning defaults: {}", e),
            );
        }
    }

    progress.update(0.9, Some("Calculating final size...".to_string()));

    // Get the new size
    let new_size = fs_extra::dir::get_size(project_dir).unwrap_or(0);
    let saved_size = original_size.saturating_sub(new_size);

    let result = CleaningResult {
        original_size,
        new_size,
        saved_size,
        cleaned_items,
    };

    // Log completion
    let saved_size_str = format_size(saved_size);
    let new_size_str = format_size(new_size);
    let completion_msg = format!(
        "Cleaning process complete for {}, new size on disk: {}, saved {}.",
        project_name, new_size_str, saved_size_str
    );

    info!("{}", completion_msg);
    log(&app_handle, ErrorLevel::Info, &completion_msg);

    // Update project size in store
    if let Err(e) = update_project_size(&app_handle, &project_path, new_size) {
        error!("Failed to update project size: {}", e);
    }

    progress.complete(Some(format!("Cleaned {} items, saved {}", cleaned_items.len(), saved_size_str)));

    Ok(result)
}

fn clean_directory(base_dir: &Path, dir_name: &str, cleaned_items: &mut Vec<String>) {
    let target_dir = base_dir.join(dir_name);
    if target_dir.exists() {
        match fs::remove_dir_all(&target_dir) {
            Ok(_) => {
                cleaned_items.push(format!("{}/{}", base_dir.display(), dir_name));
                info!("Cleaned directory: {}", target_dir.display());
            }
            Err(e) => {
                error!("Failed to clean directory {}: {}", target_dir.display(), e);
            }
        }
    }
}

fn save_cleaning_defaults(app_handle: &AppHandle, selection: &CleaningSelection) -> Result<()> {
    let mut settings = settings_manager::load_settings(app_handle)?;

    settings.cleaning_defaults.ide_files = selection.ide_files;
    settings.cleaning_defaults.binaries = selection.binaries;
    settings.cleaning_defaults.build = selection.build;
    settings.cleaning_defaults.intermediate = selection.intermediate;
    settings.cleaning_defaults.derived_data_cache = selection.derived_data_cache;
    settings.cleaning_defaults.saved = selection.saved;
    settings.cleaning_defaults.analyze_plugins = selection.analyze_plugins;
    settings.cleaning_defaults.plugin_binaries = selection.plugin_binaries;
    settings.cleaning_defaults.plugin_intermediate = selection.plugin_intermediate;
    settings.cleaning_defaults.plugin_node_size_cache = selection.plugin_node_size_cache;

    settings_manager::store_settings(app_handle, &settings)?;

    Ok(())
}

fn update_project_size(app_handle: &AppHandle, project_path: &Path, new_size: u64) -> Result<()> {
    let mut projects = Project::get_projects(app_handle)?;

    if let Some(project) = projects.iter_mut().find(|p| p.path == project_path) {
        project.size_on_disk = new_size;
        project.last_scan_date = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    Project::save_projects(app_handle, &projects)?;

    Ok(())
}