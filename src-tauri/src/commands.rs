use tauri::State;

use crate::core::intent::Context;
use crate::core::interpreter::{InterpretError, Interpreter};
use crate::core::registry::{CommandError, Registry};

/// Runs what the user typed and returns the text to show in the launcher.
///
/// `Ok` carries the confirmation, `Err` carries a message for the user.
/// Technical details are printed to the console, never shown in the launcher.
pub fn execute(
    registry: &Registry,
    interpreter: &dyn Interpreter,
    text: &str,
) -> Result<String, String> {
    let intent = interpreter
        .interpret(text, &Context::default())
        .map_err(|error| interpret_message(&error))?;

    let mut messages = Vec::new();
    for call in intent.commands {
        let output = registry.run(&call.name, call.params).map_err(|error| {
            eprintln!("command `{}` failed: {error}", call.name);
            user_message(&error)
        })?;
        messages.push(output.message);
    }

    Ok(messages.join(" · "))
}

fn interpret_message(error: &InterpretError) -> String {
    match error {
        InterpretError::EmptyInput => "Type what you want to do".to_string(),
        InterpretError::NotUnderstood => "I didn't understand that".to_string(),
    }
}

/// Turns a command error into a message the user can read.
fn user_message(error: &CommandError) -> String {
    match error {
        CommandError::NotFound(name) => format!("I don't know how to \"{name}\""),
        CommandError::InvalidParams(_) => "I didn't understand that".to_string(),
        CommandError::Duplicate(_) => "Something went wrong inside Kai".to_string(),
        CommandError::Failed(message) => message.clone(),
    }
}

/// Tauri command called from the launcher when the user presses Enter.
#[tauri::command]
pub fn run_text(
    text: String,
    registry: State<'_, Registry>,
    interpreter: State<'_, Box<dyn Interpreter>>,
) -> Result<String, String> {
    execute(&registry, interpreter.inner().as_ref(), &text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        intent::{CommandCall, Intent},
        interpreter::rules::RuleInterpreter,
        registry::{Command, CommandOutput},
    };
    use serde_json::json;

    struct FakeInterpreter(Result<Intent, InterpretError>);

    impl Interpreter for FakeInterpreter {
        fn interpret(&self, _text: &str, _context: &Context) -> Result<Intent, InterpretError> {
            self.0.clone()
        }
    }

    fn open_app_call(name: &str) -> CommandCall {
        CommandCall {
            name: "open_app".to_string(),
            params: json!({ "name": name }),
        }
    }

    /// A fake `open_app` that opens nothing: it just echoes the name it receives.
    fn registry_with_fake_open_app() -> Registry {
        let mut registry = Registry::new();
        registry
            .register(Command::new(
                "open_app",
                "Fake open_app for tests",
                json!({
                    "type": "object",
                    "properties": { "name": { "type": "string" } },
                    "required": ["name"]
                }),
                |params| {
                    let name = params["name"].as_str().unwrap_or_default();
                    Ok(CommandOutput {
                        message: format!("Running {name}"),
                    })
                },
            ))
            .unwrap();
        registry
    }

    #[test]
    fn executing_empty_text_asks_for_something() {
        let registry = registry_with_fake_open_app();

        let result = execute(&registry, &RuleInterpreter::new(), "   ");

        assert_eq!(result, Err("Type what you want to do".to_string()));
    }

    #[test]
    fn executing_a_known_command_returns_its_message() {
        let registry = registry_with_fake_open_app();

        let result = execute(&registry, &RuleInterpreter::new(), "  calc  ");

        assert_eq!(result, Ok("Running calc".to_string()));
    }

    #[test]
    fn executing_without_open_app_registered_returns_a_friendly_error() {
        let registry = Registry::new();

        let result = execute(&registry, &RuleInterpreter::new(), "calc");

        assert_eq!(result, Err("I don't know how to \"open_app\"".to_string()));
    }

    #[test]
    fn failed_errors_show_their_own_message() {
        let error = CommandError::Failed("Could not open the Calculator".to_string());

        assert_eq!(user_message(&error), "Could not open the Calculator");
    }

    #[test]
    fn other_errors_get_a_generic_message() {
        let invalid = CommandError::InvalidParams("missing name".to_string());
        let duplicate = CommandError::Duplicate("open_app".to_string());

        assert_eq!(user_message(&invalid), "I didn't understand that");
        assert_eq!(user_message(&duplicate), "Something went wrong inside Kai");
    }

    #[test]
    fn an_interpreter_that_understands_nothing_gives_a_friendly_error() {
        let registry = registry_with_fake_open_app();
        let interpreter = FakeInterpreter(Err(InterpretError::NotUnderstood));

        let result = execute(&registry, &interpreter, "whatever");

        assert_eq!(result, Err("I didn't understand that".to_string()));
    }

    #[test]
    fn several_commands_run_in_order_and_join_their_messages() {
        let registry = registry_with_fake_open_app();
        let interpreter = FakeInterpreter(Ok(Intent {
            commands: vec![open_app_call("one"), open_app_call("two")],
            reply: None,
        }));

        let result = execute(&registry, &interpreter, "whatever");

        assert_eq!(result, Ok("Running one · Running two".to_string()));
    }
}
