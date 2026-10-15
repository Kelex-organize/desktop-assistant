use crate::core::intent::{CommandCall, Context, Intent};
use crate::core::interpreter::{InterpretError, Interpreter};

/// A rule tries to understand a phrase: it returns the command to run, or `None`
/// when the phrase is not for it.
pub type Rule = fn(&str) -> Option<CommandCall>;

/// Interpreter based on fixed phrases. Works without AI and without internet.
pub struct RuleInterpreter {
    // TODO(#30): remove the `allow` once `interpret` reads this field.
    #[allow(dead_code)]
    rules: Vec<Rule>,
}

impl RuleInterpreter {
    /// The default rules. They are tried in this order, and the first one that
    /// understands the phrase wins: the specific ones go first, the catch-all last.
    pub fn new() -> Self {
        let rules: Vec<Rule> = vec![open_app_with_verb, open_app_plain];
        Self { rules }
    }
}

impl Default for RuleInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter for RuleInterpreter {
    /// Fails with `InterpretError::EmptyInput` for an empty (or blank) text;
    /// otherwise returns the `Intent` of the first rule that understands it.
    // TODO(#30): implement, then remove the `allow`.
    #[allow(unused_variables)]
    fn interpret(&self, text: &str, context: &Context) -> Result<Intent, InterpretError> {
        todo!()
    }
}

/// "abrir X", "abrí X", "abre X", "open X", "launch X", "start X" -> open_app { name: X }.
// TODO(#30): implement (test first!), then remove the `allow`.
#[allow(unused_variables)]
fn open_app_with_verb(text: &str) -> Option<CommandCall> {
    todo!()
}

/// Catch-all: any other text is taken as the name of an app (M1 behavior).
/// To be removed when there is an index of installed apps.
// TODO(#30): implement (test first!), then remove the `allow`.
#[allow(unused_variables)]
fn open_app_plain(text: &str) -> Option<CommandCall> {
    todo!()
}
