use std::path::{Path, PathBuf};
use tauri::{command, AppHandle, Emitter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri_plugin_store::StoreExt;
use crate::env;
use crate::prelude::{log, ErrorLevel};
use crate::projects::models::project::Project;

/// # Project Discovery Actions
/// This module provides actions for discovering Unreal Engine projects
/// and their associated metadata, such as plugins and engine versions from
/// the user's file system. 

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDiscoveryRequest {
    pub base_folder: String,
    pub ignore_engine: bool,
    pub ignore_templates: bool,
    pub ignore_samples: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDiscoveryResult {
    pub projects: Vec<Project>,
    pub total_found: usize,
    pub scan_duration_ms: u128,
}

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
        .map(|p| p.path)
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