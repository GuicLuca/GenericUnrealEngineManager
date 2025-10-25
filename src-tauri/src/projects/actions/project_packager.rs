use crate::misc::errors::Result;
use crate::misc::errors::Verror::MessageError;
use crate::misc::payloads::{CompressionRequest, CompressionResult, PackageRequest, PackageResult};
use crate::misc::progress::TaskProgress;
use crate::projects::actions::project_compressor;
use crate::projects::actions::engine_discovery;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::AppHandle;
use tokio::fs;
use crate::projects::models::project::Project;

#[tauri::command]
pub async fn package_project(
    app_handle: AppHandle,
    request: PackageRequest,
) -> std::result::Result<(), String> {
    // Extract the project name from the project path

    let task_id = format!("package_project_{}", uuid::Uuid::new_v4());
    let task_name = format!("Packaging {} for {}", request.project.name, request.target_platform);

    // Start progress tracking
    let progress_manager =
        TaskProgress::new(app_handle.clone(), task_id.clone(), task_name.clone());

    progress_manager.update(
        0.1,
        Some("Initializing packaging process...".to_string()),
    );

    let result = package_project_worker(&app_handle, request.clone(), &progress_manager)
        .await;

    match result {
        Ok(_package_result) => {
            progress_manager.complete(Some("Packaging completed successfully".to_string()));
            Ok(())
        }
        Err(e) => {
            progress_manager.fail(Some(format!("Packaging failed: {}", e)));
            Err(e.to_string())
        }
    }
}

async fn package_project_worker(
    app_handle: &AppHandle,
    request: PackageRequest,
    progress_manager: &TaskProgress,
) -> Result<PackageResult> {
    let start_time = std::time::Instant::now();

    // Validate project path
    let project_path = Path::new(&request.project.path);
    if !project_path.exists() {
        return Err(MessageError(format!(
            "Project file does not exist: {}",
            request.project.path.display()
        )));
    }

    // Find engine installation
    progress_manager.update(
        0.2,
        Some("Finding Unreal Engine installation...".to_string()),
    );

    let engine_path_str = engine_discovery::find_engine_for_project(
        app_handle.clone(),
        request.project.path.display().to_string(),
    )?
    .ok_or_else(|| MessageError(format!(
        "No compatible engine found for project requiring version: {}",
        request.project.engine_association.display()
    )))?;

    let engine_path = PathBuf::from(engine_path_str);

    // Prepare output directory
    progress_manager.update(0.3, Some("Preparing output directory...".to_string()));

    let output_dir = Path::new(&request.output_directory);
    fs::create_dir_all(output_dir).await?;

    // Build RunUAT command
    progress_manager.update(0.4, Some("Building packaging command...".to_string()));

    let runuat_path = get_runuat_path(&engine_path)?;
    let mut command = build_package_command(&runuat_path, &request)?;

    // Execute packaging
    progress_manager.update(0.5, Some("Starting Unreal Engine packaging...".to_string()));

    let package_start = std::time::Instant::now();
    let output = command.output()?;
    let package_duration = package_start.elapsed();

    if !output.status.success() {
        return Err(MessageError(format!(
            "Packaging failed. Stdout: {}\nStderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    progress_manager.update(0.8, Some("Packaging completed, finalizing...".to_string()));

    // Determine the actual output path (Output directory + platform subfolder)
    let output_path = find_packaged_output(output_dir, &request.target_platform)?;

    let mut archive_path = None;

    // Create an archive if requested
    if request.create_archive {
        progress_manager.update(0.9, Some("Creating archive...".to_string()));

        let archive_result = Some(create_archive(app_handle, &request).await?);
        if let Some(archive) = &archive_result {
            archive_path = Some(archive.output_path.clone());
        }
    }

    let total_duration = start_time.elapsed();

    Ok(PackageResult {
        success: true,
        output_path: output_path.to_string_lossy().to_string(),
        archive_path,
        package_duration_ms: package_duration.as_millis() as u64,
        total_duration_ms: total_duration.as_millis() as u64,
    })
}


fn get_runuat_path(engine_path: &Path) -> Result<PathBuf> {
    let runuat_path = if cfg!(target_os = "windows") {
        engine_path
            .join("Engine")
            .join("Build")
            .join("BatchFiles")
            .join("RunUAT.bat")
    } else {
        engine_path
            .join("Engine")
            .join("Build")
            .join("BatchFiles")
            .join("RunUAT.sh")
    };

    if !runuat_path.exists() {
        return Err(MessageError(format!(
            "RunUAT script not found at: {}",
            runuat_path.display()
        )));
    }

    Ok(runuat_path)
}

fn build_package_command(
    runuat_path: &Path,
    request: &PackageRequest,
) -> Result<Command> {
    let mut command = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.args(&["/C", &runuat_path.to_string_lossy()]);
        cmd
    } else {
        let mut cmd = Command::new("bash");
        cmd.arg(&runuat_path.as_os_str());
        cmd
    };

    // Basic BuildCookRun command
    command.arg("BuildCookRun");
    command.args(&["-project", request.project.path.as_os_str().to_string_lossy().trim()]);
    command.args(&["-platform", &request.target_platform]);
    command.args(&["-configuration", &request.build_type]);
    command.args(&["-archivedirectory", &request.output_directory]);

    // Standard flags
    command.arg("-build");
    command.arg("-cook");
    command.arg("-stage");
    command.arg("-archive");
    command.arg("-unattended");
    command.arg("-nop4");

    // Platform-specific optimizations
    match request.target_platform.as_str() {
        "Win64" => {
            command.arg("-targetplatform=Win64");
        }
        "Mac" => {
            command.arg("-targetplatform=Mac");
        }
        "Linux" => {
            command.arg("-targetplatform=Linux");
        }
        "Android" => {
            command.arg("-targetplatform=Android");
            command.arg("-cookflavor=ASTC");
        }
        "iOS" => {
            command.arg("-targetplatform=IOS");
        }
        _ => {}
    }

    Ok(command)
}

fn find_packaged_output(output_dir: &Path, platform: &str) -> Result<PathBuf> {
    // The packaged output is typically in a subdirectory named after the platform
    let platform_dir = output_dir.join(platform);
    if platform_dir.exists() {
        return Ok(platform_dir);
    }

    // Fallback: return the output directory itself
    Ok(output_dir.to_path_buf())
}

async fn create_archive(app_handle: &AppHandle, request: &PackageRequest) -> Result<CompressionResult> {
    if request.archive_format.is_none() {
        return Err(MessageError(
            "Archive format not specified for archiving".to_string(),
        ));
    }

    // Use the existing compression functionality
    let compression_request = CompressionRequest {
        project_path: request.project.path.to_string_lossy().to_string(),
        destination_path: request.output_directory.clone(),
        compression_algorithm: request.archive_format.clone().unwrap(),
        clean_before_compress: false,
        cleaning_selection: None,
    };

    let result =
        project_compressor::compress_project(app_handle.clone(), compression_request)
            .await?;

    Ok(result) 
}
