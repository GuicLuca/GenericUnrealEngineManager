use crate::misc::prelude::*;
use crate::misc::progress::ProgressManager;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::AppHandle;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageRequest {
    pub project_path: String,
    pub build_type: String,
    pub target_platform: String,
    pub output_directory: String,
    pub for_distribution: bool,
    pub include_prerequisites: bool,
    pub include_app_local_prerequisites: bool,
    pub include_crash_reporter: bool,
    pub use_pak_file: bool,
    pub compress_content: bool,
    pub create_archive: bool,
    pub archive_format: Option<String>,
    pub archive_filename_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageResult {
    pub success: bool,
    pub output_path: String,
    pub archive_path: Option<String>,
    pub package_duration_ms: u64,
    pub total_duration_ms: u64,
}

pub struct ProjectPackager {
    app_handle: AppHandle,
    progress_manager: ProgressManager,
}

impl ProjectPackager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            progress_manager: ProgressManager::new(app_handle.clone()),
            app_handle,
        }
    }

    pub async fn package_project(&self, request: PackageRequest) -> Result<PackageResult> {
        let start_time = std::time::Instant::now();
        let project_name = Path::new(&request.project_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .replace(".uproject", "");

        let task_id = format!("package_project_{}", uuid::Uuid::new_v4());
        let task_name = format!("Packaging {} for {}", project_name, request.target_platform);

        // Start progress tracking
        self.progress_manager
            .start_task(&task_id, &task_name, "Initializing packaging process...")
            .await?;

        let result = self.package_project_internal(request.clone(), &task_id).await;

        match result {
            Ok(package_result) => {
                self.progress_manager
                    .complete_task(&task_id, "Packaging completed successfully")
                    .await?;
                Ok(package_result)
            }
            Err(e) => {
                self.progress_manager
                    .fail_task(&task_id, &format!("Packaging failed: {}", e))
                    .await?;
                Err(e)
            }
        }
    }

    async fn package_project_internal(
        &self,
        request: PackageRequest,
        task_id: &str,
    ) -> Result<PackageResult> {
        let start_time = std::time::Instant::now();

        // Validate project path
        let project_path = Path::new(&request.project_path);
        if !project_path.exists() {
            return Err(anyhow::anyhow!("Project file does not exist: {}", request.project_path));
        }

        // Find engine installation
        self.progress_manager
            .update_task(task_id, 0.1, "Finding Unreal Engine installation...")
            .await?;

        let engine_path = self.find_engine_for_project(project_path).await?;

        // Prepare output directory
        self.progress_manager
            .update_task(task_id, 0.2, "Preparing output directory...")
            .await?;

        let output_dir = Path::new(&request.output_directory);
        fs::create_dir_all(output_dir).await?;

        // Build RunUAT command
        self.progress_manager
            .update_task(task_id, 0.3, "Building packaging command...")
            .await?;

        let runuat_path = self.get_runuat_path(&engine_path)?;
        let mut command = self.build_package_command(&runuat_path, &request)?;

        // Execute packaging
        self.progress_manager
            .update_task(task_id, 0.4, "Starting Unreal Engine packaging...")
            .await?;

        let package_start = std::time::Instant::now();
        let output = command.output()?;
        let package_duration = package_start.elapsed();

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(anyhow::anyhow!(
                "Packaging failed. Stdout: {}\nStderr: {}",
                stdout,
                stderr
            ));
        }

        self.progress_manager
            .update_task(task_id, 0.8, "Packaging completed, finalizing...")
            .await?;

        // Determine the actual output path
        let output_path = self.find_packaged_output(&output_dir, &request.target_platform)?;

        let mut archive_path = None;

        // Create archive if requested
        if request.create_archive {
            self.progress_manager
                .update_task(task_id, 0.9, "Creating archive...")
                .await?;

            archive_path = Some(self.create_archive(&request, &output_path).await?);
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
                    // Try to find standard engine installation
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

        Err(anyhow::anyhow!("Could not find Unreal Engine {} installation", version))
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
            vec![
                "/Users/Shared/Epic Games",
                "/Applications/Epic Games",
            ]
        } else {
            vec![
                "/opt/UnrealEngine",
                "/usr/local/UnrealEngine",
            ]
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

        Err(anyhow::anyhow!("Could not find any Unreal Engine installation"))
    }

    fn get_runuat_path(&self, engine_path: &Path) -> Result<PathBuf> {
        let runuat_path = if cfg!(target_os = "windows") {
            engine_path.join("Engine").join("Build").join("BatchFiles").join("RunUAT.bat")
        } else {
            engine_path.join("Engine").join("Build").join("BatchFiles").join("RunUAT.sh")
        };

        if !runuat_path.exists() {
            return Err(anyhow::anyhow!("RunUAT script not found at: {}", runuat_path.display()));
        }

        Ok(runuat_path)
    }

    fn build_package_command(&self, runuat_path: &Path, request: &PackageRequest) -> Result<Command> {
        let mut command = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(&["/C", &runuat_path.to_string_lossy()]);
            cmd
        } else {
            let mut cmd = Command::new("bash");
            cmd.arg(&runuat_path.to_string_lossy());
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

        // Conditional flags
        if request.for_distribution {
            command.arg("-distribution");
        }

        if request.include_prerequisites {
            command.arg("-prereqs");
        }

        if request.include_app_local_prerequisites {
            command.arg("-applocaldirectory");
        }

        if request.include_crash_reporter {
            command.arg("-crashreporter");
        }

        if request.use_pak_file {
            command.arg("-pak");
        }

        if request.compress_content {
            command.arg("-compressed");
        }

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

    async fn create_archive(&self, request: &PackageRequest, output_path: &Path) -> Result<String> {
        // Use the existing compression functionality
        use crate::projects::actions::project_compressor::ProjectCompressor;

        let compressor = ProjectCompressor::new(self.app_handle.clone());
        
        // Generate archive filename
        let archive_name = self.generate_archive_filename(request)?;
        let archive_path = output_path.parent()
            .unwrap_or(output_path)
            .join(&archive_name);

        // Create compression request
        let compression_request = crate::projects::actions::project_compressor::CompressionRequest {
            source_path: output_path.to_string_lossy().to_string(),
            destination_path: archive_path.parent().unwrap().to_string_lossy().to_string(),
            compression_algorithm: request.archive_format.as_ref().unwrap().clone(),
            filename: Some(archive_name),
        };

        let result = compressor.compress_directory(compression_request).await?;
        Ok(result.output_path)
    }

    fn generate_archive_filename(&self, request: &PackageRequest) -> Result<String> {
        let format = request.archive_filename_format.as_ref()
            .unwrap_or(&"[Project]_[Platform]_[BuildType]_[YYYY][MM][DD][HH][mm]".to_string());

        let now = chrono::Local::now();
        let project_name = Path::new(&request.project_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .replace(".uproject", "");

        let mut filename = format.clone();
        
        // Replace placeholders
        filename = filename.replace("[Project]", &project_name);
        filename = filename.replace("[Platform]", &request.target_platform);
        filename = filename.replace("[BuildType]", &request.build_type);
        filename = filename.replace("[YYYY]", &now.format("%Y").to_string());
        filename = filename.replace("[MM]", &now.format("%m").to_string());
        filename = filename.replace("[DD]", &now.format("%d").to_string());
        filename = filename.replace("[HH]", &now.format("%H").to_string());
        filename = filename.replace("[mm]", &now.format("%M").to_string());
        filename = filename.replace("[ss]", &now.format("%S").to_string());

        // Add extension if not present
        let extension = match request.archive_format.as_ref().unwrap().as_str() {
            "Zip" => "zip",
            "SevenZip" => "7z",
            "Tar" => "tar",
            "TarGz" => "tar.gz",
            _ => "zip",
        };

        if !filename.contains('.') {
            filename.push('.');
            filename.push_str(extension);
        }

        Ok(filename)
    }
}