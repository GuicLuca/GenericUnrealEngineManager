use tauri::App;
use tauri::menu::MenuBuilder;

pub fn init_window_menu(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {

    let builder = MenuBuilder::new(app);
    
    let menu = builder
        .text("projects", "Projects")
        .text("settings", "Settings")
        .text("about", "About")
        .build()?;
    
    app.set_menu(menu)?;
    
    Ok(())
}