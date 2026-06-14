//! Filling a macro's `template:` metadata into rules text.
//! `~` -> the subject; `{i}` -> the rendered i-th positional arg.

use deckmaste_core::ExpansionArgs;

/// Fill a template. Returns `None` if any `{i}` can't be rendered cleanly
/// (caller then falls back to structural rendering — never emit a half-filled
/// template with a literal `{0}` left in it).
pub(super) fn fill(template: &str, subject: &str, args: &ExpansionArgs) -> Option<String> {
    let positional: &[String] = match args {
        ExpansionArgs::Positional(v) => v,
        ExpansionArgs::Named(_) => return None, // named-arg templates: not handled here
    };
    let mut out = String::new();
    let mut chars = template.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        match c {
            '~' => out.push_str(subject),
            '{' => {
                // parse the index up to '}'
                let mut idx = String::new();
                for (_, d) in chars.by_ref() {
                    if d == '}' {
                        break;
                    }
                    idx.push(d);
                }
                let i: usize = idx.trim().parse().ok()?;
                let raw = positional.get(i)?;
                out.push_str(&render_arg(raw)?);
            }
            other => out.push(other),
        }
    }
    Some(out)
}

/// Render one raw-RON-source positional arg. v1: bare integers
/// (`PumpThisUntilEot`'s magnitudes) pass through verbatim; anything else
/// returns `None` so the caller falls back to structural rendering.
/// (Grammar-node args — filters, costs — are a later enhancement.)
fn render_arg(raw: &str) -> Option<String> {
    let t = raw.trim();
    if t.parse::<i64>().is_ok() { Some(t.to_string()) } else { None }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::ExpansionArgs;

    use super::fill;

    #[test]
    fn fills_pump_template() {
        let args = ExpansionArgs::Positional(vec!["1".into(), "1".into()]);
        let s = fill("~ gets +{0}/+{1} until end of turn", "Goblin", &args);
        assert_eq!(s.as_deref(), Some("Goblin gets +1/+1 until end of turn"));
    }

    #[test]
    fn returns_none_for_unknown_arg() {
        let args = ExpansionArgs::Positional(vec!["SomeFilter".into()]);
        let s = fill("do {0} things", "it", &args);
        assert_eq!(s, None);
    }

    #[test]
    fn returns_none_for_named_args() {
        let args = ExpansionArgs::Named(vec![("x".into(), "1".into())]);
        let s = fill("~ gets +{0}", "it", &args);
        assert_eq!(s, None);
    }

    #[test]
    fn tilde_expands_to_subject() {
        let args = ExpansionArgs::Positional(vec![]);
        let s = fill("~ is here", "Goblin Guide", &args);
        assert_eq!(s.as_deref(), Some("Goblin Guide is here"));
    }

    #[test]
    fn missing_positional_returns_none() {
        let args = ExpansionArgs::Positional(vec!["1".into()]);
        // {1} doesn't exist
        let s = fill("~ gets +{0}/+{1}", "it", &args);
        assert_eq!(s, None);
    }
}
