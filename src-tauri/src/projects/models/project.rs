use crate::env;
use crate::misc::prelude::{log};
use crate::projects::models::plugins::ProjectPlugin;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use tauri::Emitter;
use tauri_plugin_store::StoreExt;
use crate::misc::errors::ErrorLevel;
use crate::misc::payloads::ProjectsUpdatedPayload;

/// Represents the association of a project with a specific Unreal Engine version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineAssociation {
    Standard(String), // For standard version (4.27, 5.0, etc.)
    Custom,           // For custom engines (Unreal Source, etc.)
}

/// A project represents an Unreal Engine project with its associated metadata.
/// It's build from the .uproject file and allows to access various properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,        // Name of the project (from .uproject file)
    pub description: String, // Description of the project (from .uproject file)
    pub engine_association: EngineAssociation, // Engine version or "Custom" for Unreal Source
    pub path: PathBuf,       // Path to the project (.uproject file)
    pub has_cpp: bool,       // Indicates if the project has C++ code
    pub plugins: Vec<ProjectPlugin>, // List of plugins associated with the project
    pub size_on_disk: u64, // Size on disk in bytes
    pub last_scan_date: u64, // Last scan date of the project
}

impl Project {
    /// Creates a new project instance.
    pub fn try_from_path(path: &PathBuf) -> Result<Project, Box<dyn std::error::Error>> {
        // try to read the contents of the .uproject file
        let contents = std::fs::read_to_string(&path)?;
        let uproject_content: serde_json::Value = serde_json::from_str(&contents)?;

        // Extract the description, engine association, and plugins from the .uproject file
        let description = uproject_content
            .get("Description")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let engine_association = {
            if let Some(engine_version) = uproject_content.get("EngineAssociation") {
                if engine_version.is_string() && !engine_version.as_str().unwrap().is_empty() {
                    EngineAssociation::Standard(engine_version.as_str().unwrap().to_string())
                } else {
                    EngineAssociation::Custom
                }
            } else {
                EngineAssociation::Custom // Default to Custom if not specified
            }
        };

        // todo: implement plugin discovery

        // Fetch the project name from the file name
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();

        // fetch if there is a Source folder in the project
        let has_cpp = path
            .parent()
            .map(|p| p.join("Source").exists())
            .unwrap_or(false);
        
        // Calculate the size on the disk
        let size_on_disk = fs_extra::dir::get_size(&path.parent().unwrap())
            .unwrap_or(0); // Default to 0 if size cannot be determined

        Ok(Project {
            name,
            description,
            engine_association,
            path: path.clone(),
            has_cpp,
            plugins: Vec::new(), // Initialize with an empty vector for plugins
            size_on_disk,
            last_scan_date: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }
    
    pub fn scan_projects(
        app_handle: &tauri::AppHandle,
        project_paths: &[PathBuf],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut projects = Project::get_projects(app_handle)?;
        
        for project_path in project_paths {
            let project = Project::try_from_path(project_path)?;
            
            // Check if the project already exists
            if let Some(existing_project) = projects.iter_mut().find(|p| p.path == project.path) {
                *existing_project = project.clone(); // Update the existing project
            } else {
                projects.push(project.clone()); // Else, add the project
            }
        }
        
        // Save the updated projects list to the store
        let store = match app_handle.store(env::STORE_FILE_NAME) {
            Ok(store) => store,
            Err(e) => return Err(Box::new(e)),
        };
        
        let projects_json = serde_json::to_value(projects)?;
        store.set(env::STORE_PROJECTS, projects_json);
        store.save()?;
        
        
        // Emit the updated projects event
        Project::emit_project_updated(app_handle)?;
        
        Ok(())
    }

    pub fn get_projects(
        app_handle: &tauri::AppHandle,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        let store = match app_handle.store(env::STORE_FILE_NAME) {
            Ok(store) => store,
            Err(e) => return Err(Box::new(e)),
        };

        let projects_list: Vec<Project> = serde_json::from_value::<Vec<Project>>(
            store.get(env::STORE_PROJECTS).unwrap_or(json!([])),
        )
        .unwrap_or_else(|e| {
            error!("Error parsing projects from store: {}", e);
            log(
                &app_handle,
                ErrorLevel::Error,
                &format!("Error parsing projects from store: {}", e),
            );
            vec![]
        });

        Ok(projects_list)
    }
    
    pub fn save_projects(
        app_handle: &tauri::AppHandle,
        projects: &[Project],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let store = match app_handle.store(env::STORE_FILE_NAME) {
            Ok(store) => store,
            Err(e) => return Err(Box::new(e)),
        };

        let projects_json = serde_json::to_value(projects)?;
        store.set(env::STORE_PROJECTS, projects_json);
        store.save()?;

        Self::emit_project_updated(app_handle)?;

        Ok(())
    }
    
    pub fn add_projects(
        app_handle: &tauri::AppHandle,
        projects: &[Project],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut known_projects = Project::get_projects(app_handle)?;
        
        // Check if the project already exists
        for project in projects {
            if !known_projects.iter().any(|p| p.path == project.path) {
                known_projects.push(project.clone());
            }
        }

        Project::save_projects(app_handle, &known_projects)?;
        
        Ok(())
    }
    
    pub fn remove_projects(
        app_handle: &tauri::AppHandle,
        project_paths: &[PathBuf],
    ) -> Result<(), Box<dyn std::error::Error>>
    {
        let mut known_projects = Project::get_projects(app_handle)?;
        
        // Filter out the projects that are in the project_paths
        known_projects.retain(|p| !project_paths.contains(&p.path));
        
        Project::save_projects(app_handle, &known_projects)?;
        
        Ok(())
    }
    
    pub fn emit_project_updated(
        app_handle: &tauri::AppHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        app_handle.emit(env::EVENT_PROJECTS_UPDATED, ProjectsUpdatedPayload{
            projects: Project::get_projects(app_handle)?
        })?;
        Ok(())
    }
}
