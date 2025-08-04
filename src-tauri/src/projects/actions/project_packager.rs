use crate::misc::prelude::*;
use crate::misc::progress::ProgressManager;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::{AppHandle, Manager};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageRequest {
    pub project_path: String,
    pub build_type: String,
    pub target_platform: String,
    pub output_directory: String,
    pub cook_content: bool,
    pub pak_files: bool,
    pub include_prerequisites: bool,
    pub include_debug_files: bool,
    pub create_release_version: bool,
    pub compress_after_build: bool,
    pub compression_algorithm: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageResult {
    pub success: bool,
    pub output_path: String,
    pub compressed_path: Option<String>,
    pub build_duration_ms: u64,
    pub total_size_bytes: u64,
}

pub async fn package_project(
    app_handle: AppHandle,
    request: PackageRequest,
) -> Result<PackageResult> {
    let start_time = std::time::Instant::now();
    let task_id = format!("package_project_{}", uuid::Uuid::new_v4());
    let project_name = extract_project_name(&request.project_path)?;
    
    let progress_manager = ProgressManager::new(app_handle.clone());
    
    // Start the packaging task
    progress_manager.start_task(
        &task_id,
        &format!("Packaging {} for {}", project_name, request.target_platform),
        "Initializing packaging process...",
    ).await?;

    let result = tokio::task::spawn_blocking({
        let app_handle = app_handle.clone();
        let progress_manager = progress_manager.clone();
        let task_id = task_id.clone();
        let request = request.clone();
        
        move || {
            tokio::runtime::Handle::current().block_on(async {
                package_project_internal(app_handle, progress_manager, task_id, request).await
            })
        }
    }).await??;

    Ok(result)
}

async fn package_project_internal(
    app_handle: AppHandle,
    progress_manager: ProgressManager,
    task_id: String,
    request: PackageRequest,
) -> Result<PackageResult> {
    let start_time = std::time::Instant::now();
    
    // Validate project path
    let project_path = Path::new(&request.project_path);
    if !project_path.exists() {
        return Err(AppError::InvalidInput(format!(
            "Project file not found: {}",
            request.project_path
        )));
    }

    // Validate output directory
    let output_dir = Path::new(&request.output_directory);
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).await.map_err(|e| {
            AppError::IoError(format!("Failed to create output directory: {}", e))
        })?;
    }

    // Find engine installation
    progress_manager.update_task(&task_id, 0.1, "Finding Unreal Engine installation...").await?;
    
    let engine_path = find_engine_for_project(&request.project_path).await?;
    let uat_script_path = get_uat_script_path(&engine_path)?;

    // Build RunUAT command
    progress_manager.update_task(&task_id, 0.2, "Building packaging command...").await?;
    
    let mut command = build_package_command(&uat_script_path, &request)?;
    
    // Execute packaging
    progress_manager.update_task(&task_id, 0.3, "Starting Unreal Engine packaging...").await?;
    
    let output = command.output().map_err(|e| {
        AppError::ProcessError(format!("Failed to execute RunUAT: {}", e))
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        progress_manager.fail_task(&task_id, "Packaging failed").await?;
        
        return Err(AppError::ProcessError(format!(
            "RunUAT packaging failed:\nStdout: {}\nStderr: {}",
            stdout, stderr
        )));
    }

    progress_manager.update_task(&task_id, 0.8, "Packaging completed, calculating size...").await?;

    // Calculate output size
    let packaged_path = get_packaged_output_path(&request)?;
    let total_size = calculate_directory_size(&packaged_path).await?;

    let mut result = PackageResult {
        success: true,
        output_path: packaged_path.to_string_lossy().to_string(),
        compressed_path: None,
        build_duration_ms: start_time.elapsed().as_millis() as u64,
        total_size_bytes: total_size,
    };

    // Compress if requested
    if request.compress_after_build {
        progress_manager.update_task(&task_id, 0.9, "Compressing packaged build...").await?;
        
        let compressed_path = compress_packaged_build(&request, &packaged_path).await?;
        result.compressed_path = Some(compressed_path);
    }

    progress_manager.complete_task(&task_id, "Packaging completed successfully").await?;
    
    Ok(result)
}

fn build_package_command(uat_script_path: &Path, request: &PackageRequest) -> Result<Command> {
    let mut command = Command::new(uat_script_path);
    
    // Basic BuildCookRun command
    command.arg("BuildCookRun");
    
    // Project path
    command.arg("-project").arg(&request.project_path);
    
    // Target platform
    command.arg("-targetplatform").arg(&request.target_platform);
    
    // Build configuration
    command.arg("-configuration").arg(&request.build_type);
    
    // Output directory
    command.arg("-archivedirectory").arg(&request.output_directory);
    
    // Common flags
    command.arg("-build");
    command.arg("-archive");
    command.arg("-stage");
    command.arg("-package");
    
    // Conditional flags
    if request.cook_content {
        command.arg("-cook");
    } else {
        command.arg("-skipcook");
    }
    
    if request.pak_files {
        command.arg("-pak");
    }
    
    if !request.include_debug_files {
        command.arg("-nodebuginfo");
    }
    
    if request.include_prerequisites {
        command.arg("-prereqs");
    }
    
    if request.create_release_version {
        command.arg("-createreleaseversion");
    }
    
    // Platform-specific optimizations
    match request.target_platform.as_str() {
        "Win64" => {
            command.arg("-platform=Win64");
        }
        "Mac" => {
            command.arg("-platform=Mac");
        }
        "Linux" => {
            command.arg("-platform=Linux");
        }
        "Android" => {
            command.arg("-platform=Android");
            command.arg("-cookflavor=ASTC");
        }
        "iOS" => {
            command.arg("-platform=IOS");
        }
        _ => {}
    }
    
    // Disable unnecessary features for faster builds
    command.arg("-nocompileeditor");
    command.arg("-utf8output");
    
    Ok(command)
}

fn get_uat_script_path(engine_path: &Path) -> Result<PathBuf> {
    let script_name = if cfg!(target_os = "windows") {
        "RunUAT.bat"
    } else {
        "RunUAT.sh"
    };
    
    let uat_path = engine_path
        .join("Engine")
        .join("Build")
        .join("BatchFiles")
        .join(script_name);
    
    if !uat_path.exists() {
        return Err(AppError::NotFound(format!(
            "RunUAT script not found at: {}",
            uat_path.display()
        )));
    }
    
    Ok(uat_path)
}

async fn find_engine_for_project(project_path: &str) -> Result<PathBuf> {
    // This is a simplified version - in a real implementation, you'd:
    // 1. Parse the .uproject file to get engine association
    // 2. Look up the engine path from registry/settings
    // 3. Handle custom engines
    
    // For now, we'll try to find a common engine installation
    let common_paths = if cfg!(target_os = "windows") {
        vec![
            r"C:\Program Files\Epic Games\UE_5.4",
            r"C:\Program Files\Epic Games\UE_5.3",
            r"C:\Program Files\Epic Games\UE_5.2",
            r"C:\Program Files\Epic Games\UE_5.1",
        ]
    } else if cfg!(target_os = "macos") {
        vec![
            "/Users/Shared/Epic Games/UE_5.4",
            "/Users/Shared/Epic Games/UE_5.3",
            "/Users/Shared/Epic Games/UE_5.2",
            "/Users/Shared/Epic Games/UE_5.1",
        ]
    } else {
        vec![
            "/opt/UnrealEngine/UE_5.4",
            "/opt/UnrealEngine/UE_5.3",
            "/opt/UnrealEngine/UE_5.2",
            "/opt/UnrealEngine/UE_5.1",
        ]
    };
    
    for path_str in common_paths {
        let path = Path::new(path_str);
        if path.exists() {
            return Ok(path.to_path_buf());
        }
    }
    
    Err(AppError::NotFound(
        "No Unreal Engine installation found. Please ensure Unreal Engine is installed.".to_string()
    ))
}

fn get_packaged_output_path(request: &PackageRequest) -> Result<PathBuf> {
    let project_name = extract_project_name(&request.project_path)?;
    
    let output_path = Path::new(&request.output_directory)
        .join(&request.target_platform)
        .join(&project_name);
    
    Ok(output_path)
}

async fn calculate_directory_size(dir_path: &Path) -> Result<u64> {
    let mut total_size = 0u64;
    
    if !dir_path.exists() {
        return Ok(0);
    }
    
    let mut entries = fs::read_dir(dir_path).await.map_err(|e| {
        AppError::IoError(format!("Failed to read directory: {}", e))
    })?;
    
    while let Some(entry) = entries.next_entry().await.map_err(|e| {
        AppError::IoError(format!("Failed to read directory entry: {}", e))
    })? {
        let metadata = entry.metadata().await.map_err(|e| {
            AppError::IoError(format!("Failed to read file metadata: {}", e))
        })?;
        
        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            total_size += calculate_directory_size(&entry.path()).await?;
        }
    }
    
    Ok(total_size)
}

async fn compress_packaged_build(request: &PackageRequest, packaged_path: &Path) -> Result<String> {
    // This would use the same compression logic as the compress feature
    // For now, we'll return a placeholder
    let compressed_filename = format!(
        "{}_{}_{}.zip",
        extract_project_name(&request.project_path)?,
        request.target_platform,
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );
    
    let compressed_path = Path::new(&request.output_directory).join(compressed_filename);
    
    // TODO: Implement actual compression using the compression algorithm
    // This would call the same compression functions used in project_compressor.rs
    
    Ok(compressed_path.to_string_lossy().to_string())
}

fn extract_project_name(project_path: &str) -> Result<String> {
    let path = Path::new(project_path);
    let file_stem = path.file_stem()
        .ok_or_else(|| AppError::InvalidInput("Invalid project path".to_string()))?;
    
    Ok(file_stem.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn package_project_command(
    app_handle: AppHandle,
    request: PackageRequest,
) -> Result<PackageResult> {
    package_project(app_handle, request).await
}

#[tauri::command]
pub async fn get_current_platform() -> Result<String> {
    let platform = if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else {
        "Unknown"
    };
    
    Ok(platform.to_string())
}