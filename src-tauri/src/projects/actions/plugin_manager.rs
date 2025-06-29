use crate::misc::errors::{ErrorLevel, Result};
use crate::misc::prelude::log;
use crate::misc::progress::TaskProgress;
use crate::projects::models::project::Project;
use log::error;
use std::path::PathBuf;
use tauri::{command, AppHandle};

/// Scan/refresh plugins for specific projects
#[command]
pub fn scan_plugins(app_handle: AppHandle, project_paths: Vec<String>) -> Result<()> {
    let task_id = format!("scan_plugins_{}", chrono::Utc::now().timestamp_millis());
    let progress = TaskProgress::new(
        app_handle.clone(),
        task_id,
        format!("Scanning plugins for {} project(s)", project_paths.len())
    );

    // Convert the project paths from strings to PathBuf
    let paths_to_scan = project_paths
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>();

    progress.update(0.5, Some("Analyzing plugin directories...".to_string()));

    // Refresh the plugins for the specified projects
    match Project::scan_project_plugins(&app_handle, &paths_to_scan) {
        Ok(_) => {
            log(
                &app_handle,
                ErrorLevel::Info,
                &format!(
                    "Refreshed plugins for following project(s):\n\t-> {}",
                    paths_to_scan
                        .iter()
                        .map(|p| p.display().to_string())
                        .collect::<Vec<String>>()
                        .join("\n\t-> ")
                ),
            );
            progress.complete(Some("Plugin scanning completed".to_string()));
            Ok(())
        }
        Err(e) => {
            error!("Error refreshing plugins for project(s): {}", e);
            log(
                &app_handle,
                ErrorLevel::Error,
                &format!("Error refreshing plugins for project(s): {}", e),
            );
            progress.fail(Some(format!("Plugin scanning failed: {}", e)));
            Err(e.into())
        }
    }
}

/// Refresh plugins for all tracked projects
#[command]
pub fn refresh_all_plugins(app_handle: AppHandle) -> Result<()> {
    let task_id = format!("refresh_all_plugins_{}", chrono::Utc::now().timestamp_millis());
    let progress = TaskProgress::new(
        app_handle.clone(),
        task_id,
        "Refreshing plugins for all projects".to_string()
    );

    progress.update(0.3, Some("Loading project list...".to_string()));

    match Project::refresh_all_plugins(&app_handle) {
        Ok(_) => {
            log(
                &app_handle,
                ErrorLevel::Info,
                "Refreshed plugins for all tracked projects",
            );
            progress.complete(Some("All plugins refreshed successfully".to_string()));
            Ok(())
        }
        Err(e) => {
            error!("Error refreshing plugins for all projects: {}", e);
            log(
                &app_handle,
                ErrorLevel::Error,
                &format!("Error refreshing plugins for all projects: {}", e),
            );
            progress.fail(Some(format!("Plugin refresh failed: {}", e)));
            Err(e.into())
        }
    }
}