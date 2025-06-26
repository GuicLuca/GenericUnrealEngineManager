use crate::misc::errors::{ErrorLevel, Result};
use crate::misc::prelude::{format_size, log};
use crate::misc::progress::TaskProgress;
use crate::projects::actions::project_cleaner::{CleaningSelection, clean_project};
use log::{error};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::{command, AppHandle};
use crate::misc::errors::Verror::MessageError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionRequest {
    pub project_path: String,
    pub destination_path: String,
    pub compression_algorithm: CompressionAlgorithm,
    pub clean_before_compress: bool,
    pub cleaning_selection: Option<CleaningSelection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Zip,
    SevenZip,
    Tar,
    TarGz,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    pub output_path: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub duration_ms: u128,
}

/// Compress a project with optional cleaning
#[command]
pub async fn compress_project(
    app_handle: AppHandle,
    request: CompressionRequest,
) -> Result<CompressionResult> {
    let start_time = std::time::Instant::now();
    let project_path = PathBuf::from(&request.project_path);
    
    if !project_path.exists() {
        let error_msg = format!("Project file does not exist: {}", project_path.display());
        error!("{}", error_msg);
        log(&app_handle, ErrorLevel::Error, &error_msg);
        return Err(MessageError(error_msg));
    }
    
    let project_dir = project_path.parent().unwrap();
    let project_name = project_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown");
    
    let task_id = format!("compress_project_{}", chrono::Utc::now().timestamp_millis());
    let progress = TaskProgress::new(
        app_handle.clone(),
        task_id,
        format!("Compressing project: {}", project_name)
    );
    
    log(&app_handle, ErrorLevel::Info, &format!("Starting compression process for project: {}", project_name));
    
    // Clean the project if requested
    if request.clean_before_compress {
        if let Some(cleaning_selection) = request.cleaning_selection {
            progress.update(0.1, Some("Cleaning project before compression...".to_string()));
            log(&app_handle, ErrorLevel::Info, "Cleaning project before compression...");
            
            match clean_project(app_handle.clone(), request.project_path.clone(), cleaning_selection).await {
                Ok(_) => {
                    log(&app_handle, ErrorLevel::Info, "Project cleaned successfully before compression");
                }
                Err(e) => {
                    let error_msg = format!("Failed to clean project before compression: {}", e);
                    log(&app_handle, ErrorLevel::Error, &error_msg);
                    progress.fail(Some(error_msg.clone()));
                    return Err(e);
                }
            }
        }
    }
    
    progress.update(0.3, Some("Calculating project size...".to_string()));
    
    // Get the original size
    let original_size = fs_extra::dir::get_size(project_dir).unwrap_or(0);
    
    // Generate output filename with timestamp
    let now = chrono::Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M").to_string();
    let extension = get_extension_for_algorithm(&request.compression_algorithm);
    let output_filename = format!("{}_{}.{}", project_name, timestamp, extension);
    let output_path = PathBuf::from(&request.destination_path).join(output_filename);
    
    log(&app_handle, ErrorLevel::Info, &format!("Compressing to: {}", output_path.display()));
    
    progress.update(0.4, Some(format!("Creating {} archive...", get_algorithm_name(&request.compression_algorithm))));
    
    // Perform compression based on the selected algorithm
    match compress_directory(project_dir, &output_path, &request.compression_algorithm, &progress) {
        Ok(_) => {
            progress.update(0.9, Some("Finalizing compression...".to_string()));
            
            let compressed_size = fs::metadata(&output_path)
                .map(|m| m.len())
                .unwrap_or(0);
            
            let compression_ratio = if original_size > 0 {
                (compressed_size as f64 / original_size as f64) * 100.0
            } else {
                0.0
            };
            
            let duration = start_time.elapsed();
            
            let result = CompressionResult {
                output_path: output_path.to_string_lossy().to_string(),
                original_size,
                compressed_size,
                compression_ratio,
                duration_ms: duration.as_millis(),
            };
            
            let completion_msg = format!(
                "Compression completed for {}. Original: {}, Compressed: {} ({:.1}% of original)",
                project_name,
                format_size(original_size),
                format_size(compressed_size),
                compression_ratio
            );
            
            log(&app_handle, ErrorLevel::Info, &completion_msg);
            progress.complete(Some(format!("Compressed to {} ({:.1}% of original)", format_size(compressed_size), compression_ratio)));
            
            Ok(result)
        }
        Err(e) => {
            let error_msg = format!("Failed to compress project: {}", e);
            log(&app_handle, ErrorLevel::Error, &error_msg);
            progress.fail(Some(error_msg.clone()));
            Err(MessageError(error_msg))
        }
    }
}

fn compress_directory(
    source_dir: &Path,
    output_path: &Path,
    algorithm: &CompressionAlgorithm,
    progress: &TaskProgress,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    match algorithm {
        CompressionAlgorithm::Zip => {
            progress.update(0.5, Some("Creating ZIP archive...".to_string()));
            compress_with_zip(source_dir, output_path)
        },
        CompressionAlgorithm::SevenZip => {
            progress.update(0.5, Some("Creating 7-Zip archive...".to_string()));
            compress_with_7zip(source_dir, output_path)
        },
        CompressionAlgorithm::Tar => {
            progress.update(0.5, Some("Creating TAR archive...".to_string()));
            compress_with_tar(source_dir, output_path, false)
        },
        CompressionAlgorithm::TarGz => {
            progress.update(0.5, Some("Creating TAR.GZ archive...".to_string()));
            compress_with_tar(source_dir, output_path, true)
        },
    }
}

fn compress_with_zip(
    source_dir: &Path,
    output_path: &Path,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Use the system zip command if available
    if cfg!(target_os = "windows") {
        // On Windows, try PowerShell Compress-Archive
        let result = Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "Compress-Archive -Path '{}\\*' -DestinationPath '{}'",
                    source_dir.display(),
                    output_path.display()
                ),
            ])
            .output()?;
        
        if !result.status.success() {
            return Err(format!(
                "PowerShell compression failed: {}",
                String::from_utf8_lossy(&result.stderr)
            ).into());
        }
    } else {
        // On Unix systems, use the zip command
        let result = Command::new("zip")
            .args([
                "-r",
                output_path.to_str().unwrap(),
                ".",
            ])
            .current_dir(source_dir)
            .output()?;
        
        if !result.status.success() {
            return Err(format!(
                "Zip compression failed: {}",
                String::from_utf8_lossy(&result.stderr)
            ).into());
        }
    }
    
    Ok(())
}

fn compress_with_7zip(
    source_dir: &Path,
    output_path: &Path,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let result = Command::new("7z")
        .args([
            "a",
            "-t7z",
            output_path.to_str().unwrap(),
            &format!("{}/*", source_dir.display()),
        ])
        .output()?;
    
    if !result.status.success() {
        return Err(format!(
            "7-Zip compression failed: {}",
            String::from_utf8_lossy(&result.stderr)
        ).into());
    }
    
    Ok(())
}

fn compress_with_tar(
    source_dir: &Path,
    output_path: &Path,
    use_gzip: bool,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut args = vec!["czf"];
    if !use_gzip {
        args = vec!["cf"];
    }
    
    args.push(output_path.to_str().unwrap());
    args.push(".");
    
    let result = Command::new("tar")
        .args(args)
        .current_dir(source_dir)
        .output()?;
    
    if !result.status.success() {
        return Err(format!(
            "Tar compression failed: {}",
            String::from_utf8_lossy(&result.stderr)
        ).into());
    }
    
    Ok(())
}

fn get_extension_for_algorithm(algorithm: &CompressionAlgorithm) -> &'static str {
    match algorithm {
        CompressionAlgorithm::Zip => "zip",
        CompressionAlgorithm::SevenZip => "7z",
        CompressionAlgorithm::Tar => "tar",
        CompressionAlgorithm::TarGz => "tar.gz",
    }
}

fn get_algorithm_name(algorithm: &CompressionAlgorithm) -> &'static str {
    match algorithm {
        CompressionAlgorithm::Zip => "ZIP",
        CompressionAlgorithm::SevenZip => "7-Zip",
        CompressionAlgorithm::Tar => "TAR",
        CompressionAlgorithm::TarGz => "TAR.GZ",
    }
}

/// Get available compression algorithms based on system capabilities
#[command]
pub fn get_available_compression_algorithms() -> Vec<CompressionAlgorithm> {
    let mut algorithms = Vec::new();
    
    // Zip is available on most systems
    if cfg!(target_os = "windows") {
        // PowerShell Compress-Archive is available on Windows 10+
        algorithms.push(CompressionAlgorithm::Zip);
    } else {
        // Check if the zip command is available
        if Command::new("zip").arg("--help").output().is_ok() {
            algorithms.push(CompressionAlgorithm::Zip);
        }
    }
    
    // Check for 7-Zip
    if Command::new("7z").output().is_ok() {
        algorithms.push(CompressionAlgorithm::SevenZip);
    }
    
    // Check for tar (usually available on Unix systems)
    if Command::new("tar").arg("--help").output().is_ok() {
        algorithms.push(CompressionAlgorithm::Tar);
        algorithms.push(CompressionAlgorithm::TarGz);
    }
    
    // If no algorithms are available, at least offer zip as fallback
    if algorithms.is_empty() {
        algorithms.push(CompressionAlgorithm::Zip);
    }
    
    algorithms
}