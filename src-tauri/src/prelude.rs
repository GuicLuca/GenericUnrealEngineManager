use log::error;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use crate::projects::models::project::Project;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInitializedPayload {
    pub projects: Vec<Project>,
}




pub enum ErrorLevel {
    Info,
    Warning,
    Error,
    Debug,
}

impl ErrorLevel {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorLevel::Info => "info",
            ErrorLevel::Warning => "warning",
            ErrorLevel::Error => "error",
            ErrorLevel::Debug => "debug",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LogEntry<'a> {
    pub level: &'a str,
    pub message: &'a str,
}

pub fn log(app_handle: &AppHandle, level: ErrorLevel, message: &str) {
    match app_handle.emit("add-log", LogEntry{
        message,
        level: level.as_str()
    }) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to emit log event: {}", e);
            error!("Intended log message: {} - {}", level, message);
        }
    }
}