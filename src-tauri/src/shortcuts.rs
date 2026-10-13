use tauri::AppHandle;
use tauri_plugin_global_shortcut::{
    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutEvent, ShortcutState,
};

use crate::windows::toggle_launcher;

/// The shortcut that shows or hides the launcher: Ctrl+Alt+K.
pub fn launcher_shortcut() -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyK)
}

/// Registers the global shortcuts.
pub fn register(app: &AppHandle) -> Result<(), tauri_plugin_global_shortcut::Error> {
    app.global_shortcut()
        .on_shortcut(launcher_shortcut(), |app, _shortcut, event| {
            on_launcher_shortcut(app, event);
        })
}

/// Reacts to an event of the launcher shortcut.
fn on_launcher_shortcut(app: &AppHandle, event: ShortcutEvent) {
    if event.state != ShortcutState::Pressed {
        return;
    }

    if let Err(error) = toggle_launcher(app) {
        eprintln!("could not toggle the launcher: {error}");
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn launcher_shortcut_is_ctrl_alt_k() {
        let expected: Shortcut = "ctrl+alt+k".parse().unwrap();

        assert_eq!(launcher_shortcut(), expected);
    }
}
