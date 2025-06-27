use crate::misc::errors;
use log::{error, info};
use tauri::{command, AppHandle, Manager};
use tauri_plugin_autostart::ManagerExt;

/// Enable autostart for the application
#[command]
pub async fn enable_autostart(app_handle: AppHandle) -> Result<(), String> {
    match app_handle.autolaunch().enable() {
        Ok(_) => {
            info!("Autostart enabled successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to enable autostart: {}", e);
            Err(format!("Failed to enable autostart: {}", e))
        }
    }
}

/// Disable autostart for the application
#[command]
pub async fn disable_autostart(app_handle: AppHandle) -> Result<(), String> {
    match app_handle.autolaunch().disable() {
        Ok(_) => {
            info!("Autostart disabled successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to disable autostart: {}", e);
            Err(format!("Failed to disable autostart: {}", e))
        }
    }
}

/// Check if autostart is currently enabled
#[command]
pub async fn is_autostart_enabled(app_handle: AppHandle) -> Result<bool, String> {
    match app_handle.autolaunch().is_enabled() {
        Ok(enabled) => Ok(enabled),
        Err(e) => {
            error!("Failed to check autostart status: {}", e);
            Err(format!("Failed to check autostart status: {}", e))
        }
    }
}