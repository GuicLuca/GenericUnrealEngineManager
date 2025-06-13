use std::path::Path;
use tauri::command;
use serde::{Deserialize, Serialize};

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
    pub projects: Vec<String>,
    pub total_found: usize,
    pub scan_duration_ms: u128,
}

#[command]
pub async fn discover_projects(request: ProjectDiscoveryRequest) -> Result<ProjectDiscoveryResult, String> {
    let start_time = std::time::Instant::now();
    
    match scan_folder_for_projects(
        &request.base_folder,
        request.ignore_templates,
        request.ignore_engine,
        request.ignore_samples,
    ).await {
        Ok(projects) => {
            let duration = start_time.elapsed();
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
    folder_path: &str,
    ignore_template: bool,
    ignore_engine: bool,
    ignore_samples: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut projects = Vec::new();

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
                    
                    projects.push(project_path.to_string());
                }
            }
            Err(e) => eprintln!("Error reading glob entry: {}", e),
        }
    }

    Ok(projects)
}