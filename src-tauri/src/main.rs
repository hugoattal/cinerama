#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicUsize, Ordering};

use tauri::{
  AppHandle, Manager, RunEvent, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Window,
  menu::{Menu, MenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

static WINDOW_COUNT: AtomicUsize = AtomicUsize::new(0);

#[tauri::command]
fn toggle_fullscreen(window: Window) -> tauri::Result<()> {
  window.set_fullscreen(!window.is_fullscreen()?)
}

#[tauri::command]
fn exit_fullscreen(window: Window) -> tauri::Result<()> {
  window.set_fullscreen(false)
}

fn create_window(app: &AppHandle) -> tauri::Result<WebviewWindow> {
  let label = format!("window-{}", WINDOW_COUNT.fetch_add(1, Ordering::Relaxed));
  WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
    .title("Cinerama")
    .inner_size(260.0, 140.0)
    .build()
}

/// Brings an existing window to the front, or opens a new one if they were all closed.
fn show_window(app: &AppHandle) -> tauri::Result<()> {
  let window = match app.webview_windows().into_values().next() {
    Some(window) => window,
    None => create_window(app)?,
  };
  window.unminimize()?;
  window.show()?;
  window.set_focus()
}

fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
  let new_window = MenuItem::with_id(app, "new", "New window", true, None::<&str>)?;
  let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
  let menu = Menu::with_items(app, &[&new_window, &quit])?;

  let mut tray = TrayIconBuilder::new()
    .tooltip("Cinerama")
    .menu(&menu)
    .show_menu_on_left_click(false)
    .on_menu_event(|app, event| match event.id.as_ref() {
      "new" => {
        if let Err(error) = create_window(app) {
          eprintln!("Failed to create window: {error}");
        }
      }
      "quit" => app.exit(0),
      _ => {}
    })
    .on_tray_icon_event(|tray, event| {
      if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
      } = event
        && let Err(error) = show_window(tray.app_handle())
      {
        eprintln!("Failed to show window: {error}");
      }
    });

  if let Some(icon) = app.default_window_icon() {
    tray = tray.icon(icon.clone());
  }

  tray.build(app)?;
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .setup(|app| Ok(setup_tray(app.handle())?))
    .invoke_handler(tauri::generate_handler![toggle_fullscreen, exit_fullscreen])
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|_app, event| {
      // Keep running in the tray when the last window is closed, but let "Quit" (which sets an
      // exit code) go through.
      if let RunEvent::ExitRequested {
        code: None, api, ..
      } = event
      {
        api.prevent_exit();
      }
    });
}
