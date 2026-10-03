use std::process::Command as Process;

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
            KnownApp::Calculator => "the Calculator",
            KnownApp::Notepad => "the Notepad",
            KnownApp::Explorer => "the Explorer",
        }
    }
}

/// Finds the app a user-typed name refers to, or `None` if it is unknown.
///
/// This is the single place where names are resolved. Custom apps (issue #21)
/// will plug in here later.
pub fn resolve(name: &str) -> Option<KnownApp> {
    let normalized = name
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    match normalized.as_str() {
        "calculadora" | "calc" | "calculator" => Some(KnownApp::Calculator),
        "bloc de notas" | "notepad" => Some(KnownApp::Notepad),
        "explorador" | "explorador de archivos" | "explorer" => Some(KnownApp::Explorer),
        _ => None,
    }
}

/// Launches an app. Never goes through a shell: the executable is started directly.
fn launch(app: KnownApp) -> Result<(), CommandError> {
    Process::new(app.executable())
        .spawn()
        .map_err(|e| CommandError::Failed(format!("could not start {}: {e}", app.executable())))?;
    Ok(())
}

/// Handler of the `open_app` command: reads `name`, resolves it and launches the app.
fn open_app(params: Value) -> Result<CommandOutput, CommandError> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| CommandError::InvalidParams("missing \"name\"".to_string()))?;
    let app =
        resolve(name).ok_or_else(|| CommandError::Failed(format!("not found app \"{name}\"")))?;
    launch(app)?;
    Ok(CommandOutput {
        message: format!("Running {}", app.display_name()),
    })
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

#[cfg(test)]
mod tests {

    use std::assert_eq;

    use super::*;

    #[test]
    fn resolves_every_known_name() {
        let cases = [
            ("calculadora", KnownApp::Calculator),
            ("calc", KnownApp::Calculator),
            ("calculator", KnownApp::Calculator),
            ("bloc de notas", KnownApp::Notepad),
            ("notepad", KnownApp::Notepad),
            ("explorador", KnownApp::Explorer),
            ("explorador de archivos", KnownApp::Explorer),
            ("explorer", KnownApp::Explorer),
        ];

        for (name, expected) in cases {
            assert_eq!(resolve(name), Some(expected), "name: {name}");
        }
    }

    #[test]
    fn resolve_ignores_case_and_extra_spaces() {
        assert_eq!(resolve("bloc  de  notas"), Some(KnownApp::Notepad));
        assert_eq!(resolve("CALCULADORA"), Some(KnownApp::Calculator));
    }

    #[test]
    fn resolve_returns_none_for_unknown_names() {
        assert_eq!(resolve("chrome"), None);
    }

    #[test]
    fn open_app_with_unknown_name_fails_without_panicking() {
        assert!(matches!(
            open_app(json!({"name": "foo"})),
            Err(CommandError::Failed(_))
        ));
    }

    #[test]
    fn open_app_without_name_is_rejected() {
        assert!(matches!(
            open_app(json!({})),
            Err(CommandError::InvalidParams(_))
        ));
    }

    #[test]
    fn open_app_is_registered() {
        let mut registry = Registry::new();
        register(&mut registry).unwrap();

        assert!(registry.get("open_app").is_some());
    }

    #[test]
    fn registry_rejects_an_empty_name() {
        let mut registry = Registry::new();
        register(&mut registry).unwrap();

        let result = registry.run("open_app", json!({"name": ""}));

        assert!(matches!(result, Err(CommandError::InvalidParams(_))));
    }

    #[test]
    fn executables_are_bare_file_names() {
        let apps = [KnownApp::Calculator, KnownApp::Explorer, KnownApp::Notepad];

        for app in apps {
            let exe = app.executable();
            assert!(
                !exe.chars().any(std::path::is_separator),
                "{exe} has a path separator"
            );
            assert!(!exe.contains(' '), "{exe} has a space");
        }
    }

    #[test]
    #[ignore = "opens a real window: run with 'cargo test -- --ignored'"]
    fn open_app_really_opens_the_calculator() {
        let result = open_app(json!({"name": "calc"}));

        assert_eq!(
            result,
            Ok(CommandOutput {
                message: "Running the Calculator".to_string()
            })
        )
    }
}
