use std::path::{Path, PathBuf};
use tauri::{command, AppHandle};
use serde::{Deserialize, Serialize};
use crate::prelude::{log, ErrorLevel};
use crate::projects::models::project::Project;
use crate::projects::payloads::{ProjectDiscoveryRequest, ProjectDiscoveryResult};

/// # Project Discovery Actions
/// This module provides actions for discovering Unreal Engine projects
/// and their associated metadata, such as plugins and engine versions from
/// the user's file system.

#[command]
pub async fn discover_projects(app_handle: AppHandle, request: ProjectDiscoveryRequest) -> Result<ProjectDiscoveryResult, String> {
    let start_time = std::time::Instant::now();
    
    match scan_folder_for_projects(
        &app_handle,
        &request.base_folder,
        request.ignore_templates,
        request.ignore_engine,
        request.ignore_samples,
    ).await {
        Ok(projects) => {
            let duration = start_time.elapsed();
            
            Project::add_projects(&app_handle, &projects)
                .unwrap_or_else(|e| {
                    eprintln!("Error adding projects to store: {}", e);
                    log(&app_handle, ErrorLevel::Error, 
                        &format!("Error adding projects to store: {}", e));
                });
            
            Ok(ProjectDiscoveryResult {
                total_found: projects.len(),
                projects,
                scan_duration_ms: duration.as_millis(),
            })
        }
        Err(e) => Err(format!("Failed to discover projects: {}", e)),
    }
}

pub async fn scan_folder_for_projects(
    app_handle: &AppHandle,
    folder_path: &str,
    ignore_template: bool,
    ignore_engine: bool,
    ignore_samples: bool,
) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
    let mut detected_projects = Vec::new();

    // Use glob to find all .uproject files in the specified folder
    let pattern = format!("{}/**/*.uproject", folder_path);
    'entries: for entry in glob::glob(&pattern)? {
        match entry {
            Ok(path) => {
                if let Some(project_path) = path.to_str() {
                    //check if the path pass through a folder named "Templates"
                    // and if ignore_template is true, skip those projects
                    for folder in Path::new(project_path).components(){
                        if (ignore_template && folder.as_os_str() == "Templates")
                            || (ignore_engine && folder.as_os_str() == "Engine")
                            || (ignore_samples && folder.as_os_str() == "Samples") {
                            continue 'entries;
                        }
                    }

                    detected_projects.push(path);
                }
            }
            Err(e) => eprintln!("Error reading glob entry: {}", e),
        }
    }
    
    // for each project found, create a Project object and if
    // it's not already in the list, add it
    let projects_list = Project::get_projects(app_handle)
        .unwrap_or_else(|e| {
            eprintln!("Error getting projects from store: {}", e);
            log(&app_handle, ErrorLevel::Error, 
                &format!("Error getting projects from store: {}", e));
            vec![]
        });
    
    let known_path = projects_list.iter()
        .map(|p| p.path.clone())
        .collect::<Vec<PathBuf>>();
    
    let mut new_projects = Vec::new();
    
    for path in detected_projects {
        if !known_path.contains(&path) {
            // Create a new Project object
            let new_project = match Project::try_from_path(path.clone()) {
                Ok(project) => project,
                Err(e) => {
                    eprintln!("Error creating project from path {}: {}", path.display(), e);
                    log(&app_handle, ErrorLevel::Error, 
                        &format!("Error creating project from path {}: {}", path.display(), e));
                    continue;
                }
            };
            
            new_projects.push(new_project);
        }
    }
    
    Ok(new_projects)
}

#[command]
pub fn remove_project(
    app_handle: AppHandle,
    project_path: String,
) -> Result<(), String> {
    // Remove the project from the store
    match Project::remove_projects(&app_handle, &vec![PathBuf::from(project_path.clone())]) {
        Ok(_) => {
            log(&app_handle, ErrorLevel::Info, 
                &format!("Project at {} removed successfully.", project_path));
            Ok(())
        }
        Err(e) => {
            eprintln!("Error removing project: {}", e);
            log(&app_handle, ErrorLevel::Error, 
                &format!("Error removing project: {}", e));
            Err(format!("Failed to remove project: {}", e))
        }
    }
}

#[command]
pub fn get_projects(
    app_handle: AppHandle,
) -> Result<Vec<Project>, String> {
    // Get the list of projects from the store
    match Project::get_projects(&app_handle) {
        Ok(projects) => Ok(projects),
        Err(e) => {
            eprintln!("Error getting projects: {}", e);
            log(&app_handle, ErrorLevel::Error, 
                &format!("Error getting projects: {}", e));
            Err(format!("Failed to get projects: {}", e))
        }
    }
}