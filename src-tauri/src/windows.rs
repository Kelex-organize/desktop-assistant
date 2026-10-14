use std::eprintln;

use tauri::{AppHandle, Manager, WebviewWindow, Window, WindowEvent};

/// Label of the launcher window, as declared in `tauri.conf.json`.
pub const LAUNCHER_LABEL: &str = "launcher";

fn launcher_window(app: &AppHandle) -> tauri::Result<WebviewWindow> {
    app.get_webview_window(LAUNCHER_LABEL)
        .ok_or(tauri::Error::WindowNotFound)
}

/// Shows the launcher if it is hidden, and hides it if it is visible.
/// When it is shown it must also get the keyboard focus.
pub fn toggle_launcher(app: &AppHandle) -> tauri::Result<()> {
    let window = launcher_window(app)?;

    if window.is_visible()? {
        window.hide()?;
    } else {
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
}

/// Shows the launcher and gives it the keyboard focus. Unlike `toggle_launcher`,
/// it never hides it.
pub fn show_launcher(app: &AppHandle) -> tauri::Result<()> {
    let window = launcher_window(app)?;

    window.show()?;
    window.set_focus()?;

    Ok(())
}

/// Keeps the app alive when its main window is closed: the window is hidden
/// instead of destroyed, so the tray icon is the way to quit.
pub fn keep_main_window_alive(window: &Window, event: &WindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        if let Err(error) = window.hide() {
            eprintln!("could not hide the main window: {error}");
        }
    }
}
