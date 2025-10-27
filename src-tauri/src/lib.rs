use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{Manager, Window};

#[derive(Debug, Serialize, Deserialize)]
struct WindowState {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

fn get_state_path(app: &tauri::AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    fs::create_dir_all(&app_data_dir)?;
    Ok(app_data_dir.join("window_state.json"))
}

fn load_window_state(app: &tauri::AppHandle) -> Option<WindowState> {
    let state_path = get_state_path(app).ok()?;
    let contents = fs::read_to_string(state_path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn save_window_state(
    app: &tauri::AppHandle,
    state: &WindowState,
) -> Result<(), Box<dyn std::error::Error>> {
    let state_path = get_state_path(app)?;
    let json = serde_json::to_string_pretty(state)?;
    fs::write(state_path, json)?;
    Ok(())
}

#[tauri::command]
async fn save_window_position(window: Window) -> Result<(), String> {
    let position = window.outer_position().map_err(|e| e.to_string())?;
    let size = window.outer_size().map_err(|e| e.to_string())?;

    let state = WindowState {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
    };

    save_window_state(window.app_handle(), &state).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![save_window_position])
        .setup(|app| {
            let main_window = app
                .get_webview_window("main")
                .expect("Failed to get main window");

            // Load and apply saved window state
            if let Some(state) = load_window_state(app.handle()) {
                let _ =
                    main_window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                        x: state.x,
                        y: state.y,
                    }));
                let _ = main_window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                    width: state.width,
                    height: state.height,
                }));
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
