use lazy_static::lazy_static;

/// # **README : environment variable file** <br><br>
/// This file is used to store all the constants that are used in the application.
/// This is done to make it easier to change the values of these constants in the future.**
/// <br><br>
/// **Usage :**
/// - Always use the full type to declare a constant instead of importing the crate to ensure an easy and quick comprehension of the variable. <br>
/// *(e.g. `pub(crate) const MY_CONSTANT: tauri_plugin_log::TargetKind = ...;`)<br>*
/// - To access the constant in another file, use the full path to the constant (e.g. `env::LOG_MAX_SIZE`) to make it easy to locate the const declaration. <br>
/// - Prefer using regular constants instead of lazy_static to avoid unnecessary complexity. <br>
/// => lazy_static should only be used when the constant requires a complex initialization or when there is no const constructor for the type needed. <br>

///# ====================================
///# == Store related configuration
///# ====================================
/// Store documentation webpage : https://v2.tauri.app/plugin/store

/// The name of the local store file
pub(crate) const STORE_FILE_NAME: &str = "project_manager_store.json";

/// The key used to store the list of tracked project
pub(crate) const STORE_PROJECTS_KEY: &str = "projects";

/// The key used to store settings in the store
pub(crate) const STORE_SETTINGS_KEY: &str = "app_settings";

///# ====================================
///# == Logging configuration
///# ====================================
/// Logging documentation webpage : https://v2.tauri.app/plugin/logging

/// ## Log targets
/// The targets where the logs will be written to.<br>
/// - Stdout : Write logs to the console (in the development environment)<br>
/// - Webview : Write logs to the webview console<br>
/// - LogDir : Write logs to a file.<br>
/// - Folder : Write logs to a folder.<br>
/// For LogDir and Folder, the logs will be written to a file named `log.txt` in a specified directory. (name can be customized).
/// @see https://v2.tauri.app/plugin/logging/#persisting-logs for more information.

// Due to const/static type initialization issues, see values of these settings in lib.rs in run().

///## Log max size
/// The maximum size of the log file (in bytes) before it triggers a rotation.<br>
/// Use the following format to specify the size : (1024 * 1024) * **[size in MB]** <br>
/// Default is 10MB
pub(crate) const LOG_MAX_SIZE: u128 = (1024 * 1024) * 10; // 10MB

///## Log rotation strategy
/// - KeepOne : Keep only one log file -> Default
/// - KeepAll : Create a new log file each time the MAX_LOG_SIZE is reached and keep all the log files
pub(crate) const LOG_ROTATION_STRATEGY: tauri_plugin_log::RotationStrategy =
    tauri_plugin_log::RotationStrategy::KeepOne;

///## Log Level
/// The maximum level of the logs to display.<br>
/// Development profile default: Debug<br>
/// Release profile default: Warn
#[cfg(debug_assertions)]
pub(crate) const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;

#[cfg(not(debug_assertions))]
pub(crate) const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Warn;

/// ## Log colors
/// The colors of the log levels in the console.
/// You can customize the colors of the log levels in the console by overriding the default colors.
/// ```rust
/// tauri_plugin_log::fern::colors::ColoredLevelConfig::default()
///     .info(tauri_plugin_log::fern::colors::Color::Green) // Change the color of the info level to green
///     .debug(tauri_plugin_log::fern::colors::Color::Cyan) // Change the color of the debug level to cyan
///     .warn(tauri_plugin_log::fern::colors::Color::Yellow) // ...
///     .error(tauri_plugin_log::fern::colors::Color::Red);
/// ```
/// **Note:** This constant has no const constructor, so it must be initialized using the `lazy_static` crate.
/// To use it, you will need to dereference the constant to access the inner value. -> `*env::LOG_COLORS`
lazy_static! {
    pub(crate) static ref LOG_COLORS: tauri_plugin_log::fern::colors::ColoredLevelConfig =
        tauri_plugin_log::fern::colors::ColoredLevelConfig::default();
}

///## Log timezone
/// The timezone strategy to use when logging the time.<br>
/// - UseUtc: Use the UTC timezone
/// - UseLocal: Use the local machine timezone
pub(crate) const LOG_TIMEZONE: tauri_plugin_log::TimezoneStrategy =
    tauri_plugin_log::TimezoneStrategy::UseLocal;

///# ====================================
///# == Event configuration
///# ====================================
/// **Warning**: When updating the event names, you must also update the event names in the frontend !

/// The event name used to broadcast the initialization of the launcher on startup
pub(crate) const EVENT_INIT: &str = "app_initialized";

pub(crate) const EVENT_ADD_LOG: &str = "add_log";

pub(crate) const EVENT_PROJECTS_UPDATED: &str = "projects_updated";

/// Background task progress events
pub(crate) const EVENT_TASK_PROGRESS: &str = "task_progress";