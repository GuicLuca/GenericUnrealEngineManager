use tauri::command;
use std::process::Command;
use log::info;
use crate::projects::actions::project_cleaner::ProjectCleaner;
use crate::projects::actions::project_compressor::ProjectCompressor;
use crate::projects::actions::project_discovery::ProjectDiscovery;
use crate::projects::actions::project_launcher::ProjectLauncher;
use crate::projects::actions::project_packager::{ProjectPackager, PackageRequest};
use crate::projects::actions::plugin_manager::PluginManager;

#[command]
pub async fn open_file_explorer(path: String) -> Result<(), String> {
    let result = if cfg!(target_os = "windows") {
        // For Windows, use the explorer command
        info!("Opening file explorer at path: {}", path);
        let windows_path = path.replace("/", "\\"); // Ensure backslashes for Windows paths

        Command::new("explorer")
            .arg(windows_path)
            .spawn()
    } else if cfg!(target_os = "macos") {
        // For macOS, use the open command
        Command::new("open")
            .arg(path)
            .spawn()
    } else {
        // For Linux, use xdg-open
        Command::new("xdg-open")
            .arg(path)
            .spawn()
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to open file explorer: {}", e)),
    }
}

#[tauri::command]
pub async fn package_project(
    app_handle: AppHandle,
    request: PackageRequest,
) -> Result<(), String> {
    let packager = ProjectPackager::new(app_handle);
    
    // Run packaging in background
    tokio::spawn(async move {
        if let Err(e) = packager.package_project(request).await {
            eprintln!("Packaging failed: {}", e);
        }
    });
    
    Ok(())
}