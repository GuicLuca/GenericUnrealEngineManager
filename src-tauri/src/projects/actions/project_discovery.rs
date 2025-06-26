use crate::misc::prelude::{log};
use crate::misc::progress::TaskProgress;
use crate::projects::models::project::Project;
use crate::misc::payloads::{ProjectDiscoveryRequest, ProjectDiscoveryResult};
use std::path::{Path, PathBuf};
use log::error;
use tauri::{command, AppHandle};
use crate::misc::errors::ErrorLevel;

/// # Project Discovery Actions
/// This module provides actions for discovering Unreal Engine projects
/// and their associated metadata, such as plugins and engine versions from
/// the user's file system.

#[command]
pub async fn discover_projects(
    app_handle: AppHandle,
    request: ProjectDiscoveryRequest,
) -> Result<ProjectDiscoveryResult, String> {
    let start_time = std::time::Instant::now();
    let task_id = format!("discover_projects_{}", chrono::Utc::now().timestamp_millis());
    let progress = TaskProgress::new(
        app_handle.clone(),
        task_id,
        format!("Discovering projects in {}", request.base_folder)
    );

    match scan_folder_for_projects(
        &app_handle,
        &request.base_folder,
        request.ignore_templates,
        request.ignore_engine,
        request.ignore_samples,
        &progress,
    )
    .await
    {
        Ok(projects) => {
            let duration = start_time.elapsed();

            progress.update(0.9, Some("Adding projects to store...".to_string()));

            Project::add_projects(&app_handle, &projects).unwrap_or_else(|e| {
                eprintln!("Error adding projects to store: {}", e);
                log(
                    &app_handle,
                    ErrorLevel::Error,
                    &format!("Error adding projects to store: {}", e),
                );
            });

            progress.complete(Some(format!("Found {} projects", projects.len())));

            Ok(ProjectDiscoveryResult {
                total_found: projects.len(),
                projects,
                scan_duration_ms: duration.as_millis(),
            })
        }
        Err(e) => {
            progress.fail(Some(format!("Discovery failed: {}", e)));
            Err(format!("Failed to discover projects: {}", e))
        }
    }
}

pub async fn scan_folder_for_projects(
    app_handle: &AppHandle,
    folder_path: &str,
    ignore_template: bool,
    ignore_engine: bool,
    ignore_samples: bool,
    progress: &TaskProgress,
) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
    let mut detected_projects = Vec::new();

    progress.update(0.1, Some("Scanning for .uproject files...".to_string()));

    // Use glob to find all .uproject files in the specified folder
    let pattern = format!("{}/**/*.uproject", folder_path);
    let glob_entries: Vec<_> = glob::glob(&pattern)?.collect();
    let total_files = glob_entries.len();

    progress.update(0.2, Some(format!("Found {} .uproject files", total_files)));

    'entries: for (index, entry) in glob_entries.into_iter().enumerate() {
        // Update progress for file scanning
        let scan_progress = 0.2 + (index as f32 / total_files as f32) * 0.5;
        progress.update(scan_progress, Some(format!("Scanning file {} of {}", index + 1, total_files)));

        match entry {
            Ok(path) => {
                if let Some(project_path) = path.to_str() {
                    //check if the path pass through a folder named "Templates"
                    // and if ignore_template is true, skip those projects
                    for folder in Path::new(project_path).components() {
                        if (ignore_template && folder.as_os_str() == "Templates")
                            || (ignore_engine && folder.as_os_str() == "Engine")
                            || (ignore_samples && folder.as_os_str() == "Samples")
                        {
                            continue 'entries;
                        }
                    }

                    detected_projects.push(path);
                }
            }
            Err(e) => eprintln!("Error reading glob entry: {}", e),
        }
    }

    progress.update(0.7, Some("Processing discovered projects...".to_string()));

    // for each project found, create a Project object and if
    // it's not already in the list, add it
    let projects_list = Project::get_projects(app_handle).unwrap_or_else(|e| {
        eprintln!("Error getting projects from store: {}", e);
        log(
            &app_handle,
            ErrorLevel::Error,
            &format!("Error getting projects from store: {}", e),
        );
        vec![]
    });

    let known_path = projects_list
        .iter()
        .map(|p| p.path.clone())
        .collect::<Vec<PathBuf>>();

    let mut new_projects = Vec::new();
    let total_new = detected_projects.len();

    for (index, path) in detected_projects.into_iter().enumerate() {
        // Update progress for project processing
        let process_progress = 0.7 + (index as f32 / total_new as f32) * 0.2;
        progress.update(process_progress, Some(format!("Processing project {} of {}", index + 1, total_new)));

        if !known_path.contains(&path) {
            // Create a new Project object
            let new_project = match Project::try_from_path(&path) {
                Ok(project) => project,
                Err(e) => {
                    eprintln!("Error creating project from path {}: {}", path.display(), e);
                    log(
                        &app_handle,
                        ErrorLevel::Error,
                        &format!("Error creating project from path {}: {}", path.display(), e),
                    );
                    continue;
                }
            };

            new_projects.push(new_project);
        }
    }

    Ok(new_projects)
}

#[command]
pub fn remove_projects(app_handle: AppHandle, project_paths: Vec<String>) -> Result<(), String> {
    let task_id = format!("remove_projects_{}", chrono::Utc::now().timestamp_millis());
    let progress = TaskProgress::new(
        app_handle.clone(),
        task_id,
        format!("Removing {} project(s)", project_paths.len())
    );

    // Convert the project paths from strings to PathBuf
    let paths_to_remove = project_paths
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>();

    progress.update(0.5, Some("Updating project store...".to_string()));

    // Remove the project from the store
    match Project::remove_projects(&app_handle, &paths_to_remove) {
        Ok(_) => {
            let path_removed_log_string = paths_to_remove
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<String>>()
                .join("\n\t-> ");
            log(
                // Log the list of removed projects
                &app_handle,
                ErrorLevel::Info,
                &format!(
                    "Removed following project(s) from tracking:\n\t-> {}",
                    path_removed_log_string
                ),
            );
            progress.complete(Some("Projects removed successfully".to_string()));
            Ok(())
        }
        Err(e) => {
            eprintln!("Error removing project(s): {}", e);
            log(
                &app_handle,
                ErrorLevel::Error,
                &format!("Error removing project(s): {}", e),
            );
            progress.fail(Some(format!("Failed to remove projects: {}", e)));
            Err(format!("Failed to remove project(s): {}", e))
        }
    }
}

#[command]
pub fn get_projects(app_handle: AppHandle) -> Result<Vec<Project>, String> {
    // Get the list of projects from the store
    match Project::get_projects(&app_handle) {
        Ok(projects) => {
            log(
                &app_handle,
                ErrorLevel::Info,
                &format!(
                    "Projects refreshed from backend. Retrieved {} project(s) from store.",
                    projects.len()
                ),
            );
            Ok(projects)
        }
        Err(e) => {
            eprintln!("Error getting project(s): {}", e);
            log(
                &app_handle,
                ErrorLevel::Error,
                &format!("Error refreshing project(s): {}", e),
            );
            Err(format!("Failed to get project(s): {}", e))
        }
    }
}

#[command]
pub fn rescan_projects(app_handle: AppHandle, project_paths: Vec<String>) -> Result<(), String> {
    let task_id = format!("rescan_projects_{}", chrono::Utc::now().timestamp_millis());
    let progress = TaskProgress::new(
        app_handle.clone(),
        task_id,
        format!("Rescanning {} project(s)", project_paths.len())
    );

    // Convert the project paths from strings to PathBuf
    let paths_to_refresh = project_paths
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>();
    
    progress.update(0.5, Some("Scanning project metadata...".to_string()));
    
    // Refresh the projects in the store
    match Project::scan_projects(&app_handle, &paths_to_refresh) {
        Ok(_) => {
            log(
                &app_handle,
                ErrorLevel::Info,
                &format!(
                    "Refreshed following project(s):\n\t-> {}",
                    paths_to_refresh
                        .iter()
                        .map(|p| p.display().to_string())
                        .collect::<Vec<String>>()
                        .join("\n\t-> ")
                ),
            );
            progress.complete(Some("Projects rescanned successfully".to_string()));
            Ok(())
        }
        Err(e) => {
            error!("Error refreshing project(s): {}", e);
            log(
                &app_handle,
                ErrorLevel::Error,
                &format!("Error refreshing project(s): {}", e),
            );
            progress.fail(Some(format!("Failed to rescan projects: {}", e)));
            Err(format!("Failed to refresh project(s): {}", e))
        }
    }
}