use tauri::State;

use crate::core::registry::{CommandError, Registry};

/// Runs what the user typed and returns the text to show in the launcher.
///
/// `Ok` carries the confirmation, `Err` carries a message for the user (in Spanish).
/// Technical details are printed to the console, never shown in the launcher.
// TODO(#12): implement, then remove the `allow`. You will need
// `use crate::core::provisional;` (it has `interpret`).
#[allow(unused_variables)]
pub fn execute(registry: &Registry, text: &str) -> Result<String, String> {
    todo!()
}

/// Turns a command error into a message the user can read (Spanish).
// TODO(#12): implement, then remove the `allow`.
#[allow(unused_variables, dead_code)]
fn user_message(error: &CommandError) -> String {
    todo!()
}

/// Tauri command called from the launcher when the user presses Enter.
#[tauri::command]
pub fn run_text(text: String, registry: State<'_, Registry>) -> Result<String, String> {
    execute(&registry, &text)
}
