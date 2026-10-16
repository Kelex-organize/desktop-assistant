use crate::core::intent::{CommandCall, Context, Intent};
use crate::core::interpreter::{InterpretError, Interpreter};
use serde_json::json;

/// A rule tries to understand a phrase: it returns the command to run, or `None`
/// when the phrase is not for it.
pub type Rule = fn(&str) -> Option<CommandCall>;

/// Interpreter based on fixed phrases. Works without AI and without internet.
pub struct RuleInterpreter {
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
    fn interpret(&self, text: &str, _context: &Context) -> Result<Intent, InterpretError> {
        if text.trim().is_empty() {
            return Err(InterpretError::EmptyInput);
        }

        self.rules
            .iter()
            .find_map(|rule| rule(text))
            .map(|call| Intent {
                commands: vec![call],
                ..Default::default()
            })
            .ok_or(InterpretError::NotUnderstood)
    }
}

const OPEN_VERBS: [&str; 6] = ["abrir", "abrí", "abre", "open", "launch", "start"];

const ARTICLES: [&str; 5] = ["el", "la", "los", "las", "the"];

/// "abrir X", "abrí X", "abre X", "open X", "launch X", "start X" -> open_app { name: X }.
fn open_app_with_verb(text: &str) -> Option<CommandCall> {
    let (verb, rest) = text.trim().split_once(char::is_whitespace)?;

    if !OPEN_VERBS.contains(&verb.to_lowercase().as_str()) {
        return None;
    }

    let name = strip_article(rest.trim());
    if name.is_empty() {
        return None;
    }

    Some(CommandCall {
        name: "open_app".to_string(),
        params: json!({ "name": name }),
    })
}

/// Removes a leading article ("la calculadora" -> "calculadora").
fn strip_article(text: &str) -> &str {
    match text.split_once(char::is_whitespace) {
        Some((first, rest)) if ARTICLES.contains(&first.to_lowercase().as_str()) => rest.trim(),
        _ => text,
    }
}

/// Catch-all: any other text is taken as the name of an app (M1 behavior).
/// To be removed when there is an index of installed apps.
fn open_app_plain(text: &str) -> Option<CommandCall> {
    let app = text.trim();

    if app.is_empty() {
        return None;
    }

    Some(CommandCall {
        name: "open_app".to_string(),
        params: json!({ "name": app }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The call every `open_app` rule is expected to produce.
    fn open_app_call(name: &str) -> CommandCall {
        CommandCall {
            name: "open_app".to_string(),
            params: json!({ "name": name }),
        }
    }

    fn interpret_text(text: &str) -> Result<Intent, InterpretError> {
        RuleInterpreter::new().interpret(text, &Context::default())
    }

    fn intent_with(call: CommandCall) -> Intent {
        Intent {
            commands: vec![call],
            reply: None,
        }
    }

    #[test]
    fn plain_text_is_taken_as_an_app_name() {
        assert_eq!(
            open_app_plain("calculator"),
            Some(open_app_call("calculator"))
        );
    }

    #[test]
    fn plain_text_is_trimmed() {
        assert_eq!(
            open_app_plain("  calculator   "),
            Some(open_app_call("calculator"))
        );
    }

    #[test]
    fn plain_rule_ignores_blank_text() {
        assert_eq!(open_app_plain(""), None);
        assert_eq!(open_app_plain("   "), None);
    }

    #[test]
    fn plain_text_keeps_inner_spaces() {
        assert_eq!(
            open_app_plain("bloc de notas"),
            Some(open_app_call("bloc de notas"))
        );
    }

    #[test]
    fn verb_opens_the_named_app() {
        assert_eq!(
            open_app_with_verb("open calculator"),
            Some(open_app_call("calculator"))
        );
    }

    #[test]
    fn every_verb_is_understood() {
        for verb in ["abrir", "abrí", "abre", "open", "launch", "start"] {
            let text = format!("{verb} calculator");

            assert_eq!(
                open_app_with_verb(&text),
                Some(open_app_call("calculator")),
                "verb: {verb}"
            );
        }
    }

    #[test]
    fn verb_ignores_case_and_extra_spaces() {
        assert_eq!(
            open_app_with_verb("opeN        calculator"),
            Some(open_app_call("calculator"))
        );
    }

    #[test]
    fn verb_alone_is_not_enough() {
        assert_eq!(open_app_with_verb("open"), None);
    }

    #[test]
    fn text_without_verb_is_not_for_this_rule() {
        assert_eq!(open_app_with_verb("calculator"), None);
    }

    #[test]
    fn verb_must_be_a_whole_word() {
        assert_eq!(open_app_with_verb("opened calculator"), None);
    }

    #[test]
    fn articles_are_dropped() {
        assert_eq!(
            open_app_with_verb("open the calculator"),
            Some(open_app_call("calculator"))
        );
    }

    #[test]
    fn empty_text_is_an_error() {
        assert_eq!(interpret_text(""), Err(InterpretError::EmptyInput));
        assert_eq!(interpret_text("   "), Err(InterpretError::EmptyInput));
    }

    #[test]
    fn a_phrase_becomes_an_intent_with_one_command() {
        assert_eq!(
            interpret_text("open calculator"),
            Ok(intent_with(open_app_call("calculator")))
        );
    }

    #[test]
    fn plain_text_still_opens_the_app() {
        assert_eq!(
            interpret_text("calculator"),
            Ok(intent_with(open_app_call("calculator")))
        );
    }

    #[test]
    fn the_verb_rule_wins_over_the_plain_one() {
        assert_eq!(
            interpret_text("open notepad"),
            Ok(intent_with(open_app_call("notepad")))
        );
    }
}
