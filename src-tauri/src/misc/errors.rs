pub type Result<T> = std::result::Result<T, Verror>;
#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum Verror {
    // BASIC ERRORS
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    TauriError(#[from] tauri::Error),

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
