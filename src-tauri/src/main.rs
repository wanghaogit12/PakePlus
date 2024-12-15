// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde_json::Error;
use tauri::{utils::config::WindowConfig, Menu, MenuItem, Submenu, WindowBuilder};

fn json_to_window_config(window_json: &str) -> Result<WindowConfig, Error> {
    serde_json::from_str(window_json)
}

fn main() {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let menu = Menu::new();
    #[cfg(target_os = "macos")]
    let menu = Menu::new().add_submenu(Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll)
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit),
    ));
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let window_json = r#"{"label":"4","url":"https://blog.tsinbei.com/archives/1871/#14-%E5%88%87%E6%AF%94%E9%9B%AA%E5%A4%AB%E4%B8%8D%E7%AD%89%E5%BC%8F%E7%9A%84%E7%A7%AF%E5%88%86%E5%BD%A2%E5%BC%8F","userAgent":"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36","fileDropEnabled":true,"center":true,"width":800,"height":600,"minWidth":null,"minHeight":null,"maxWidth":null,"maxHeight":null,"resizable":true,"maximizable":true,"minimizable":true,"closable":true,"title":"4","fullscreen":false,"focus":false,"transparent":false,"maximized":false,"visible":true,"decorations":true,"alwaysOnTop":false,"contentProtected":false,"skipTaskbar":false,"titleBarStyle":"Visible","hiddenTitle":false,"acceptFirstMouse":false,"tabbingIdentifier":"","additionalBrowserArgs":""}"#;
            match json_to_window_config(window_json) {
                Ok(config) => {
                    println!("Parsed WindowConfig: {:?}", config);
                    let _main_window = WindowBuilder::from_config(&app_handle, config)
                        .build()
                        .unwrap();
                }
                Err(err) => {
                    eprintln!("Failed to parse JSON: {}", err);
                }
            }
            Ok(())
        })
        .menu(menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
