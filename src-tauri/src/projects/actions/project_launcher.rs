use crate::misc::errors::ErrorLevel;
use crate::misc::prelude::log;
use log::{info};
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::{command, AppHandle};

/// Launch a project with Unreal Engine
#[command]
pub async fn launch_project_with_engine(
    app_handle: AppHandle,
    project_path: String,
) -> Result<(), String> {
    let project_path = PathBuf::from(project_path);

    if !project_path.exists() {
        let error_msg = format!("Project file does not exist: {}", project_path.display());
        log(&app_handle, ErrorLevel::Error, &error_msg);
        return Err(error_msg);
    }

    log(
        &app_handle,
        ErrorLevel::Info,
        &format!("Launching project: {}", project_path.display()),
    );

    let result = if cfg!(target_os = "windows") {
        // On Windows, we can directly open the .uproject file
        Command::new("cmd")
            .args(["/C", "start", "", project_path.to_str().unwrap()])
            .spawn()
    } else if cfg!(target_os = "macos") {
        // On macOS, use the open command
        Command::new("open").arg(project_path).spawn()
    } else {
        // On Linux, try to open with the default application
        Command::new("xdg-open").arg(project_path).spawn()
    };

    match result {
        Ok(_) => {
            log(
                &app_handle,
                ErrorLevel::Info,
                "Project launched successfully",
            );
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to launch project: {}", e);
            log(&app_handle, ErrorLevel::Error, &error_msg);
            Err(error_msg)
        }
    }
}

/// Launch a project with IDE
#[command]
pub async fn launch_project_with_ide(
    app_handle: AppHandle,
    project_path: String,
    ide_path: String,
) -> Result<(), String> {
    let project_path = PathBuf::from(project_path);
    let ide_path = PathBuf::from(ide_path);

    if !project_path.exists() {
        let error_msg = format!("Project file does not exist: {}", project_path.display());
        log(&app_handle, ErrorLevel::Error, &error_msg);
        return Err(error_msg);
    }

    if !ide_path.exists() {
        let error_msg = format!("IDE executable does not exist: {}", ide_path.display());
        log(&app_handle, ErrorLevel::Error, &error_msg);
        return Err(error_msg);
    }

    // Try to find the .sln file first
    let project_dir = project_path.parent().unwrap();
    let sln_file = find_sln_file(project_dir);

    let file_to_open = if let Some(sln_path) = sln_file {
        info!("Found .sln file: {}", sln_path.display());
        sln_path
    } else {
        info!("No .sln file found, using .uproject file");
        project_path.clone()
    };

    log(
        &app_handle,
        ErrorLevel::Info,
        &format!("Launching project with IDE: {}", file_to_open.display()),
    );

    let result = Command::new(&ide_path).arg(&file_to_open).spawn();

    match result {
        Ok(_) => {
            log(
                &app_handle,
                ErrorLevel::Info,
                "Project launched with IDE successfully",
            );
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to launch project with IDE: {}", e);
            log(&app_handle, ErrorLevel::Error, &error_msg);
            Err(error_msg)
        }
    }
}

/// Launch custom engine with IDE
#[command]
pub async fn launch_custom_engine_with_ide(
    app_handle: AppHandle,
    custom_engine_dir: String,
    ide_path: String,
) -> Result<(), String> {
    let custom_engine_dir = PathBuf::from(custom_engine_dir);
    let ide_path = PathBuf::from(ide_path);

    if !custom_engine_dir.exists() {
        let error_msg = format!(
            "Custom engine directory does not exist: {}",
            custom_engine_dir.display()
        );
        log(&app_handle, ErrorLevel::Error, &error_msg);
        return Err(error_msg);
    }

    if !ide_path.exists() {
        let error_msg = format!("IDE executable does not exist: {}", ide_path.display());
        log(&app_handle, ErrorLevel::Error, &error_msg);
        return Err(error_msg);
    }

    // Look for the .sln file in the custom engine directory
    let sln_file = find_sln_file(&custom_engine_dir);

    let file_to_open = if let Some(sln_path) = sln_file {
        info!("Found custom engine .sln file: {}", sln_path.display());
        sln_path
    } else {
        let error_msg = format!(
            "No .sln file found in custom engine directory: {}",
            custom_engine_dir.display()
        );
        log(&app_handle, ErrorLevel::Error, &error_msg);
        return Err(error_msg);
    };

    log(
        &app_handle,
        ErrorLevel::Info,
        &format!(
            "Launching custom engine with IDE: {}",
            file_to_open.display()
        ),
    );

    let result = Command::new(&ide_path).arg(&file_to_open).spawn();

    match result {
        Ok(_) => {
            log(
                &app_handle,
                ErrorLevel::Info,
                "Custom engine launched with IDE successfully",
            );
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to launch custom engine with IDE: {}", e);
            log(&app_handle, ErrorLevel::Error, &error_msg);
            Err(error_msg)
        }
    }
}

/// Find .sln file in the project directory
fn find_sln_file(project_dir: &Path) -> Option<PathBuf> {
    if let Ok(entries) = std::fs::read_dir(project_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sln") {
                return Some(path);
            }
        }
    }
    None
}

/// Check if a project has C++ code by looking for the Source directory
#[command]
pub fn project_has_cpp(project_path: String) -> Result<bool, String> {
    let project_path = PathBuf::from(project_path);

    if let Some(project_dir) = project_path.parent() {
        let source_dir = project_dir.join("Source");
        Ok(source_dir.exists())
    } else {
        Err("Invalid project path".to_string())
    }
}
