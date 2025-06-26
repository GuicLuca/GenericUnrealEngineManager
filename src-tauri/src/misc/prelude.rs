use log::error;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use crate::env;
use crate::misc::errors::ErrorLevel;

#[derive(Clone, Serialize, Deserialize)]
pub struct LogEntry<'a> {
    pub level: &'a str,
    pub message: &'a str,
}

pub fn log(app_handle: &AppHandle, level: ErrorLevel, message: &str) {
    match app_handle.emit(env::EVENT_ADD_LOG, LogEntry{
        message,
        level: level.as_str()
    }) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to emit log event: {}", e);
            error!("Intended log message: {} - {}", level.as_str(), message);
        }
    }
}

pub fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}