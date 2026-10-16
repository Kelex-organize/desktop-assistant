use serde_json::Value;

/// A command to run: its name in the registry and its parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandCall {
    pub name: String,
    pub params: Value,
}

/// What the interpreter understood from what the user said.
///
/// A greeting has no commands and only a `reply`; a request has commands and,
/// optionally, a friendly `reply` to show.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Intent {
    /// Zero, one or several commands to run, in order.
    pub commands: Vec<CommandCall>,
    /// A short answer for the user. Unused until greetings and the AI arrive.
    pub reply: Option<String>,
}

/// What the interpreter may know besides the text (previous requests, ...).
///
/// Empty on purpose until M7 (decision 008): it exists so that adding memory
/// later does not change the `Interpreter` signature.
#[derive(Debug, Clone, Default)]
pub struct Context {}
