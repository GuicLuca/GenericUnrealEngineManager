use crate::misc::errors::Result;
use crate::misc::errors::Verror::MessageError;
use crate::misc::payloads::{CompressionRequest, CompressionResult, PackageRequest, PackageResult};
use crate::misc::progress::TaskProgress;
use crate::projects::actions::project_compressor;
use crate::projects::actions::engine_discovery;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tauri::AppHandle;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
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
    let command_args = build_package_command_args(&runuat_path, &request)?;

    // Execute packaging with real-time output streaming
    progress_manager.update(0.5, Some("Starting Unreal Engine packaging...".to_string()));

    let package_start = std::time::Instant::now();
    let exit_status = execute_with_streaming(
        &runuat_path,
        &command_args,
        app_handle,
        &format!("[BUILD-{}]", request.project.name),
    ).await?;
    let package_duration = package_start.elapsed();

    if !exit_status.success() {
        return Err(MessageError(format!(
            "Packaging failed with exit code: {:?}",
            exit_status.code()
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


async fn execute_with_streaming(
    runuat_path: &Path,
    args: &[String],
    app_handle: &AppHandle,
    log_prefix: &str,
) -> Result<std::process::ExitStatus> {
    let mut command = if cfg!(target_os = "windows") {
        let mut cmd = tokio::process::Command::new("cmd");
        cmd.args(&["/C", &runuat_path.to_string_lossy()]);
        cmd
    } else {
        let mut cmd = tokio::process::Command::new("bash");
        cmd.arg(&runuat_path.as_os_str());
        cmd
    };

    // Add all arguments
    command.args(args);

    // Configure to capture stdout and stderr
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    // Hide console window on Windows
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    // Spawn the process
    let mut child = command
        .spawn()
        .map_err(|e| MessageError(format!("Failed to spawn process: {}", e)))?;

    // Get stdout and stderr handles
    let stdout = child.stdout.take()
        .ok_or_else(|| MessageError("Failed to capture stdout".to_string()))?;
    let stderr = child.stderr.take()
        .ok_or_else(|| MessageError("Failed to capture stderr".to_string()))?;

    // Create async readers
    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let app_handle_stdout = app_handle.clone();
    let app_handle_stderr = app_handle.clone();
    let prefix_stdout = log_prefix.to_string();
    let prefix_stderr = log_prefix.to_string();

    // Spawn tasks to read stdout and stderr concurrently
    let stdout_task = tokio::spawn(async move {
        let mut lines = stdout_reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let message = format!("{} {}", prefix_stdout, line);
            let _ = app_handle_stdout.emit("log-message", serde_json::json!({
                "message": message,
                "level": "info"
            }));
        }
    });

    let stderr_task = tokio::spawn(async move {
        let mut lines = stderr_reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let message = format!("{} {}", prefix_stderr, line);
            let _ = app_handle_stderr.emit("log-message", serde_json::json!({
                "message": message,
                "level": "info"
            }));
        }
    });

    // Wait for the process to complete
    let status = child.wait().await
        .map_err(|e| MessageError(format!("Failed to wait for process: {}", e)))?;

    // Wait for output tasks to complete
    let _ = stdout_task.await;
    let _ = stderr_task.await;

    Ok(status)
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

fn build_package_command_args(
    _runuat_path: &Path,
    request: &PackageRequest,
) -> Result<Vec<String>> {
    let mut args = Vec::new();

    // Basic BuildCookRun command
    args.push("BuildCookRun".to_string());

    // Use format! to create the arguments with proper path handling
    args.push(format!("-project={}", request.project.path.display()));
    args.push(format!("-platform={}", request.target_platform));
    args.push(format!("-configuration={}", request.build_type));
    args.push(format!("-archivedirectory={}", request.output_directory));

    // Standard flags
    args.push("-build".to_string());
    args.push("-cook".to_string());
    args.push("-stage".to_string());
    args.push("-archive".to_string());
    args.push("-unattended".to_string());
    args.push("-nop4".to_string());

    // Platform-specific optimizations
    match request.target_platform.as_str() {
        "Win64" => {
            args.push("-targetplatform=Win64".to_string());
        }
        "Mac" => {
            args.push("-targetplatform=Mac".to_string());
        }
        "Linux" => {
            args.push("-targetplatform=Linux".to_string());
        }
        "Android" => {
            args.push("-targetplatform=Android".to_string());
            args.push("-cookflavor=ASTC".to_string());
        }
        "iOS" => {
            args.push("-targetplatform=IOS".to_string());
        }
        _ => {}
    }

    Ok(args)
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
