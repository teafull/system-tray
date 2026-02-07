use tauri::{
  menu::{Menu, MenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder},
  Manager,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().setup(|app| {
        let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
        let about_i = MenuItem::with_id(app, "about", "关于", true, None::<&str>)?;
        let menu = Menu::with_items(app, &[&about_i, &  quit_i])?;
        let _ = TrayIconBuilder::new()
            .icon(app.default_window_icon().unwrap().clone())
            .menu(&menu)
            .show_menu_on_left_click(false)
            .on_menu_event(move |_app, event| match event.id.as_ref() {

                "quit" => {
                    std::process::exit(0);
                }
                "about" => {
                    // TODO: Show about dialog
                }
                _ => {
                    println!("Unknown menu item:");
                }

            })
            .on_tray_icon_event(|tray, event| {
                if let tauri::tray::TrayIconEvent::Click {
                    id: _,
                    position: _,
                    rect: _,
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                } = event {
                    println!("left click pressed and released");
                    // let's show and focus the main window when the tray is clicked
                    if let Some(window) = tray.app_handle().get_webview_window("main") {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            })
            .build(app)?;
        Ok(())
    })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
