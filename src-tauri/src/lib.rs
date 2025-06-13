#![allow(unused_doc_comments)]

use std::process::exit;
use log::{error, info};
use tauri::{App, AppHandle, Builder, Emitter, Manager, RunEvent, Window, WindowEvent, Wry};
use tauri_plugin_fs::FsExt;
use tauri_plugin_log::Target;
use tauri_plugin_store::StoreExt;
use crate::misc::errors;

mod misc;
mod env;
mod projects;

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
    app.fs_scope()
        .allow_directory(app.handle().path().app_data_dir()?, true)?;

    // Set up the system tray
    match misc::tray::setup_system_tray(app) {
        Ok(_) => {
            info!("System tray initialized !");
        }
        Err(error) => {
            info!("Error setting up system tray: {}", error);
        }
    };
    
    match misc::menu::init_window_menu(app) {
        Ok(_) => {
            info!("Window menu initialized !");
        }
        Err(error) => {
            info!("Error setting up window menu: {}", error);
        }
    }
    
    tauri::async_runtime::spawn(async move {
        let result = projects::actions::project_discovery::scan_folder_for_projects(
            "D:/Dev", true, true, true
        ).await;
        match result {
            Ok(projects) => {
                info!("Found {} projects :", projects.len());
                for project in projects {
                    info!("-> {}", project);
                }
            }
            Err(e) => {
                error!("Error scanning folder for projects: {:?}", e);
            }
        }
    });
    

    // Fire the app initialized event
    match app.emit(env::EVENT_INIT, ()) {
        Ok(_) => {}
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