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