// This file is not used anymore, as the menu is now handled by the frontend.

// use tauri::{App, AppHandle};
// use tauri::menu::{AboutMetadataBuilder, MenuBuilder, MenuEvent, PredefinedMenuItem, SubmenuBuilder};
// 
// pub fn init_window_menu(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
// 
//     let builder = MenuBuilder::new(app);
//     
//     let actions_menu = SubmenuBuilder::new(app, "Actions")
//         .text("actions", "Actions")
//         .build()?;
//     
//     let separator = PredefinedMenuItem::separator(app)?;
//     
//     let about_menu = PredefinedMenuItem::about(
//         app, 
//         None,
//         Some(
//             AboutMetadataBuilder::new()
//                 .name(Some(option_env!("CARGO_PKG_NAME").unwrap().replace("_", " ")))
//                 .version(option_env!("CARGO_PKG_VERSION"))
//                 .authors(Some(
//                     vec![
//                         "Lucas Guichard <lucasguichard127@gmail.com>".to_string()
//                     ]
//                 ))
//                 .license(option_env!("CARGO_PKG_LICENSE"))
//                 .build()
//         )
//     )?;
//     
//     let menu = builder
//         .items(&[&actions_menu,&separator, &about_menu])
//         .build()?;
//     
//     app.set_menu(menu)?;
// 
//     app.on_menu_event(move |app_handle: &AppHandle, event: MenuEvent| {
//         handle_menu_event(app_handle, event);
//     });
// 
//     Ok(())
// }
// 
// 
// fn handle_menu_event(_app: &AppHandle, event: MenuEvent) {
//     match event.id().0.as_str() {
//         "projects" => {
//             // Handle projects menu action
//             println!("Projects menu clicked");
//         }
//         "settings" => {
//             // Handle settings menu action
//             println!("Settings menu clicked");
//         }
//         "about" => {
//             // Handle about menu action
//             println!("About menu clicked");
//         }
//         _ => {
//             println!("Unknown menu item clicked: {}", event.id().0.as_str());
//         }
//     }
// }
