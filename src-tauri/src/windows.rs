use tauri::{AppHandle, Manager};

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
