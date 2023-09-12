// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::utils::config::AppUrl;
use tauri::window::WindowBuilder;
use tauri::Manager;
use tauri::{generate_context, WindowUrl};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};


#[tauri::command]
  fn hide_me(_window: tauri::Window, app_handle: tauri::AppHandle) {
    let window = _window.get_window("main").unwrap();
    window.hide().unwrap();
}

#[tauri::command]
fn show_me(_window: tauri::Window, app_handle: tauri::AppHandle) {
  let window = _window.get_window("main").unwrap();
  window.show().unwrap();
}

#[tauri::command]
fn quit_me() {
  std::process::exit(0);
}

fn main() {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show);


    let tray = SystemTray::new().with_menu(tray_menu);


    let mut context = generate_context!();
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .setup(move |app| {
            WindowBuilder::new(
                app,
                "main".to_string(),
                if cfg!(dev) {
                    Default::default()
                } else {
                    window_url
                },
            )
            .title("Localhost Example")
            .build()?;
            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                },
                "show" => {
                  let window = app.get_window("main").unwrap();
                  window.show().unwrap();
              }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![show_me, hide_me, quit_me])
        .run(context)
        .expect("error while running tauri application");
}
