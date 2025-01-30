use tauri_plugin_prevent_default::Flags;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let prevent_default = tauri_plugin_prevent_default::Builder::new()
    .with_flags(Flags::all().difference(Flags::CONTEXT_MENU))
    .general_autofill(false)
    .password_autosave(false)
    .build();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
