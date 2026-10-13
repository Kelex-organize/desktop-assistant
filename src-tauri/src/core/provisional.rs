use serde_json::Value;

/// A command to run: its name in the registry and its parameters.
///
/// Provisional: M2 introduces the real `Intent`/`CommandCall` types.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandCall {
    pub name: String,
    pub params: Value,
}

/// Turns what the user typed into a command to run.
///
/// PROVISIONAL (decision 022): until the rule interpreter exists (M2), any
/// non-empty text is taken as the name of an app to open. This function is the
/// only thing M2 has to replace.
// TODO(#12): implement, then remove the `allow`.
#[allow(unused_variables)]
pub fn interpret(text: &str) -> Option<CommandCall> {
    todo!()
}
