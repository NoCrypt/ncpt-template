use tauri_plugin_prevent_default::PlatformOptions;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    use tauri_plugin_prevent_default::Flags;

    let mut builder = tauri_plugin_prevent_default::Builder::new()
        .platform(PlatformOptions {
            general_autofill: false,
            password_autosave: false,
        });

    #[cfg(debug_assertions)]
    {
        builder = builder.with_flags(
            Flags::all().difference(Flags::CONTEXT_MENU | Flags::DEV_TOOLS | Flags::RELOAD)
        );
    }

    #[cfg(not(debug_assertions))]
    {
        builder = builder.with_flags(
            Flags::all().difference(Flags::CONTEXT_MENU)
        );
    }

    builder.build()
}
