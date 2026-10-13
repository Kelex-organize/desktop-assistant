use tauri::{AppHandle, Manager, Window, WindowEvent};

/// Label of the launcher window, as declared in `tauri.conf.json`.
pub const LAUNCHER_LABEL: &str = "launcher";

/// Shows the launcher if it is hidden, and hides it if it is visible.
/// When it is shown it must also get the keyboard focus.
pub fn toggle_launcher(app: &AppHandle) -> tauri::Result<()> {
    let window = app
        .get_webview_window(LAUNCHER_LABEL)
        .ok_or(tauri::Error::WindowNotFound)?;

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
// TODO(#7): implement, then remove the `allow`.
#[allow(unused_variables)]
pub fn show_launcher(app: &AppHandle) -> tauri::Result<()> {
    todo!()
}

/// Keeps the app alive when its main window is closed: the window is hidden
/// instead of destroyed, so the tray icon is the way to quit.
// TODO(#7): implement, then remove the `allow`. Right now it does nothing.
#[allow(unused_variables)]
pub fn keep_main_window_alive(window: &Window, event: &WindowEvent) {}
