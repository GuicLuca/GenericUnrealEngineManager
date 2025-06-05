use std::path::Path;

/// # Project Discovery Actions
/// This module provides actions for discovering Unreal Engine projects
/// and their associated metadata, such as plugins and engine versions from
/// the user's file system. 


pub async fn scan_folder_for_projects(
    folder_path: &str,
    ignore_template: bool,
    ignore_engine: bool,
    ignore_samples: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut projects = Vec::new();

    // Use glob to find all .uproject files in the specified folder
    let pattern = format!("{}/**/*.uproject", folder_path);
    'entries: for entry in glob::glob(&pattern)? {
        match entry {
            Ok(path) => {
                if let Some(project_path) = path.to_str() {
                    //check if the path pass through a folder named "Templates"
                    // and if ignore_template is true, skip those projects
                    for folder in Path::new(project_path).components(){
                        if (ignore_template && folder.as_os_str() == "Templates")
                            || (ignore_engine && folder.as_os_str() == "Engine")
                            || (ignore_samples && folder.as_os_str() == "Samples") {
                            continue 'entries;
                        }
                    }
                    
                    projects.push(project_path.to_string());
                }
            }
            Err(e) => eprintln!("Error reading glob entry: {}", e),
        }
    }

    Ok(projects)
}