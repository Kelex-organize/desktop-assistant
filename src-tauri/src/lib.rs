pub mod commands;
pub mod core;
pub mod modules;
pub mod shortcuts;
pub mod tray;
pub mod windows;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri::Manager;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let mut registry = core::registry::Registry::new();
            modules::apps::register(&mut registry)?;
            app.manage(registry);

            if let Err(error) = shortcuts::register(app.handle()) {
                eprintln!("could not register global shortcuts: {error}");
            }
            if let Err(error) = tray::setup(app.handle()) {
                eprintln!("could not create the tray icon: {error}");
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == "main" {
                windows::keep_main_window_alive(window, event);
            }
        })
        .invoke_handler(tauri::generate_handler![greet, commands::run_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_includes_the_name() {
        let result = greet("Emanuel");
        assert!(result.contains("Emanuel"), "unexpected greeting: {result}");
    }
}
