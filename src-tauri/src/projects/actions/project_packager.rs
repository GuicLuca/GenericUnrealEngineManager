use crate::misc::errors::Result;
use crate::misc::errors::Verror::MessageError;
use crate::misc::payloads::{CompressionRequest, CompressionResult, PackageRequest, PackageResult};
use crate::misc::progress::TaskProgress;
use crate::projects::actions::project_compressor;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::AppHandle;
use tokio::fs;

pub struct ProjectPackager {
    app_handle: AppHandle,
}

impl ProjectPackager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub async fn package_project(&self, request: PackageRequest) -> Result<PackageResult> {
        let project_name = Path::new(&request.project_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .replace(".uproject", "");

        let task_id = format!("package_project_{}", uuid::Uuid::new_v4());
        let task_name = format!("Packaging {} for {}", project_name, request.target_platform);

        // Start progress tracking
        let progress_manager =
            TaskProgress::new(self.app_handle.clone(), task_id.clone(), task_name.clone());

        progress_manager.update(
            0.1f32,
            Some("Initializing packaging process...".to_string()),
        );

        let result = self
            .package_project_internal(request.clone(), &progress_manager)
            .await;

        match result {
            Ok(package_result) => {
                progress_manager.complete(Some("Packaging completed successfully".to_string()));
                Ok(package_result)
            }
            Err(e) => {
                progress_manager.fail(Some(format!("Packaging failed: {}", e)));
                Err(e)
            }
        }
    }

    async fn package_project_internal(
        &self,
        request: PackageRequest,
        progress_manager: &TaskProgress,
    ) -> Result<PackageResult> {
        let start_time = std::time::Instant::now();

        // Validate project path
        let project_path = Path::new(&request.project_path);
        if !project_path.exists() {
            return Err(MessageError(format!(
                "Project file does not exist: {}",
                request.project_path
            )));
        }

        // Find engine installation
        progress_manager.update(
            0.1,
            Some("Finding Unreal Engine installation...".to_string()),
        );

        let engine_path = self.find_engine_for_project(project_path).await?;

        // Prepare output directory
        progress_manager.update(0.2, Some("Preparing output directory...".to_string()));

        let output_dir = Path::new(&request.output_directory);
        fs::create_dir_all(output_dir).await?;

        // Build RunUAT command
        progress_manager.update(0.3, Some("Building packaging command...".to_string()));

        let runuat_path = self.get_runuat_path(&engine_path)?;
        let mut command = self.build_package_command(&runuat_path, &request)?;

        // Execute packaging
        progress_manager.update(0.4, Some("Starting Unreal Engine packaging...".to_string()));

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

        // Determine the actual output path
        let output_path = self.find_packaged_output(&output_dir, &request.target_platform)?;

        let mut archive_path = None;

        // Create an archive if requested
        if request.create_archive {
            progress_manager.update(0.9, Some("Creating archive...".to_string()));

            let archive_result = Some(self.create_archive(&request).await?);
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

    async fn find_engine_for_project(&self, project_path: &Path) -> Result<PathBuf> {
        // Read the .uproject file to determine engine association
        let project_content = fs::read_to_string(project_path).await?;
        let project_json: serde_json::Value = serde_json::from_str(&project_content)?;

        if let Some(engine_association) = project_json.get("EngineAssociation") {
            if let Some(version) = engine_association.as_str() {
                if version == "Custom" {
                    // For custom engines, look in the parent directory
                    if let Some(parent) = project_path.parent() {
                        if let Some(grandparent) = parent.parent() {
                            let engine_path = grandparent.join("Engine");
                            if engine_path.exists() {
                                return Ok(grandparent.to_path_buf());
                            }
                        }
                    }
                } else {
                    // Try to find a standard engine installation
                    return self.find_standard_engine(version).await;
                }
            }
        }

        // Fallback: try to find any available engine
        self.find_any_available_engine().await
    }

    async fn find_standard_engine(&self, version: &str) -> Result<PathBuf> {
        // Common engine installation paths
        let possible_paths = if cfg!(target_os = "windows") {
            vec![
                format!("C:\\Program Files\\Epic Games\\UE_{}", version),
                format!("C:\\Program Files (x86)\\Epic Games\\UE_{}", version),
                format!("D:\\Epic Games\\UE_{}", version),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                format!("/Users/Shared/Epic Games/UE_{}", version),
                format!("/Applications/Epic Games/UE_{}", version),
            ]
        } else {
            vec![
                format!("/opt/UnrealEngine/UE_{}", version),
                format!("/usr/local/UnrealEngine/UE_{}", version),
                format!("~/UnrealEngine/UE_{}", version),
            ]
        };

        for path_str in possible_paths {
            let path = Path::new(&path_str);
            if path.exists() && path.join("Engine").exists() {
                return Ok(path.to_path_buf());
            }
        }

        Err(MessageError(format!(
            "Could not find Unreal Engine {} installation",
            version
        )))
    }

    async fn find_any_available_engine(&self) -> Result<PathBuf> {
        // Try to find any available engine installation
        let search_paths = if cfg!(target_os = "windows") {
            vec![
                "C:\\Program Files\\Epic Games",
                "C:\\Program Files (x86)\\Epic Games",
                "D:\\Epic Games",
            ]
        } else if cfg!(target_os = "macos") {
            vec!["/Users/Shared/Epic Games", "/Applications/Epic Games"]
        } else {
            vec!["/opt/UnrealEngine", "/usr/local/UnrealEngine"]
        };

        for search_path in search_paths {
            let path = Path::new(search_path);
            if path.exists() {
                if let Ok(entries) = fs::read_dir(path).await {
                    let mut entries = entries;
                    while let Some(entry) = entries.next_entry().await? {
                        let entry_path = entry.path();
                        if entry_path.is_dir() && entry_path.join("Engine").exists() {
                            return Ok(entry_path);
                        }
                    }
                }
            }
        }

        Err(MessageError(
            "Could not find any Unreal Engine installation".to_string(),
        ))
    }

    fn get_runuat_path(&self, engine_path: &Path) -> Result<PathBuf> {
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
        &self,
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
        command.args(&["-project", &request.project_path]);
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

    fn find_packaged_output(&self, output_dir: &Path, platform: &str) -> Result<PathBuf> {
        // The packaged output is typically in a subdirectory named after the platform
        let platform_dir = output_dir.join(platform);
        if platform_dir.exists() {
            return Ok(platform_dir);
        }

        // Fallback: return the output directory itself
        Ok(output_dir.to_path_buf())
    }

    async fn create_archive(&self, request: &PackageRequest) -> Result<CompressionResult> {
        if request.archive_format.is_none() {
            return Err(MessageError(
                "Archive format not specified for archiving".to_string(),
            ));
        }

        // Use the existing compression functionality
        let compression_request = CompressionRequest {
            project_path: request.project_path.clone(),
            destination_path: request.output_directory.clone(),
            compression_algorithm: request.archive_format.clone().unwrap(),
            clean_before_compress: false,
            cleaning_selection: None,
        };

        let result =
            project_compressor::compress_project(self.app_handle.clone(), compression_request)
                .await?;

        Ok(result)
    }
}

#[tauri::command]
pub async fn package_project(
    app_handle: AppHandle,
    request: PackageRequest,
) -> std::result::Result<(), String> {
    let packager = ProjectPackager::new(app_handle);

    // Run packaging in the background
    tokio::spawn(async move {
        if let Err(e) = packager.package_project(request).await {
            eprintln!("Packaging failed: {}", e);
        }
    });

    Ok(())
}
