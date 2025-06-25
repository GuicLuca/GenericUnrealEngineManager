use crate::env;
use crate::misc::errors;
use crate::settings::models::settings::AppSettings;
use log::{error, info};
use serde_json::json;
use std::sync::Arc;
use tauri::{command, AppHandle};
use tauri_plugin_store::{Store, StoreExt, Wry};

/// The key used to store settings in the store
const STORE_SETTINGS_KEY: &str = "app_settings";

/// Get the current application settings
#[command]
pub fn get_settings(app_handle: AppHandle) -> Result<AppSettings, String> {
    match load_settings(&app_handle) {
        Ok(settings) => Ok(settings),
        Err(e) => {
            error!("Failed to load settings: {}", e);
            Err(format!("Failed to load settings: {}", e))
        }
    }
}

/// Save application settings
#[command]
pub fn save_settings(app_handle: AppHandle, settings: AppSettings) -> Result<(), String> {
    match store_settings(&app_handle, &settings) {
        Ok(_) => {
            info!("Settings saved successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to save settings: {}", e);
            Err(format!("Failed to save settings: {}", e))
        }
    }
}

/// Load settings from the store
pub fn load_settings(app_handle: &AppHandle) -> errors::Result<AppSettings> {
    let store: Arc<Store<Wry>> = app_handle.store(env::STORE_FILE_NAME)?;
    
    let settings = if let Some(settings_value) = store.get(STORE_SETTINGS_KEY) {
        serde_json::from_value::<AppSettings>(settings_value.clone())
            .unwrap_or_else(|e| {
                error!("Failed to parse settings from store: {}, using defaults", e);
                AppSettings::default()
            })
    } else {
        info!("No settings found in store, using defaults");
        AppSettings::default()
    };
    
    Ok(settings)
}

/// Store settings in the store
pub fn store_settings(app_handle: &AppHandle, settings: &AppSettings) -> errors::Result<()> {
    let store: Arc<Store<Wry>> = app_handle.store(env::STORE_FILE_NAME)?;
    
    let settings_json = serde_json::to_value(settings)?;
    store.set(STORE_SETTINGS_KEY, settings_json);
    store.save()?;
    
    Ok(())
}

/// Initialize settings in the store if they don't exist
pub fn initialize_settings(app_handle: &AppHandle) -> errors::Result<()> {
    let store: Arc<Store<Wry>> = app_handle.store(env::STORE_FILE_NAME)?;
    
    if store.get(STORE_SETTINGS_KEY).is_none() {
        let default_settings = AppSettings::default();
        let settings_json = serde_json::to_value(default_settings)?;
        store.set(STORE_SETTINGS_KEY, settings_json);
        store.save()?;
        info!("Initialized default settings");
    }
    
    Ok(())
}