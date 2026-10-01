use std::collections::HashMap;

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
    // TODO(#10): remove the `allow` once `register`/`get`/`run` use this field.
    #[allow(dead_code)]
    commands: HashMap<String, Command>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a command. Fails with `CommandError::Duplicate` if the name is taken.
    // TODO(#10): implement, then remove the `allow`.
    #[allow(unused_variables)]
    pub fn register(&mut self, command: Command) -> Result<(), CommandError> {
        todo!()
    }

    /// Looks a command up by name.
    // TODO(#10): implement, then remove the `allow`.
    #[allow(unused_variables)]
    pub fn get(&self, name: &str) -> Option<&Command> {
        todo!()
    }

    /// Validates `params` against the command's schema and runs it.
    /// Never panics: an unknown name or bad parameters come back as errors.
    // TODO(#10): implement, then remove the `allow`.
    #[allow(unused_variables)]
    pub fn run(&self, name: &str, params: Value) -> Result<CommandOutput, CommandError> {
        todo!()
    }
}

/// Checks `params` against a JSON Schema.
// TODO(#10): implement, then remove the `allow`.
#[allow(unused_variables, dead_code)]
fn validate_params(schema: &Value, params: &Value) -> Result<(), CommandError> {
    todo!()
}
