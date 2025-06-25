pub type Result<T> = std::result::Result<T, Verror>;

#[allow(dead_code)]
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
#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum Verror {
    // BASIC ERRORS
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    TauriError(#[from] tauri::Error),

    #[error(transparent)]
    TauriPluginStoreError(#[from] tauri_plugin_store::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    Error(#[from] Box<dyn std::error::Error>),

    #[error("{0}")]
    MessageError(String),

    // STORE ERRORS
    #[error("An error occurred while fetching the store at {0}")]
    StoreAccessError(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Verror {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
