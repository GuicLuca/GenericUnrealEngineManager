use crate::misc::errors::{ErrorLevel, Result};
use crate::misc::prelude::log;
use crate::projects::models::project::Project;
use log::{error, info};
use std::path::PathBuf;
use tauri::{command, AppHandle};

/// Scan/refresh plugins for specific projects
#[command]
pub fn scan_plugins(app_handle: AppHandle, project_paths: Vec<String>) -> Result<()> {
    // Convert the project paths from strings to PathBuf
    let paths_to_scan = project_paths
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>();
    
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
            Ok(())
        }
        Err(e) => {
            error!("Error refreshing plugins for project(s): {}", e);
            log(
                &app_handle,
                ErrorLevel::Error,
                &format!("Error refreshing plugins for project(s): {}", e),
            );
            Err(e.into())
        }
    }
}

/// Refresh plugins for all tracked projects
#[command]
pub fn refresh_all_plugins(app_handle: AppHandle) -> Result<()> {
    match Project::refresh_all_plugins(&app_handle) {
        Ok(_) => {
            log(
                &app_handle,
                ErrorLevel::Info,
                "Refreshed plugins for all tracked projects",
            );
            Ok(())
        }
        Err(e) => {
            error!("Error refreshing plugins for all projects: {}", e);
            log(
                &app_handle,
                ErrorLevel::Error,
                &format!("Error refreshing plugins for all projects: {}", e),
            );
            Err(e.into())
        }
    }
}