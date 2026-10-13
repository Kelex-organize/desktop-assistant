use tauri::AppHandle;

/// Label of the launcher window, as declared in `tauri.conf.json`.
pub const LAUNCHER_LABEL: &str = "launcher";

/// Shows the launcher if it is hidden, and hides it if it is visible.
/// When it is shown it must also get the keyboard focus.
// TODO(#8): implement, then remove the `allow`.
#[allow(unused_variables)]
pub fn toggle_launcher(app: &AppHandle) -> tauri::Result<()> {
    todo!()
}
