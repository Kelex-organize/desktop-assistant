use serde_json::{json, Value};

use crate::core::registry::{Command, CommandError, CommandOutput, Registry};

/// Apps that ship with Windows and can be opened by name.
///
/// This is a closed list on purpose: user text is only ever *matched* against it,
/// never executed (decision 014).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnownApp {
    Calculator,
    Notepad,
    Explorer,
}

impl KnownApp {
    /// Executable that launches the app.
    pub fn executable(self) -> &'static str {
        match self {
            KnownApp::Calculator => "calc.exe",
            KnownApp::Notepad => "notepad.exe",
            KnownApp::Explorer => "explorer.exe",
        }
    }

    /// Name shown to the user in confirmations (Spanish, user-facing).
    pub fn display_name(self) -> &'static str {
        match self {
            KnownApp::Calculator => "la Calculadora",
            KnownApp::Notepad => "el Bloc de notas",
            KnownApp::Explorer => "el Explorador de archivos",
        }
    }
}

/// Finds the app a user-typed name refers to, or `None` if it is unknown.
///
/// This is the single place where names are resolved. Custom apps (issue #21)
/// will plug in here later.
// TODO(#11): implement, then remove the `allow`.
#[allow(unused_variables)]
pub fn resolve(name: &str) -> Option<KnownApp> {
    todo!()
}

/// Launches an app. Never goes through a shell: the executable is started directly.
// TODO(#11): implement, then remove the `allow`. You will need
// `use std::process::Command as Process;` (aliased: it clashes with our registry `Command`).
#[allow(unused_variables, dead_code)]
fn launch(app: KnownApp) -> Result<(), CommandError> {
    todo!()
}

/// Handler of the `open_app` command: reads `name`, resolves it and launches the app.
// TODO(#11): implement, then remove the `allow`.
#[allow(unused_variables)]
fn open_app(params: Value) -> Result<CommandOutput, CommandError> {
    todo!()
}

/// Registers this module's commands.
pub fn register(registry: &mut Registry) -> Result<(), CommandError> {
    registry.register(Command::new(
        "open_app",
        "Opens an app by name",
        json!({
            "type": "object",
            "properties": { "name": { "type": "string", "minLength": 1 } },
            "required": ["name"],
            "additionalProperties": false
        }),
        open_app,
    ))
}
