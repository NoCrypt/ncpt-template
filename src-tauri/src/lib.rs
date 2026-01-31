use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::WebviewWindow;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
// use tauri_plugin_positioner::{Position, WindowExt};
use tauri_plugin_prevent_default::PlatformOptions;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn move_window_to_workingarea_bottom_right(window: WebviewWindow) {
    window.primary_monitor().ok().flatten().map(|monitor| {
        let working_area = monitor.work_area();
        let window_size = window.outer_size().ok()?;
        let new_x = working_area.size.width.saturating_sub(window_size.width);
        let new_y = working_area.size.height.saturating_sub(window_size.height);
        window.set_position(tauri::PhysicalPosition { x: new_x as i32, y: new_y as i32 }).ok()
    });
}

pub fn setup_tray(app: AppHandle) {
    let quit_i = MenuItem::with_id(&app, "quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(&app, &[&quit_i]).unwrap();

    let was_focused = Arc::new(AtomicBool::new(false));
    let was_focused_clone = was_focused.clone();

    TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                // println!("quit menu item was clicked");
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(move |tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                // println!("left click pressed and released");
                // in this example, let's show and focus the main window when the tray is clicked
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                    match window.is_visible() {
                        Ok(is_visible) => {
                            if is_visible {
                                if was_focused.load(Ordering::SeqCst) {
                                    let _ = window.hide();
                                    was_focused.store(false, Ordering::SeqCst);
                                } else {
                                    let _ = window.set_focus();
                                    was_focused.store(true, Ordering::SeqCst);
                                }
                            } else {
                                // let _ = window.as_ref().window().move_window(Position::BottomRight);
                                move_window_to_workingarea_bottom_right(window.clone());
                                let _ = window.unminimize();
                                let _ = window.show();
                                let _ = window.set_focus();
                                was_focused.store(true, Ordering::SeqCst);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error checking if window is minimized: {}", e);
                        }
                    }
                }
            }
            TrayIconEvent::Enter { .. } => {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    if let Ok(focused) = window.is_focused() {
                        was_focused_clone.store(focused, Ordering::SeqCst);
                        // println!("Window focused: {}", focused);
                    }
                }
            }
            _ => {}
        })
        .build(&app)
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            setup_tray(app.app_handle().clone());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    use tauri_plugin_prevent_default::Flags;

    let mut builder = tauri_plugin_prevent_default::Builder::new().platform(PlatformOptions {
        general_autofill: false,
        password_autosave: false,
    });

    #[cfg(debug_assertions)]
    {
        builder = builder.with_flags(
            Flags::all().difference(Flags::CONTEXT_MENU | Flags::DEV_TOOLS | Flags::RELOAD),
        );
    }

    #[cfg(not(debug_assertions))]
    {
        builder = builder.with_flags(Flags::all().difference(Flags::CONTEXT_MENU));
    }

    builder.build()
}
