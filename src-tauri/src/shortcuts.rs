use tauri::AppHandle;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutEvent};

/// The shortcut that shows or hides the launcher: Ctrl+Alt+K.
pub fn launcher_shortcut() -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyK)
}

/// Registers the global shortcuts.
///
/// Another app may already own a shortcut: the error is returned (not unwrapped)
/// so the caller can report it without taking Kai down.
// TODO(#8): implement, then remove the `allow`. Right now it registers nothing.
#[allow(unused_variables)]
pub fn register(app: &AppHandle) -> Result<(), tauri_plugin_global_shortcut::Error> {
    Ok(())
}

/// Reacts to an event of the launcher shortcut.
///
/// A shortcut produces one event when the keys go down and another when they go up.
// TODO(#8): implement, then remove the `allow`.
#[allow(unused_variables, dead_code)]
fn on_launcher_shortcut(app: &AppHandle, event: ShortcutEvent) {
    todo!()
}
