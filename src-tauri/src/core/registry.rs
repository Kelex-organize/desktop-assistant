use std::collections::{hash_map::Entry, HashMap};

use serde_json::Value;
use thiserror::Error;

/// What a command returns when it succeeds: a short, user-facing confirmation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    pub message: String,
}

/// Everything that can go wrong when registering or running a command.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CommandError {
    #[error("unknown command: {0}")]
    NotFound(String),
    #[error("command already registered: {0}")]
    Duplicate(String),
    #[error("invalid parameters: {0}")]
    InvalidParams(String),
    #[error("command failed: {0}")]
    Failed(String),
}

/// The function that runs a command. It receives the (already validated)
/// parameters as JSON. `Send + Sync` because Tauri shares the registry between threads.
pub type Handler = Box<dyn Fn(Value) -> Result<CommandOutput, CommandError> + Send + Sync>;

/// A command the app knows how to run.
pub struct Command {
    pub name: String,
    pub description: String,
    /// JSON Schema describing the parameters. It is used to validate calls
    /// and, later, to describe the command to the AI.
    pub params_schema: Value,
    pub handler: Handler,
}

impl Command {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        params_schema: Value,
        handler: impl Fn(Value) -> Result<CommandOutput, CommandError> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            params_schema,
            handler: Box::new(handler),
        }
    }
}

/// Central list of commands, indexed by name.
#[derive(Default)]
pub struct Registry {
    commands: HashMap<String, Command>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a command. Fails with `CommandError::Duplicate` if the name is taken.
    pub fn register(&mut self, command: Command) -> Result<(), CommandError> {
        match self.commands.entry(command.name.clone()) {
            Entry::Occupied(_) => Err(CommandError::Duplicate(command.name)),
            Entry::Vacant(slot) => {
                slot.insert(command);
                Ok(())
            }
        }
    }

    /// Looks a command up by name.
    pub fn get(&self, name: &str) -> Option<&Command> {
        self.commands.get(name)
    }

    /// Validates `params` against the command's schema and runs it.
    pub fn run(&self, name: &str, params: Value) -> Result<CommandOutput, CommandError> {
        let command = self
            .get(name)
            .ok_or_else(|| CommandError::NotFound(name.to_string()))?;
        validate_params(&command.params_schema, &params)?;
        (command.handler)(params)
    }
}

/// Checks `params` against a JSON Schema.
fn validate_params(schema: &Value, params: &Value) -> Result<(), CommandError> {
    let validator = jsonschema::validator_for(schema)
        .map_err(|e| CommandError::Failed(format!("invalid schema: {e}")))?;
    validator
        .validate(params)
        .map_err(|e| CommandError::InvalidParams(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn greet_command() -> Command {
        Command::new(
            "greet",
            "Greets someone by name",
            json!({
                "type": "object",
                "properties": { "name": { "type": "string" } },
                "required": ["name"],
                "additionalProperties": false
            }),
            |params| {
                let name = params["name"].as_str().unwrap_or_default();
                Ok(CommandOutput {
                    message: format!("Hello, {name}!"),
                })
            },
        )
    }

    #[test]
    fn registered_command_can_be_found_by_name() {
        let mut registry = Registry::new();
        registry.register(greet_command()).unwrap();

        assert!(registry.get("greet").is_some());
    }

    #[test]
    fn registering_a_duplicate_name_fails() {
        let mut registry = Registry::new();
        registry.register(greet_command()).unwrap();
        assert_eq!(
            registry.register(greet_command()),
            Err(CommandError::Duplicate("greet".to_string()))
        );
    }

    #[test]
    fn getting_an_unknown_command_returns_none() {
        let registry = Registry::new();
        assert!(registry.get("none").is_none());
    }

    #[test]
    fn running_an_unknown_command_returns_not_found() {
        let registry = Registry::new();
        assert_eq!(
            registry.run("none", json!({})),
            Err(CommandError::NotFound("none".to_string()))
        );
    }

    #[test]
    fn running_with_valid_params_calls_the_handler() {
        let mut registry = Registry::new();
        registry.register(greet_command()).unwrap();
        let result = registry.run("greet", json!({"name": "Emanuel"})).unwrap();

        assert_eq!(result.message, "Hello, Emanuel!");
    }

    #[test]
    fn running_without_a_required_param_is_rejected() {
        let mut registry = Registry::new();
        registry.register(greet_command()).unwrap();

        assert!(matches!(
            registry.run("greet", json!({})),
            Err(CommandError::InvalidParams(_))
        ));
    }

    #[test]
    fn running_with_a_wrong_param_type_is_rejected() {
        let mut registry = Registry::new();
        registry.register(greet_command()).unwrap();

        assert!(matches!(
            registry.run("greet", json!({"name": 42})),
            Err(CommandError::InvalidParams(_))
        ));
    }

    #[test]
    fn a_broken_schema_returns_failed() {
        let mut registry = Registry::new();
        let broken = Command::new(
            "broken",
            "Has an invalid schema",
            json!({"type": "not-a-real-type"}),
            |_| {
                Ok(CommandOutput {
                    message: String::new(),
                })
            },
        );

        registry.register(broken).unwrap();
        let result = registry.run("broken", json!({}));
        assert!(matches!(result, Err(CommandError::Failed(_))))
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn registry_is_send_and_sync() {
        assert_send_sync::<Registry>();
    }
}
