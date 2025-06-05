use tauri::command;
use std::process::Command;
use log::info;

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