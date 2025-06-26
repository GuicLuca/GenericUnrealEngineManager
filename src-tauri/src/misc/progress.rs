use crate::env;
use crate::misc::payloads::{TaskProgressPayload, TaskStatus};
use log::error;
use tauri::{AppHandle, Emitter};

/// Helper struct to manage background task progress reporting
pub struct TaskProgress {
    app_handle: AppHandle,
    task_id: String,
    task_name: String,
}

impl TaskProgress {
    /// Create a new task progress tracker
    pub fn new(app_handle: AppHandle, task_id: String, task_name: String) -> Self {
        let progress = Self {
            app_handle,
            task_id,
            task_name,
        };
        
        // Emit task started event
        progress.emit_progress(0.0, TaskStatus::Started, None);
        progress
    }

    /// Update task progress
    pub fn update(&self, progress: f32, message: Option<String>) {
        self.emit_progress(progress.clamp(0.0, 1.0), TaskStatus::InProgress, message);
    }

    /// Mark task as completed
    pub fn complete(&self, message: Option<String>) {
        self.emit_progress(1.0, TaskStatus::Completed, message);
    }

    /// Mark task as failed
    pub fn fail(&self, message: Option<String>) {
        self.emit_progress(0.0, TaskStatus::Failed, message);
    }

    /// Emit progress event to frontend
    fn emit_progress(&self, progress: f32, status: TaskStatus, message: Option<String>) {
        let payload = TaskProgressPayload {
            task_id: self.task_id.clone(),
            task_name: self.task_name.clone(),
            progress,
            status,
            message,
        };

        if let Err(e) = self.app_handle.emit(env::EVENT_TASK_PROGRESS, payload) {
            error!("Failed to emit task progress event: {}", e);
        }
    }
}

impl Drop for TaskProgress {
    fn drop(&mut self) {
        // Ensure task is marked as completed when dropped
        if let Err(e) = self.app_handle.emit(env::EVENT_TASK_PROGRESS, TaskProgressPayload {
            task_id: self.task_id.clone(),
            task_name: self.task_name.clone(),
            progress: 1.0,
            status: TaskStatus::Completed,
            message: None,
        }) {
            error!("Failed to emit task completion event on drop: {}", e);
        }
    }
}