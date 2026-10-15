use thiserror::Error;

use crate::core::intent::{Context, Intent};

pub mod rules;

/// Why a text could not be turned into an `Intent`.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum InterpretError {
    #[error("there is nothing to interpret")]
    EmptyInput,
}

/// Turns what the user typed or said into an `Intent`.
///
/// Implementations are interchangeable: rules now (`RuleInterpreter`), Claude
/// in M5 and Ollama in M6. `Send + Sync` because Tauri shares it between threads.
pub trait Interpreter: Send + Sync {
    fn interpret(&self, text: &str, context: &Context) -> Result<Intent, InterpretError>;
}
