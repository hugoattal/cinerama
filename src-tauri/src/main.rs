#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{
    CustomMenuItem,
    Event,
    Manager,
    SystemTray, SystemTrayEvent, SystemTrayMenu,
    WindowBuilder, WindowUrl
};

use std::time::{SystemTime, UNIX_EPOCH};

#[tauri::command]
async fn toggle_fullscreen(window: tauri::Window) {
    let is_fullscreen = window.is_fullscreen().unwrap();
    window.set_fullscreen(!is_fullscreen).unwrap();
}

fn main() {
    let app = tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("new", "New window"))
                .add_item(CustomMenuItem::new("exit_app", "Quit")),
        ))
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "exit_app" => {
                        app.exit(0);
                    }
                    "new" => {
                        app
                            .create_window(
                                format!("new-{}", SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis()
                                    .to_string()),
                                WindowUrl::App("index.html".into()),
                                |window_builder, webview_attributes| {
                                    (window_builder.title("Cinerama"), webview_attributes)
                                },
                            )
                            .unwrap();
                    },
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            toggle_fullscreen,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Regular);

    app.run(|_app_handle, e| match e {
        Event::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    })
}
