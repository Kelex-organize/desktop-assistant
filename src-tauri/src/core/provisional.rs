use serde_json::json;

use crate::core::intent::CommandCall;

/// Turns what the user typed into a command to run.
///
/// PROVISIONAL (decision 022): until the rule interpreter exists (M2), any
/// non-empty text is taken as the name of an app to open. This function is the
/// only thing M2 has to replace.
pub fn interpret(text: &str) -> Option<CommandCall> {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return None;
    }

    Some(CommandCall {
        name: "open_app".to_string(),
        params: json!({ "name": trimmed }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_or_blank_text_is_not_a_command() {
        assert_eq!(interpret(""), None);
        assert_eq!(interpret("   \t "), None);
    }

    #[test]
    fn text_becomes_an_open_app_call_with_the_trimmed_name() {
        let expected = CommandCall {
            name: "open_app".to_string(),
            params: json!({ "name": "calculadora" }),
        };

        assert_eq!(interpret("  calculadora  "), Some(expected));
    }

    #[test]
    fn spaces_inside_the_text_are_kept() {
        let call = interpret("bloc de notas").unwrap();

        assert_eq!(call.params, json!({ "name": "bloc de notas" }));
    }
}
