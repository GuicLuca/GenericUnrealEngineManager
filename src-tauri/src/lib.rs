#![allow(unused_doc_comments)]

use std::process::exit;
use std::sync::Arc;
use log::{error, info};
use serde_json::json;
use tauri::{App, AppHandle, Builder, Emitter, Manager, RunEvent, Window, WindowEvent, Wry};
use tauri_plugin_log::Target;
use tauri_plugin_store::{Store, StoreExt};
use crate::misc::errors;
use crate::misc::payloads::AppInitializedPayload;
use crate::projects::models::project::Project;
use crate::settings::actions::settings_manager;

mod misc;
mod env;
mod projects;
mod settings;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    ///Development profile (default: Stdout and Webview)
    #[cfg(debug_assertions)]
    let log_targets: [tauri_plugin_log::TargetKind; 2] = [
        tauri_plugin_log::TargetKind::Stdout,
        tauri_plugin_log::TargetKind::Webview,
    ];
    /// Release profile (default: LogDir)
    #[cfg(not(debug_assertions))]
    let log_targets: [tauri_plugin_log::TargetKind; 1] = [tauri_plugin_log::TargetKind::LogDir {
        file_name: Some("logs".to_string()),
    }];

    let tauri_builder: Builder<Wry> = tauri::Builder::default();

    ///### Plugins configuration
    /// For special configurations options, prefer using the env module
    let tauri_builder = tauri_builder
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // when a new instance is opened, show the main window, try to focus and show th main window
            if let Some(window) = app.get_webview_window("main") {
                if !window.is_visible().unwrap() {
                    let _ = window.show();
                }
                let _ = window.set_focus();
            }
        }))
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(
                    log_targets
                        .into_iter()
                        .map(Target::new)
                        .collect::<Vec<Target>>(),
                )
                .max_file_size(env::LOG_MAX_SIZE)
                .rotation_strategy(env::LOG_ROTATION_STRATEGY)
                .level(env::LOG_LEVEL)
                .level_for("reqwest", log::LevelFilter::Warn)
                .with_colors(*env::LOG_COLORS)
                .timezone_strategy(env::LOG_TIMEZONE)
                .build(),
        );

    ///### Application setup
    /// The setup function performe initialization tasks on startup such as:
    /// - Setting up the system tray
    /// - Fetching the remote games list from `ONLINE_CONFIGURATION_FILE` URL.
    /// - Saving the games list to the store
    let tauri_builder = tauri_builder.setup(|app| match application_setup(app) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.into()),
    });

    ///### Fronted events bindings
    /// Handle frontend events such as window close requests.<br>
    /// See the **`Application Runing`** section for backend events.
    let tauri_builder =
        tauri_builder.on_window_event(|window, event| {
            match frontend_event_handler(window, event) {
                Ok(_) => (),
                Err(e) => error!("Error handling window event: {:?}", e),
            }
        });

    ///### Tauri commands registration
    /// Register the commands that can be called from the frontend.
    /// - **Warning**: always specify the crate of the command to prevent conflicts. (The frontend will ignore the prefix and will only use the function name)<br>
    /// - **See** the commands module to see the list of available commands.<br>
    let tauri_builder = tauri_builder.invoke_handler(tauri::generate_handler![
        greet,
        projects::actions::behavior::open_file_explorer,
        projects::actions::project_discovery::discover_projects,
        projects::actions::project_discovery::get_projects,
        projects::actions::project_discovery::remove_projects,
        projects::actions::project_discovery::rescan_projects,
        projects::actions::project_launcher::launch_project_with_engine,
        projects::actions::project_launcher::launch_project_with_ide,
        projects::actions::project_launcher::launch_custom_engine_with_ide,
        projects::actions::project_launcher::project_has_cpp,
        projects::actions::project_cleaner::clean_project,
        projects::actions::project_compressor::compress_project,
        projects::actions::project_compressor::get_available_compression_algorithms,
        projects::actions::project_compressor::get_system_username,
        projects::actions::project_compressor::get_system_hostname,
        projects::actions::plugin_manager::scan_plugins,
        projects::actions::plugin_manager::refresh_all_plugins,
        settings::actions::settings_manager::get_settings,
        settings::actions::settings_manager::save_settings,
        settings::actions::autostart_manager::enable_autostart,
        settings::actions::autostart_manager::disable_autostart,
        settings::actions::autostart_manager::is_autostart_enabled,
    ]);

    ///### Application building
    /// see https://v2.tauri.app documentation for basics app behaviour.
    let tauri_builder = tauri_builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    ///### Application running
    /// Add bindings on backend events.
    tauri_builder.run(|app, event| {
        backend_event_handler(app, event);
    });
}


/// Handle backend events
fn backend_event_handler(app: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::ExitRequested { .. } => {
            // api.prevent_exit();
            quit_app(app);
        }
        _ => {
            // println!("Unhandled event: {:?}", event);
        }
    }
}

/// Handle frontend events
fn frontend_event_handler(window: &Window, event: &WindowEvent) -> errors::Result<()> {
    match event {
        WindowEvent::CloseRequested { .. } => {
            // window.hide()?;
            // misc::tray::update_tray_menu(window.app_handle())?;
            // api.prevent_close();
            quit_app(window.app_handle());
            

            Ok(())
        }
        _ => Ok(()),
    }
}

/// Perform the application setup
/// run on startup
fn application_setup(app: &mut App) -> errors::Result<()> {
    // Allow the app to access the app directory only
    // app.fs_scope()
    //     .allow_directory(app.handle().path().app_data_dir()?, true)?;

    // Set up the system tray
    match misc::tray::setup_system_tray(app) {
        Ok(_) => {
            info!("System tray initialized !");
        }
        Err(error) => {
            info!("Error setting up system tray: {}", error);
        }
    };
    
    // match misc::menu::init_window_menu(app) {
    //     Ok(_) => {
    //         info!("Window menu initialized !");
    //     }
    //     Err(error) => {
    //         info!("Error setting up window menu: {}", error);
    //     }
    // }

    /// ### Initialize the store
    /// In this step, ensure that all entries in the store are initialized
    /// to avoid dealing with null values in the future.
    /// - If the store fails to open, log the error and close the app.
    let store: Arc<Store<Wry>>;
    {
        info!("- Initializing store...");
        match app.store(env::STORE_FILE_NAME) {
            Ok(fetched_store) => {
                store = fetched_store;

                if store.get(env::STORE_PROJECTS_KEY).is_none() {
                    store.set(env::STORE_PROJECTS_KEY, json!([]));
                }
                info!("Store has been initialized.");
            }
            Err(e) => {
                eprintln!("The store failed to open. {}", e);
                quit_app(app.handle()); // END OF EXECUTION
            }
        };
    }
    // At this point the store variable is initialized and can be used.

    /// ### Initialize settings
    /// Initialize default settings if they don't exist
    match settings_manager::initialize_settings(app.handle()) {
        Ok(_) => {
            info!("Settings initialized successfully");
        }
        Err(e) => {
            error!("Failed to initialize settings: {}", e);
        }
    }

    // Check if we should show the welcome popup
    match settings_manager::should_show_welcome_popup(app.handle()) {
        Ok(should_show) => {
            if should_show {
                // Emit event to show welcome popup
                if let Err(e) = app.emit("show_welcome_popup", ()) {
                    error!("Failed to emit welcome popup event: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to check welcome popup setting: {}", e);
        }
    }

    // Fire the app initialized event
    match app.emit(env::EVENT_INIT, AppInitializedPayload{
        projects: Project::get_projects(app.handle())?
    }) {
        Ok(_) => {
            info!("App initialized event emitted successfully, sent {} projects.", 
                Project::get_projects(app.handle())?.len());
        }
        Err(e) => {
            error!("Error emitting app_initialized event: {:?}", e);
        }
    };
    
    
    Ok(())
}

/// Common function to quit the app this function is here
/// to execute some code before quitting the app.
pub fn quit_app(app: &AppHandle) {
    match app.store(env::STORE_FILE_NAME) {
        Ok(store) => {
            store.close_resource();
        }
        Err(e) => {
            eprintln!("Error clearing store: {:?}", e);
        }
    };

    info!("Quitting app...");
    app.cleanup_before_exit();
    exit(0);
}