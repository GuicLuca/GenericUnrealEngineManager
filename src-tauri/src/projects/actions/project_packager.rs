use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::process::Command as AsyncCommand;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::misc::prelude::*;
use crate::misc::progress::TaskProgress;
use crate::settings::actions::settings_manager::SettingsManager;
use crate::projects::models::project::EngineAssociation;

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
    pub duration_ms: u64,
}

pub struct ProjectPackager {
    app_handle: AppHandle,
    settings_manager: SettingsManager,
}

impl ProjectPackager {
    pub fn new(app_handle: AppHandle) -> Self {
        let settings_manager = SettingsManager::new(app_handle.clone());
        Self {
            app_handle,
            settings_manager,
        }
    }

    /// Check if packaging is available for the given project
    pub async fn is_packaging_available(&self, project_path: &str) -> AppResult<bool> {
        // Load the project to get engine association
        let project = self.load_project_info(project_path).await?;
        
        // Try to find the engine path
        match self.find_engine_path(&project.engine_association).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get the reason why packaging is not available
    pub async fn get_packaging_unavailable_reason(&self, project_path: &str) -> AppResult<String> {
        let project = self.load_project_info(project_path).await?;
        
        match self.find_engine_path(&project.engine_association).await {
            Ok(_) => Ok("Packaging is available".to_string()),
            Err(e) => Ok(format!("Engine not found: {}", e)),
        }
    }

    /// Package a project with the given configuration
    pub async fn package_project(&self, request: PackageRequest) -> AppResult<PackageResult> {
        let task_id = format!("package_project_{}", chrono::Utc::now().timestamp_millis());
        let task_name = format!("Packaging {}", 
            Path::new(&request.project_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("project")
        );

        // Create and start the task
        let mut task_progress = TaskProgress::new(
            task_id.clone(),
            task_name.clone(),
            self.app_handle.clone(),
        );

        task_progress.start("Initializing packaging process...").await?;

        let start_time = std::time::Instant::now();
        
        // Load project info
        task_progress.update(0.1, "Loading project information...").await?;
        let project = self.load_project_info(&request.project_path).await
            .map_err(|e| {
                task_progress.fail(&format!("Failed to load project: {}", e));
                e
            })?;

        // Find engine path
        task_progress.update(0.2, "Locating Unreal Engine installation...").await?;
        let engine_path = self.find_engine_path(&project.engine_association).await
            .map_err(|e| {
                task_progress.fail(&format!("Engine not found: {}", e));
                e
            })?;

        // Validate output directory
        task_progress.update(0.3, "Validating output directory...").await?;
        self.ensure_output_directory(&request.output_directory).await
            .map_err(|e| {
                task_progress.fail(&format!("Output directory error: {}", e));
                e
            })?;

        // Build RunUAT command
        task_progress.update(0.4, "Preparing build command...").await?;
        let runuat_path = self.get_runuat_path(&engine_path)?;
        let mut command = self.build_runuat_command(&runuat_path, &request)?;

        // Execute packaging
        task_progress.update(0.5, "Starting Unreal Engine packaging...").await?;
        let output = self.execute_packaging_command(&mut command, &mut task_progress).await
            .map_err(|e| {
                task_progress.fail(&format!("Packaging failed: {}", e));
                e
            })?;

        let output_path = self.determine_output_path(&request)?;
        
        // Handle post-build compression if requested
        let compressed_path = if request.compress_after_build {
            task_progress.update(0.9, "Compressing packaged build...").await?;
            match self.compress_build(&output_path, &request).await {
                Ok(path) => Some(path),
                Err(e) => {
                    // Log compression error but don't fail the entire packaging
                    eprintln!("Compression failed: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let duration = start_time.elapsed();
        let result = PackageResult {
            success: output.status.success(),
            output_path,
            compressed_path,
            duration_ms: duration.as_millis() as u64,
        };

        if result.success {
            task_progress.complete(&format!(
                "Packaging completed successfully in {:.1}s", 
                duration.as_secs_f64()
            )).await?;
        } else {
            task_progress.fail("Packaging process failed").await?;
        }

        Ok(result)
    }

    /// Load basic project information
    async fn load_project_info(&self, project_path: &str) -> AppResult<ProjectInfo> {
        let path = Path::new(project_path);
        if !path.exists() {
            return Err(AppError::InvalidInput(format!("Project file not found: {}", project_path)));
        }

        // Read and parse the .uproject file
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| AppError::IoError(format!("Failed to read project file: {}", e)))?;
        
        let project_data: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| AppError::ParseError(format!("Invalid project file format: {}", e)))?;

        // Extract engine association
        let engine_association = if let Some(engine_assoc) = project_data.get("EngineAssociation") {
            if let Some(version) = engine_assoc.as_str() {
                if version == "Custom" {
                    EngineAssociation::Custom
                } else {
                    EngineAssociation::Standard(version.to_string())
                }
            } else {
                EngineAssociation::Custom
            }
        } else {
            return Err(AppError::InvalidInput("Project file missing EngineAssociation".to_string()));
        };

        Ok(ProjectInfo {
            engine_association,
        })
    }

    /// Find the engine path based on the engine association
    async fn find_engine_path(&self, engine_association: &EngineAssociation) -> AppResult<PathBuf> {
        let settings = self.settings_manager.get_settings().await?;
        
        match engine_association {
            EngineAssociation::Custom => {
                // For custom engines, try to find a matching registered engine
                if let Some(custom_engines) = settings.engine_programs.as_ref()
                    .and_then(|ep| ep.custom_engines.as_ref()) {
                    
                    if let Some((_, engine_path)) = custom_engines.iter().next() {
                        let path = PathBuf::from(engine_path);
                        if self.validate_engine_path(&path).await? {
                            return Ok(path);
                        }
                    }
                }
                Err(AppError::NotFound("No custom engine installations found in settings".to_string()))
            }
            EngineAssociation::Standard(version) => {
                // First, try to find in custom engines (user might have registered standard engines)
                if let Some(custom_engines) = settings.engine_programs.as_ref()
                    .and_then(|ep| ep.custom_engines.as_ref()) {
                    
                    for (name, engine_path) in custom_engines.iter() {
                        if name.contains(version) {
                            let path = PathBuf::from(engine_path);
                            if self.validate_engine_path(&path).await? {
                                return Ok(path);
                            }
                        }
                    }
                }
                
                // If not found in custom engines, try standard installation paths
                self.find_standard_engine_path(version).await
            }
        }
    }

    /// Validate that the engine path contains necessary files
    async fn validate_engine_path(&self, engine_path: &Path) -> AppResult<bool> {
        if !engine_path.exists() {
            return Ok(false);
        }

        // Check for RunUAT script
        let runuat_path = self.get_runuat_path(engine_path)?;
        Ok(runuat_path.exists())
    }

    /// Find standard engine installation path
    async fn find_standard_engine_path(&self, version: &str) -> AppResult<PathBuf> {
        let standard_paths = self.get_standard_engine_paths(version);
        
        for path in standard_paths {
            if self.validate_engine_path(&path).await? {
                return Ok(path);
            }
        }
        
        Err(AppError::NotFound(format!(
            "Unreal Engine {} not found in standard installation paths. Please add it to your engine settings.",
            version
        )))
    }

    /// Get standard engine installation paths for the current platform
    fn get_standard_engine_paths(&self, version: &str) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            // Epic Games Launcher default path
            paths.push(PathBuf::from(format!("C:\\Program Files\\Epic Games\\UE_{}", version)));
            // Alternative path
            paths.push(PathBuf::from(format!("C:\\Program Files (x86)\\Epic Games\\UE_{}", version)));
        }
        
        #[cfg(target_os = "macos")]
        {
            // macOS default paths
            paths.push(PathBuf::from(format!("/Users/Shared/Epic Games/UE_{}", version)));
            paths.push(PathBuf::from(format!("/Applications/Epic Games/UE_{}", version)));
        }
        
        #[cfg(target_os = "linux")]
        {
            // Linux default paths
            if let Ok(home) = std::env::var("HOME") {
                paths.push(PathBuf::from(format!("{}/UnrealEngine/{}", home, version)));
                paths.push(PathBuf::from(format!("{}/Epic Games/UE_{}", home, version)));
            }
            paths.push(PathBuf::from(format!("/opt/UnrealEngine/{}", version)));
        }
        
        paths
    }

    /// Get the RunUAT script path for the current platform
    fn get_runuat_path(&self, engine_path: &Path) -> AppResult<PathBuf> {
        #[cfg(target_os = "windows")]
        let runuat_path = engine_path.join("Engine/Build/BatchFiles/RunUAT.bat");
        
        #[cfg(not(target_os = "windows"))]
        let runuat_path = engine_path.join("Engine/Build/BatchFiles/RunUAT.sh");
        
        if !runuat_path.exists() {
            return Err(AppError::NotFound(format!(
                "RunUAT script not found at: {}",
                runuat_path.display()
            )));
        }
        
        Ok(runuat_path)
    }

    /// Build the RunUAT command with all necessary parameters
    fn build_runuat_command(&self, runuat_path: &Path, request: &PackageRequest) -> AppResult<AsyncCommand> {
        let mut command = AsyncCommand::new(runuat_path);
        
        // Base BuildCookRun command
        command.arg("BuildCookRun");
        
        // Project path
        command.arg("-project").arg(&request.project_path);
        
        // Target platform
        command.arg("-targetplatform").arg(&request.target_platform);
        
        // Build configuration
        command.arg("-configuration").arg(&request.build_type);
        
        // Output directory
        command.arg("-archivedirectory").arg(&request.output_directory);
        
        // Build options
        command.arg("-build");
        
        if request.cook_content {
            command.arg("-cook");
        }
        
        if request.pak_files {
            command.arg("-pak");
        }
        
        // Archive (package) the build
        command.arg("-archive");
        
        // Additional options
        command.arg("-stage");
        command.arg("-package");
        
        if !request.include_debug_files {
            command.arg("-nodebuginfo");
        }
        
        if request.create_release_version {
            command.arg("-createreleaseversion");
        }
        
        // Disable editor and client builds for packaging
        command.arg("-nocompileeditor");
        command.arg("-utf8output");
        
        Ok(command)
    }

    /// Execute the packaging command with progress updates
    async fn execute_packaging_command(
        &self,
        command: &mut AsyncCommand,
        task_progress: &mut TaskProgress,
    ) -> AppResult<std::process::Output> {
        use tokio::io::{AsyncBufReadExt, BufReader};
        use tokio::process::Stdio;
        
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        
        let mut child = command.spawn()
            .map_err(|e| AppError::ProcessError(format!("Failed to start packaging process: {}", e)))?;
        
        // Read stdout for progress updates
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            
            let mut progress = 0.5;
            while let Ok(Some(line)) = lines.next_line().await {
                // Update progress based on output patterns
                if line.contains("Cooking") {
                    progress = 0.6;
                    task_progress.update(progress, "Cooking content...").await?;
                } else if line.contains("Staging") {
                    progress = 0.7;
                    task_progress.update(progress, "Staging files...").await?;
                } else if line.contains("Packaging") {
                    progress = 0.8;
                    task_progress.update(progress, "Creating package...").await?;
                } else if line.contains("Success") || line.contains("completed successfully") {
                    progress = 0.85;
                    task_progress.update(progress, "Packaging completed successfully").await?;
                }
                
                // Log important lines
                if line.contains("Error") || line.contains("Warning") || line.contains("Success") {
                    println!("RunUAT: {}", line);
                }
            }
        }
        
        let output = child.wait_with_output().await
            .map_err(|e| AppError::ProcessError(format!("Failed to wait for packaging process: {}", e)))?;
        
        Ok(output)
    }

    /// Ensure the output directory exists
    async fn ensure_output_directory(&self, output_dir: &str) -> AppResult<()> {
        let path = Path::new(output_dir);
        if !path.exists() {
            tokio::fs::create_dir_all(path).await
                .map_err(|e| AppError::IoError(format!("Failed to create output directory: {}", e)))?;
        }
        Ok(())
    }

    /// Determine the actual output path after packaging
    fn determine_output_path(&self, request: &PackageRequest) -> AppResult<String> {
        // The actual output path depends on the platform and project name
        let project_name = Path::new(&request.project_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("UnknownProject");
        
        let platform_dir = match request.target_platform.as_str() {
            "Win64" => "Windows",
            "Mac" => "Mac",
            "Linux" => "Linux",
            "Android" => "Android",
            "iOS" => "IOS",
            _ => &request.target_platform,
        };
        
        let output_path = Path::new(&request.output_directory)
            .join(platform_dir)
            .join(project_name);
        
        Ok(output_path.to_string_lossy().to_string())
    }

    /// Compress the build if requested
    async fn compress_build(&self, output_path: &str, request: &PackageRequest) -> AppResult<String> {
        // This would integrate with your existing compression system
        // For now, return a placeholder
        let compressed_path = format!("{}.{}", output_path, 
            request.compression_algorithm.as_ref().unwrap_or(&"zip".to_string()).to_lowercase());
        
        // TODO: Integrate with existing compression functionality
        // You would call your existing compression functions here
        
        Ok(compressed_path)
    }
}

/// Basic project information needed for packaging
#[derive(Debug)]
struct ProjectInfo {
    engine_association: EngineAssociation,
}

/// Get available compression algorithms
pub async fn get_available_compression_algorithms() -> AppResult<Vec<String>> {
    // Return the same algorithms as your compression system
    Ok(vec![
        "Zip".to_string(),
        "SevenZip".to_string(),
        "Tar".to_string(),
        "TarGz".to_string(),
    ])
}

/// Get the current platform string
pub async fn get_current_platform() -> AppResult<String> {
    #[cfg(target_os = "windows")]
    return Ok("Windows".to_string());
    
    #[cfg(target_os = "macos")]
    return Ok("macOS".to_string());
    
    #[cfg(target_os = "linux")]
    return Ok("Linux".to_string());
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return Ok("Unknown".to_string());
}